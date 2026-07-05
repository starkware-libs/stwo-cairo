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

/// Controls the size of the lifting domain used by the commitment scheme (this size includes the
/// `log_blowup_factor`).
#[derive(Drop, Serde, Copy, PartialEq)]
pub enum LiftingLogSize {
    /// Lift all polynomials to the domain of exactly this log size.
    Fixed: u32,
    /// Lift each tree's polynomials to the largest domain within that tree - the natural sizing.
    /// An implicit assumption here is that the largest domains are all of equal size across trees,
    /// except possibly for the preprocessed tree.
    Auto,
    /// Lift to at least this log size, bumped up to the call site's natural minimum when the
    /// latter is larger, i.e. `max(x, natural_min)`. The natural minimum is the log size of the
    /// evaluation domain being committed, which already includes the `log_blowup_factor`. At the
    /// preprocessed-tree commitment that is `preprocessed_trace_log_size + log_blowup_factor`, so
    /// this resolves to `max(x, preprocessed_trace_log_size + log_blowup_factor)`.
    AtLeast: u32,
}

#[derive(Drop, Serde, Copy, PartialEq)]
pub struct PcsConfig {
    pub pow_bits: u32,
    pub fri_config: FriConfig,
    /// Controls the size of the lifting domain. See [`LiftingLogSize`].
    pub lifting_log_size: LiftingLogSize,
}
#[generate_trait]
pub impl PcsConfigImpl of PcsConfigTrait {
    fn mix_into(self: @PcsConfig, ref channel: Channel) {
        let PcsConfig { pow_bits, fri_config, lifting_log_size } = self;
        let FriConfig {
            log_blowup_factor, log_last_layer_degree_bound, n_queries, fold_step,
        } = fri_config;

        let zero = M31Zero::zero();
        // Domain-separated by a variant tag mixed before the value.
        let (lifting_tag, lifting_value) = match *lifting_log_size {
            LiftingLogSize::Fixed(log_size) => (0_u32, log_size),
            LiftingLogSize::Auto => (1_u32, 0_u32),
            LiftingLogSize::AtLeast(log_size) => (2_u32, log_size),
        };
        let lifting_tag_m31 = lifting_tag.try_into().unwrap();
        let lifting_value_m31 = lifting_value.try_into().unwrap();
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
                        [
                            (*fold_step).try_into().unwrap(), lifting_tag_m31, lifting_value_m31,
                            zero,
                        ],
                    ),
                ]
                    .span(),
            );
    }

    fn security_bits(self: @PcsConfig) -> u32 {
        let PcsConfig {
            pow_bits, fri_config: FriConfig {
                log_blowup_factor, log_last_layer_degree_bound: _, n_queries, fold_step: _,
            }, lifting_log_size: _,
        } = self;
        *pow_bits + *log_blowup_factor * *n_queries
    }
}
