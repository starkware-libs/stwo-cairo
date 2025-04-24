mod quotients;
pub mod verifier;

#[derive(Drop, Serde, Copy)]
pub struct PcsConfig {
    pub pow_bits: u32,
    pub fri_config: crate::fri::FriConfig,
}
