use core::box::BoxImpl;
use crate::channel::ChannelTrait;
use crate::channel::blake2s::Blake2sChannel;
use crate::fields::qm31::qm31_const;


#[test]
fn test_blake_bytes() {
    let mut channel: Blake2sChannel = Default::default();

    let result = channel.draw_random_bytes();

    // Tested against velue produced from Rust code.
    // https://github.com/starkware-libs/stwo/blob/dev/crates/prover/src/core/channel/blake2s.rs
    assert_eq!(
        result,
        array![
            156, 137, 99, 196, 121, 250, 173, 216, 202, 203, 240, 138, 54, 169, 61, 23, 148, 101,
            254, 213, 244, 221, 254, 46, 66, 230, 44, 151, 161, 215, 171, 242,
        ],
    );
}

#[test]
fn test_draw_secure_felt() {
    let mut channel: Blake2sChannel = Default::default();

    let felt = channel.draw_secure_felt();

    // Tested against values produced from Rust code.
    // https://github.com/starkware-libs/stwo/blob/dev/crates/prover/src/core/channel/blake2s.rs
    assert_eq!(felt, qm31_const::<1147373981, 1487796858, 183552971, 389916982>());
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
            qm31_const::<1147373981, 1487796858, 183552971, 389916982>(),
            qm31_const::<1442735509, 788454900, 388818499, 1923864482>(),
            qm31_const::<19235923, 1604041799, 1672672341, 594853474>(),
            qm31_const::<1723766014, 1300042830, 1903376519, 769478617>(),
            qm31_const::<1376899002, 1487153195, 1466950560, 174298319>(),
            qm31_const::<1807903912, 1308541718, 1538267614, 1849293059>(),
            qm31_const::<1505865733, 1283830469, 1224489433, 1359265092>(),
            qm31_const::<397197545, 498545251, 1437879563, 333198485>(),
        ],
    );
}
