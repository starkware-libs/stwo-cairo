use crate::fri::FriConfig;
use crate::pcs::*;

#[test]
fn test_security_bits() {
    let config = PcsConfig {
        pow_bits: 42,
        fri_config: FriConfig {
            log_blowup_factor: 10, log_last_layer_degree_bound: 1, n_queries: 70,
        },
    };
    assert_eq!(config.security_bits(), 10 * 70 + 42);
}
