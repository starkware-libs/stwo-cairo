use num_traits::Zero;
use stwo::core::channel::Channel;
use stwo::core::fields::qm31::SecureField;

use crate::air::PublicData;
use crate::claims::{CairoClaim, CairoInteractionClaim};
use crate::utils::pack_into_secure_felts;

pub struct FlatClaim {
    pub component_enable_bits: Vec<bool>,
    pub component_log_sizes: Vec<u32>,
    pub public_data: PublicData,
}
impl FlatClaim {
    pub fn from_cairo_claim(claim: &CairoClaim) -> Self {
        let (component_enable_bits, component_log_sizes) = flatten_claim(claim);
        Self {
            component_enable_bits,
            component_log_sizes,
            public_data: claim.public_data.clone(),
        }
    }

    pub fn mix_into(&self, channel: &mut impl Channel) {
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
        self.public_data.mix_into(channel);
    }
}

/// Converts enable bits to [u32], where each u32 is at most 2^31 - 1.
fn enable_bits_to_u32s(enable_bits: &[bool]) -> Vec<u32> {
    enable_bits.iter().map(|&b| if b { 1 } else { 0 }).collect()
}

/// Extracts component enable bits, and component log sizes from a [CairoClaim] and returns it as
/// vectors of [bool] and [u32] respectively.
/// The order must match the order of components as they appear in
/// [cairo_air::air::CairoComponents].
fn flatten_claim(claim: &CairoClaim) -> (Vec<bool>, Vec<u32>) {
    let mut component_enable_bits = vec![];
    let mut component_log_sizes = vec![];

    for c in claim.components {
        if let Some(log_size) = c {
            component_enable_bits.push(true);
            component_log_sizes.push(log_size);
        } else {
            component_enable_bits.push(false);
            component_log_sizes.push(0);
        }
    }

    (component_enable_bits, component_log_sizes)
}

/// Extracts the claimed sums from a [CairoInteractionClaim].
///
/// Returns a vector of all claimed sums for the logup argument, one per component.
/// The order must match the order of components as they appear in
/// [cairo_air::air::CairoComponents].
pub fn flatten_interaction_claim(interaction_claim: &CairoInteractionClaim) -> Vec<SecureField> {
    let mut claimed_sums = Vec::new();

    for ic in interaction_claim {
        if let Some(ic) = ic {
            claimed_sums.push(*ic);
        } else {
            claimed_sums.push(SecureField::zero());
        }
    }

    claimed_sums
}
