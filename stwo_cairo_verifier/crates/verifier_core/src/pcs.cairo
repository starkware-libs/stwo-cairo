//! Implements a FRI polynomial commitment scheme.
//!
//! This is a protocol where the prover can commit on a set of polynomials and then prove their
//! opening on a set of points.
//! Note: This implementation is not really a polynomial commitment scheme, because we are not in
//! the unique decoding regime. This is enough for a STARK proof though, where we only want to imply
//! the existence of such polynomials, and are ok with having a small decoding list.
//! Note: Opened points cannot come from the commitment domain.

use crate::channel::{Channel, ChannelTrait};
use crate::fields::m31::M31Zero;
use crate::fields::qm31::QM31Trait;
use crate::fri::FriConfig;

mod quotients;
pub mod verifier;
#[cfg(test)]
mod verifier_test;

#[derive(Drop, Serde, Copy, PartialEq)]
pub struct PcsConfig {
    pub pow_bits: u32,
    pub fri_config: FriConfig,
    /// A lower bound on the size of the lifting domain (this size includes the
    /// `log_blowup_factor`). Each tree is committed with height
    /// `max(min_lifting_log_size, max_column_log_size)`, where `max_column_log_size` is the log
    /// size of the largest (extended) domain within that tree.
    /// In particular, `0` lifts each tree's polynomials to the largest domain within that tree
    /// (an implicit assumption here is that the largest domains are all of equal size across
    /// trees, except possibly for the preprocessed tree).
    pub min_lifting_log_size: u32,
}
#[generate_trait]
pub impl PcsConfigImpl of PcsConfigTrait {
    fn mix_into(self: @PcsConfig, ref channel: Channel) {
        let PcsConfig { pow_bits, fri_config, min_lifting_log_size } = self;
        let FriConfig {
            log_blowup_factor, log_last_layer_degree_bound, n_queries, fold_step,
        } = fri_config;

        let zero = M31Zero::zero();
        let min_lifting_log_size_m31 = (*min_lifting_log_size).try_into().unwrap();
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
                    QM31Trait::from_fixed_array(
                        [(*fold_step).try_into().unwrap(), min_lifting_log_size_m31, zero, zero],
                    ),
                ]
                    .span(),
            );
    }

    fn security_bits(self: @PcsConfig) -> u32 {
        let PcsConfig {
            pow_bits, fri_config: FriConfig {
                log_blowup_factor, log_last_layer_degree_bound: _, n_queries, fold_step: _,
            }, min_lifting_log_size: _,
        } = self;
        *pow_bits + *log_blowup_factor * *n_queries
    }
}
