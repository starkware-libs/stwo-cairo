//! Implements a FRI polynomial commitment scheme.
//!
//! This is a protocol where the prover can commit on a set of polynomials and then prove their
//! opening on a set of points.
//! Note: This implementation is not really a polynomial commitment scheme, because we are not in
//! the unique decoding regime. This is enough for a STARK proof though, where we only want to imply
//! the existence of such polynomials, and are ok with having a small decoding list.
//! Note: Opened points cannot come from the commitment domain.

use stwo_verifier_core::utils::pack_into_qm31s;
use crate::channel::{Channel, ChannelTrait};
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
        channel
            .mix_felts(
                pack_into_qm31s(
                    array![
                        *self.pow_bits, *self.fri_config.log_blowup_factor,
                        *self.fri_config.n_queries, *self.fri_config.log_last_layer_degree_bound,
                    ]
                        .span(),
                ),
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
