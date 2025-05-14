#[cfg(test)]
mod tests {
    use core::box::BoxImpl;
    use crate::channel::ChannelTrait;
    use crate::channel::blake2s::{Blake2sChannel, new_channel};
    use crate::fields::qm31::qm31_const;
    use crate::vcs::blake2s_hasher::Blake2sHash;


    #[test]
    fn test_blake_bytes() {
        let mut channel: Blake2sChannel = Default::default();

        let result = channel.draw_random_bytes();

        // Tested against velue produced from Rust code.
        // https://github.com/starkware-libs/stwo/blob/dev/crates/prover/src/core/channel/blake2s.rs
        assert_eq!(
            result,
            array![
                174, 9, 219, 124, 213, 79, 66, 180, 144, 239, 9, 182, 188, 84, 26, 246, 136, 228,
                149, 155, 184, 197, 63, 53, 154, 111, 86, 227, 138, 180, 84, 163,
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
}
