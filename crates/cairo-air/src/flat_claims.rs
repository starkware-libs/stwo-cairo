use stwo::core::channel::{Channel, MerkleChannel};

use crate::air::PublicData;
use crate::utils::pack_into_secure_felts;

pub struct FlatClaim {
    pub component_enable_bits: Vec<bool>,
    pub component_log_sizes: Vec<u32>,
    pub public_data: PublicData,
}
impl FlatClaim {
    pub fn mix_into<MC: MerkleChannel>(&self, channel: &mut MC::C) {
        channel.mix_felts(&pack_into_secure_felts(
            [self.component_enable_bits.len() as u32].into_iter(),
        ));
        channel.mix_felts(&pack_into_secure_felts(
            enable_bits_to_u32s(&self.component_enable_bits).into_iter(),
        ));
        channel.mix_felts(&pack_into_secure_felts(
            self.component_log_sizes.iter().cloned(),
        ));
        channel.mix_felts(&pack_into_secure_felts(
            [self.public_data.public_memory.program.len() as u32].into_iter(),
        ));
        self.public_data.mix_into::<MC>(channel);
    }
}

/// Converts enable bits to [u32], where each u32 is at most 2^31 - 1.
fn enable_bits_to_u32s(enable_bits: &[bool]) -> Vec<u32> {
    enable_bits.iter().map(|&b| if b { 1 } else { 0 }).collect()
}
