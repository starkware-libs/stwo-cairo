use crate::channel::{Channel, ChannelTrait};
mod quotients;
pub mod verifier;
use crate::fri::FriConfigTrait;

#[derive(Drop, Serde, Copy)]
pub struct PcsConfig {
    pub pow_bits: u32,
    pub fri_config: crate::fri::FriConfig,
}
#[generate_trait]
pub impl PcsConfigImpl of PcsConfigTrait {
    fn mix_into(self: @PcsConfig, ref channel: Channel) {
        let PcsConfig { pow_bits, fri_config } = self;
        channel.mix_u64((*pow_bits).into());
        fri_config.mix_into(ref channel);
    }
    fn security_bits(self: @PcsConfig) -> u32 {
        let PcsConfig { pow_bits, fri_config } = self;
        *pow_bits + fri_config.security_bits()
    }
}

#[cfg(test)]
mod tests {
    use crate::fri::FriConfig;
    use super::*;

    #[test]
    fn test_security_bits() {
        let config = PcsConfig {
            pow_bits: 42,
            fri_config: FriConfig {
                log_blowup_factor: 10, log_last_layer_degree_bound: 10, n_queries: 70,
            },
        };
        assert_eq!(config.security_bits(), 10 * 70 + 42);
    }
}
