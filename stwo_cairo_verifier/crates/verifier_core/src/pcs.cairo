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
}
