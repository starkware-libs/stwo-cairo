//! Implements a FRI polynomial commitment scheme.
//!
//! This is a protocol where the prover can commit on a set of polynomials and then prove their
//! opening on a set of points.
//! Note: This implementation is not really a polynomial commitment scheme, because we are not in
//! the unique decoding regime. This is enough for a STARK proof though, where we only want to imply
//! the existence of such polynomials, and are ok with having a small decoding list.
//! Note: Opened points cannot come from the commitment domain.

use crate::channel::{Channel, ChannelTrait};
use crate::fields::m31::{M31One, M31Zero};
use crate::fields::qm31::QM31Trait;
use crate::fri::FriConfig;

mod quotients;
pub mod verifier;

#[derive(Drop, Serde, Copy)]
pub struct PcsConfig {
    pub pow_bits: u32,
    pub fri_config: FriConfig,
}
#[generate_trait]
pub impl PcsConfigImpl of PcsConfigTrait {
    fn mix_into(self: @PcsConfig, ref channel: Channel) {
        let PcsConfig { pow_bits, fri_config } = self;
        let FriConfig { log_blowup_factor, log_last_layer_degree_bound, n_queries } = fri_config;

        // Currently only line_fold_step == 1 is supported.
        let line_fold_step = M31One::one();

        let zero = M31Zero::zero();
        channel
            .mix_felts(
                array![
                    QM31Trait::from_fixed_array(
                        [
                            (*pow_bits).try_into().unwrap(),
                            (*log_blowup_factor).try_into().unwrap(),
                            (*n_queries).try_into().unwrap(),
                            (*log_last_layer_degree_bound).try_into().unwrap(),
                        ],
                    ),
                    QM31Trait::from_fixed_array([line_fold_step, zero, zero, zero]),
                ]
                    .span(),
            );
    }

    fn security_bits(self: @PcsConfig) -> u32 {
        let PcsConfig {
            pow_bits, fri_config: FriConfig {
                log_blowup_factor, log_last_layer_degree_bound: _, n_queries,
            },
        } = self;
        *pow_bits + *log_blowup_factor * *n_queries
    }
}
