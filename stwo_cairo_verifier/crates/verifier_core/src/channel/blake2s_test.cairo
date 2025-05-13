#[cfg(test)]
mod tests {
    use core::box::BoxImpl;
    use crate::fields::qm31::qm31_const;
    use crate::vcs::blake2s_hasher::Blake2sHash;
    use crate::channel::blake2s::{
        Blake2sChannel, ChannelTrait, check_proof_of_work, new_channel, get_digest
    };

    #[test]
    fn test_blake_bytes() {
        let mut channel: Blake2sChannel = Default::default();

        let result = channel.draw_random_bytes();

        // Tested against velue produced from Rust code.
        // https://github.com/starkware-libs/stwo/blob/dev/crates/prover/src/core/channel/blake2s.rs
        assert_eq!(
            result,
            array![
                174,
                9,
                219,
                124,
                213,
                79,
                66,
                180,
                144,
                239,
                9,
                182,
                188,
                84,
                26,
                246,
                136,
                228,
                149,
                155,
                184,
                197,
                63,
                53,
                154,
                111,
                86,
                227,
                138,
                180,
                84,
                163,
            ],
        );
    }

    #[test]
    fn test_draw_felt() {
        let mut channel: Blake2sChannel = Default::default();

        let felt = channel.draw_felt();

        // Tested against values produced from Rust code.
        // https://github.com/starkware-libs/stwo/blob/dev/crates/prover/src/core/channel/blake2s.rs
        assert_eq!(felt, qm31_const::<2094729646, 876761046, 906620817, 1981437117>());
    }

    #[test]
    fn test_draw_felts() {
        let mut channel: Blake2sChannel = Default::default();

        let felts = channel.draw_felts(8);

        // Tested against values produced from Rust code.
        // https://github.com/starkware-libs/stwo/blob/dev/crates/prover/src/core/channel/blake2s.rs
        assert_eq!(
            felts,
            array![
                qm31_const::<2094729646, 876761046, 906620817, 1981437117>(),
                qm31_const::<462808201, 893371832, 1666609051, 592753803>(),
                qm31_const::<2092874317, 1414799646, 202729759, 1138457893>(),
                qm31_const::<740261418, 1566411288, 1094134286, 1085813917>(),
                qm31_const::<1782652641, 591937235, 375882621, 687600507>(),
                qm31_const::<417708784, 676515713, 1053713500, 313648782>(),
                qm31_const::<1896458727, 242850046, 267152034, 827396985>(),
                qm31_const::<1959202869, 765813487, 1783334404, 305015811>(),
            ],
        );
    }

    #[test]
    fn test_mix_felts_with_1_felt() {
        let mut channel: Blake2sChannel = Default::default();

        channel.mix_felts([qm31_const::<1, 2, 3, 4>()].span());

        // Tested against values produced from Rust code.
        // https://github.com/starkware-libs/stwo/blob/dev/crates/prover/src/core/channel/blake2s.rs
        assert_eq!(
            get_digest(ref channel),
            [
                1586304710,
                1167332849,
                1688630032,
                429142330,
                4001363212,
                2013799503,
                180553907,
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
            get_digest(ref channel),
            [
                1835698174,
                2969628929,
                1758616107,
                158303712,
                3820231193,
                179192886,
                4063347398,
                3332297509,
            ],
        );
    }

    #[test]
    fn test_mix_felts_with_3_felts() {
        let mut channel: Blake2sChannel = Default::default();

        channel
            .mix_felts(
                [
                    qm31_const::<1, 2, 3, 4>(),
                    qm31_const::<5, 6, 7, 8>(),
                    qm31_const::<9, 10, 11, 12>(),
                ].span(),
            );

        // Tested against values produced from Rust code.
        // https://github.com/starkware-libs/stwo/blob/dev/crates/prover/src/core/channel/blake2s.rs
        assert_eq!(
            get_digest(ref channel),
            [
                2116479765,
                3227507660,
                1737697798,
                2518684651,
                1068812914,
                1858078313,
                1722202885,
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
                    qm31_const::<1, 2, 3, 4>(),
                    qm31_const::<5, 6, 7, 8>(),
                    qm31_const::<9, 10, 11, 12>(),
                    qm31_const::<13, 14, 15, 16>(),
                ].span(),
            );

        // Tested against values produced from Rust code.
        // https://github.com/starkware-libs/stwo/blob/dev/crates/prover/src/core/channel/blake2s.rs
        assert_eq!(
            get_digest(ref channel),
            [
                940149128,
                1354728945,
                2816315586,
                1690943110,
                210254904,
                3746481728,
                1339132640,
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
                    qm31_const::<1, 2, 3, 4>(),
                    qm31_const::<5, 6, 7, 8>(),
                    qm31_const::<9, 10, 11, 12>(),
                    qm31_const::<13, 14, 15, 16>(),
                    qm31_const::<17, 18, 19, 20>(),
                ].span(),
            );

        // Tested against values produced from Rust code.
        // https://github.com/starkware-libs/stwo/blob/dev/crates/prover/src/core/channel/blake2s.rs
        assert_eq!(
            get_digest(ref channel),
            [
                3425911356,
                1462327982,
                3241135902,
                4212900065,
                3145879221,
                3413011910,
                3946733048,
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
            get_digest(ref channel),
            [
                0xc13f9ebc,
                0x97884ed2,
                0x59336d95,
                0x24977332,
                0xcdca6b9d,
                0x74924d22,
                0x4abae704,
                0xce6edc77,
            ],
        );
    }

    #[test]
    pub fn test_mix_u32s() {
        let mut channel: Blake2sChannel = Default::default();

        channel.mix_u32s(array![1, 2, 3, 4, 5, 6, 7, 8, 9].span());

        // Tested against values produced from Rust code.
        // https://github.com/starkware-libs/stwo/blob/dev/crates/prover/src/core/channel/blake2s.rs
        assert_eq!(
            get_digest(ref channel),
            [
                0x83769170,
                0xb31bbb57,
                0xb6da6f34,
                0xfad757b3,
                0xe3fbb846,
                0x24432e2c,
                0x94c2ffa0,
                0xc7a1f9cb,
            ],
        );
    }

    #[test]
    fn test_check_proof_of_work() {
        let digest = Blake2sHash { hash: BoxImpl::new([0b1000, 0, 0, 0, 0, 0, 0, 0]) };

        let res = check_proof_of_work(digest, 3);

        assert!(res);
    }

    #[test]
    fn test_check_proof_of_work_with_invalid_n_bits() {
        let digest = Blake2sHash { hash: BoxImpl::new([0b1000, 0, 0, 0, 0, 0, 0, 0]) };

        let res = check_proof_of_work(digest, 4);

        assert!(!res);
    }
}
