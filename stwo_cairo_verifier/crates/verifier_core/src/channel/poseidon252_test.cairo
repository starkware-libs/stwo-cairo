#[cfg(test)]
mod tests {
    use crate::fields::qm31::qm31_const;
    use crate::channel::poseidon252::{
        ChannelTrait, Poseidon252Channel, Poseidon252ChannelImpl, check_proof_of_work, gen_bit_mask,
    };

    #[test]
    fn test_initialize_channel() {
        let channel: Poseidon252Channel = Default::default();

        // Assert that the channel is initialized correctly.
        assert_eq!(channel.digest, 0);
        assert_eq!(channel.channel_time.n_challenges, 0);
        assert_eq!(channel.channel_time.n_sent, 0);
    }

    #[test]
    fn test_channel_time() {
        let mut channel: Poseidon252Channel = Default::default();

        assert_eq!(channel.channel_time.n_challenges, 0);
        assert_eq!(channel.channel_time.n_sent, 0);

        channel.draw_felt();
        assert_eq!(channel.channel_time.n_challenges, 0);
        assert_eq!(channel.channel_time.n_sent, 1);

        channel.draw_felts(9);
        assert_eq!(channel.channel_time.n_challenges, 0);
        assert_eq!(channel.channel_time.n_sent, 6);

        channel.mix_root(0);
        assert_eq!(channel.channel_time.n_challenges, 1);
        assert_eq!(channel.channel_time.n_sent, 0);

        channel.draw_felt();
        assert_eq!(channel.channel_time.n_challenges, 1);
        assert_eq!(channel.channel_time.n_sent, 1);
        assert_ne!(channel.digest, 0);
    }


    #[test]
    pub fn test_draw_felt() {
        let initial_digest = 0;
        let mut channel = new_channel(initial_digest);

        let first_random_felt = channel.draw_felt();

        // Assert that next random felt is different.
        assert_ne!(first_random_felt, channel.draw_felt());
    }

    #[test]
    pub fn test_draw_felts() {
        let initial_digest = 0;
        let mut channel = new_channel(initial_digest);

        let mut random_felts = channel.draw_felts(5);
        random_felts.append_span(channel.draw_felts(4).span());

        // Assert that all the random felts are unique.
        assert_ne!(random_felts[0], random_felts[5]);
    }

    #[test]
    pub fn test_mix_root() {
        let initial_digest = 0;
        let mut channel = new_channel(initial_digest);

        for _ in 0_usize..10 {
            channel.draw_felt();
        }

        let prev_digest = channel.digest;
        channel.mix_root(0);
        assert_ne!(prev_digest, channel.digest);
    }

    #[test]
    pub fn test_mix_felts() {
        let initial_digest = 0;
        let mut channel = new_channel(initial_digest);

        channel
            .mix_felts(
                array![
                    qm31_const::<1, 2, 3, 4>(),
                    qm31_const::<5, 6, 7, 8>(),
                    qm31_const::<9, 10, 11, 12>(),
                ]
                    .span(),
            );

        assert_ne!(initial_digest, channel.digest);
    }

    #[test]
    pub fn test_mix_u32s() {
        let initial_digest = 0;
        let mut channel = new_channel(initial_digest);

        channel.mix_u32s(array![1, 2, 3, 4, 5, 6, 7, 8, 9].span());

        assert_eq!(
            channel.digest, 0x078f5cf6a2e7362b75fc1f94daeae7ebddd64e6b2db771717519af7193dfa80b,
        );
    }

    #[test]
    pub fn test_draw_random_bytes_1() {
        let initial_digest = 0;
        let mut channel = new_channel(initial_digest);
        let result = channel.draw_random_bytes();
        let expected_result = array![
            197,
            20,
            139,
            143,
            49,
            135,
            207,
            202,
            93,
            167,
            20,
            244,
            184,
            186,
            20,
            136,
            204,
            43,
            46,
            147,
            213,
            253,
            175,
            170,
            13,
            64,
            15,
            168,
            232,
            211,
            147,
        ];
        assert_eq!(expected_result, result);
    }

    #[test]
    pub fn test_draw_random_bytes_2() {
        let initial_digest = 0xdeadbeef;
        let mut channel = new_channel(initial_digest);
        let result = channel.draw_random_bytes();
        let expected_result = array![
            168,
            175,
            85,
            209,
            218,
            65,
            155,
            212,
            165,
            88,
            130,
            167,
            44,
            242,
            17,
            127,
            75,
            251,
            142,
            180,
            157,
            176,
            27,
            167,
            179,
            247,
            27,
            113,
            149,
            41,
            12,
        ];
        assert_eq!(expected_result, result);
    }

    #[test]
    pub fn test_draw_random_bytes_3() {
        let initial_digest = 0xcafecafe;
        let mut channel = new_channel(initial_digest);
        let first_result = channel.draw_random_bytes();
        let second_result = channel.draw_random_bytes();
        assert_ne!(first_result, second_result);
    }

    #[test]
    fn test_check_proof_of_work() {
        let digest = 0b1000;

        let res = check_proof_of_work(digest, 3);

        assert!(res);
    }

    #[test]
    fn test_check_proof_of_work_with_invalid_n_bits() {
        let digest = 0b1000;

        let res = check_proof_of_work(digest, 4);

        assert!(!res);
    }

    fn new_channel(digest: felt252) -> Poseidon252Channel {
        Poseidon252Channel { digest, channel_time: Default::default() }
    }
}
