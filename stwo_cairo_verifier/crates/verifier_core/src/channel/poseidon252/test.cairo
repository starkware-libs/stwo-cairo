use crate::channel::poseidon252::{
    ChannelTrait, Poseidon252Channel, Poseidon252ChannelImpl, check_leading_zeros,
};
use crate::fields::qm31::qm31_const;

#[test]
fn test_initialize_channel() {
    let channel: Poseidon252Channel = Default::default();

    // Assert that the channel is initialized correctly.
    assert_eq!(channel.digest, 0);
    assert_eq!(channel.n_draws, 0);
}

#[test]
fn test_channel_draws() {
    let mut channel: Poseidon252Channel = Default::default();

    assert_eq!(channel.n_draws, 0);

    channel.draw_secure_felt();
    assert_eq!(channel.n_draws, 1);

    channel.draw_secure_felts(9);
    assert_eq!(channel.n_draws, 6);

    channel.mix_commitment(0);
    assert_eq!(channel.n_draws, 0);

    channel.draw_secure_felt();
    assert_eq!(channel.n_draws, 1);
    assert_ne!(channel.digest, 0);
}


#[test]
pub fn test_draw_secure_felt() {
    let initial_digest = 0;
    let mut channel = new_channel(initial_digest);

    let first_random_felt = channel.draw_secure_felt();

    // Assert that next random felt is different.
    assert_ne!(first_random_felt, channel.draw_secure_felt());
}

#[test]
pub fn test_draw_secure_felts() {
    let initial_digest = 0;
    let mut channel = new_channel(initial_digest);

    let mut random_felts = channel.draw_secure_felts(5);
    random_felts.append_span(channel.draw_secure_felts(4).span());

    // Assert that all the random felts are unique.
    assert_ne!(random_felts[0], random_felts[5]);
}

#[test]
pub fn test_mix_commitment() {
    let initial_digest = 0;
    let mut channel = new_channel(initial_digest);

    for _ in 0_usize..10 {
        channel.draw_secure_felt();
    }

    let prev_digest = channel.digest;
    channel.mix_commitment(0);
    assert_ne!(prev_digest, channel.digest);
}

#[test]
pub fn test_mix_felts() {
    let initial_digest = 0;
    let mut channel = new_channel(initial_digest);

    channel
        .mix_felts(
            array![
                qm31_const::<1, 2, 3, 4>(), qm31_const::<5, 6, 7, 8>(),
                qm31_const::<9, 10, 11, 12>(),
            ]
                .span(),
        );

    assert_ne!(initial_digest, channel.digest);
}

#[test]
pub fn test_mix_u64() {
    let initial_digest = 0;
    let mut channel = new_channel(initial_digest);

    channel.mix_u64(0x1111222233334444);

    assert_eq!(channel.digest, 0x07cecc0ee3d858c843fe63165f038353f9f80f52dd8d32eead9f635e2f7d8b8e);
}

#[test]
pub fn test_draw_u32s_1() {
    let initial_digest = 0;
    let mut channel = new_channel(initial_digest);
    let result = channel.draw_u32s();
    // The expected result is computed from the Rust stwo code.
    let expected_result = array![
        886766026, 477679824, 3218540027, 1381728512, 24873733, 2344250857, 2336456258,
    ]
        .span();
    assert_eq!(expected_result, result);
}

#[test]
pub fn test_draw_u32s_2() {
    let initial_digest = 0xdeadbeef;
    let mut channel = new_channel(initial_digest);
    let result = channel.draw_u32s();
    // The expected result is computed from the Rust stwo code.
    let expected_result = array![
        2109664634, 1571772607, 2663772549, 3445588239, 2724310011, 1181437731, 926214439,
    ]
        .span();
    assert_eq!(expected_result, result);
}

#[test]
pub fn test_draw_u32s_3() {
    let initial_digest = 0xcafecafe;
    let mut channel = new_channel(initial_digest);
    let first_result = channel.draw_u32s();
    let second_result = channel.draw_u32s();
    assert_ne!(first_result, second_result);
}

#[test]
fn test_check_proof_of_work() {
    let digest = 0b1000;

    let res = check_leading_zeros(digest, 3);

    assert!(res);
}

#[test]
fn test_check_proof_of_work_with_invalid_n_bits() {
    let digest = 0b1000;

    let res = check_leading_zeros(digest, 4);

    assert!(!res);
}

fn new_channel(digest: felt252) -> Poseidon252Channel {
    Poseidon252Channel { digest, n_draws: Default::default() }
}
