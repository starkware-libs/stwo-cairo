mod quotients;
pub mod verifier;

#[derive(Drop, Copy)]
pub struct PcsConfig {
    pub pow_bits: u32,
    pub fri_config: crate::fri::FriConfig,
}

#[generate_trait]
pub impl PcsConfigSecurityBits of SecurityBitsTrait {
    fn security_bits(self: PcsConfig) -> u32 {
        self.pow_bits
            + self.fri_config.log_blowup_factor * self.fri_config.n_queries.try_into().unwrap()
    }
}
