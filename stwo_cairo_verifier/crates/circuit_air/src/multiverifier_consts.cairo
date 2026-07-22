//! Hardcoded constants for the multiverifier circuit.

use stwo_verifier_core::fri::FriConfig;
use stwo_verifier_core::pcs::PcsConfig;
use crate::per_component::PerComponent;

/// Expected PCS config of the multiverifier circuit's proof.
///
/// Hardcoded so the verifier accepts only proofs produced with the circuit's canonical
/// configuration. This pins every FRI security parameter (a weaker config — fewer queries,
/// smaller blowup, or less proof-of-work — is rejected, independently of stwo's
/// `security_bits >= SECURITY_BITS` floor). Must match the rust prover's
/// multiverifier `PCS_CONFIG` (`get_pcs_config(23, 1)`).
/// Note `pow_bits + log_blowup_factor * n_queries = 26 + 1 * 70 = 96 = SECURITY_BITS`.
pub fn circuit_pcs_config() -> PcsConfig {
    PcsConfig {
        pow_bits: 26,
        fri_config: FriConfig {
            log_blowup_factor: 1, log_last_layer_degree_bound: 0, n_queries: 70, fold_step: 4,
        },
    }
}

/// Number of public output values of the multiverifier circuit.
///
/// The multiverifier outputs the full unreduced Blake2s digest of its two verified inputs as
/// `N_RESERVED` = 8 QM31 words. (The logup anchor `u` is appended internally by the verifier and
/// is not part of the public outputs.)
pub const N_OUTPUTS: u32 = 8;

/// Each component's log size.
pub const COMPONENT_LOG_SIZES: PerComponent<u32> = PerComponent {
    verify_bitwise_xor_4: 8,
    verify_bitwise_xor_7: 14,
    verify_bitwise_xor_8: 16,
    range_check_16: 16,
    verify_bitwise_xor_9: 18,
    triple_xor: 19,
    eq: 20,
    m_31_to_u_32: 20,
    verify_bitwise_xor_12: 20,
    qm31_ops: 23,
    blake_g_gate: 23,
};

/// Per-column log sizes of the multiverifier circuit's preprocessed trace,
/// in size-sorted column order — the same order as the
/// index constants in `crate::preprocessed_columns`. Every column of a component shares
/// that component's log size, so each entry references the owning component's
/// `COMPONENT_LOG_SIZES` field.
pub const PREPROCESSED_COLUMN_LOG_SIZES: [u32; 45] = [
    COMPONENT_LOG_SIZES.verify_bitwise_xor_4, // bitwise_xor_4_0
    COMPONENT_LOG_SIZES.verify_bitwise_xor_4, // bitwise_xor_4_1
    COMPONENT_LOG_SIZES.verify_bitwise_xor_4, // bitwise_xor_4_2
    COMPONENT_LOG_SIZES.verify_bitwise_xor_7, // bitwise_xor_7_0
    COMPONENT_LOG_SIZES.verify_bitwise_xor_7, // bitwise_xor_7_1
    COMPONENT_LOG_SIZES.verify_bitwise_xor_7, // bitwise_xor_7_2
    COMPONENT_LOG_SIZES.range_check_16, // seq_16
    COMPONENT_LOG_SIZES.verify_bitwise_xor_8, // bitwise_xor_8_0
    COMPONENT_LOG_SIZES.verify_bitwise_xor_8, // bitwise_xor_8_1
    COMPONENT_LOG_SIZES.verify_bitwise_xor_8, // bitwise_xor_8_2
    COMPONENT_LOG_SIZES.verify_bitwise_xor_9, // bitwise_xor_9_0
    COMPONENT_LOG_SIZES.verify_bitwise_xor_9, // bitwise_xor_9_1
    COMPONENT_LOG_SIZES.verify_bitwise_xor_9, // bitwise_xor_9_2
    COMPONENT_LOG_SIZES.triple_xor, // triple_xor_input_addr_0
    COMPONENT_LOG_SIZES.triple_xor, // triple_xor_input_addr_1
    COMPONENT_LOG_SIZES.triple_xor, // triple_xor_input_addr_2
    COMPONENT_LOG_SIZES.triple_xor, // triple_xor_output_addr
    COMPONENT_LOG_SIZES.triple_xor, // triple_xor_multiplicity
    COMPONENT_LOG_SIZES.eq, // eq_in0_address
    COMPONENT_LOG_SIZES.eq, // eq_in1_address
    COMPONENT_LOG_SIZES.m_31_to_u_32, // m31_to_u32_input_addr
    COMPONENT_LOG_SIZES.m_31_to_u_32, // m31_to_u32_output_addr
    COMPONENT_LOG_SIZES.m_31_to_u_32, // m31_to_u32_multiplicity
    COMPONENT_LOG_SIZES.verify_bitwise_xor_12, // bitwise_xor_10_0
    COMPONENT_LOG_SIZES.verify_bitwise_xor_12, // bitwise_xor_10_1
    COMPONENT_LOG_SIZES.verify_bitwise_xor_12, // bitwise_xor_10_2
    COMPONENT_LOG_SIZES.qm31_ops, // qm31_ops_add_flag
    COMPONENT_LOG_SIZES.qm31_ops, // qm31_ops_sub_flag
    COMPONENT_LOG_SIZES.qm31_ops, // qm31_ops_mul_flag
    COMPONENT_LOG_SIZES.qm31_ops, // qm31_ops_pointwise_mul_flag
    COMPONENT_LOG_SIZES.qm31_ops, // qm31_ops_in0_address
    COMPONENT_LOG_SIZES.qm31_ops, // qm31_ops_in1_address
    COMPONENT_LOG_SIZES.qm31_ops, // qm31_ops_out_address
    COMPONENT_LOG_SIZES.qm31_ops, // qm31_ops_mults
    COMPONENT_LOG_SIZES.blake_g_gate, // blake_g_gate_input_addr_a
    COMPONENT_LOG_SIZES.blake_g_gate, // blake_g_gate_input_addr_b
    COMPONENT_LOG_SIZES.blake_g_gate, // blake_g_gate_input_addr_c
    COMPONENT_LOG_SIZES.blake_g_gate, // blake_g_gate_input_addr_d
    COMPONENT_LOG_SIZES.blake_g_gate, // blake_g_gate_input_addr_f0
    COMPONENT_LOG_SIZES.blake_g_gate, // blake_g_gate_input_addr_f1
    COMPONENT_LOG_SIZES.blake_g_gate, // blake_g_gate_output_addr_a
    COMPONENT_LOG_SIZES.blake_g_gate, // blake_g_gate_output_addr_b
    COMPONENT_LOG_SIZES.blake_g_gate, // blake_g_gate_output_addr_c
    COMPONENT_LOG_SIZES.blake_g_gate, // blake_g_gate_output_addr_d
    COMPONENT_LOG_SIZES.blake_g_gate // blake_g_gate_multiplicity
];

#[cfg(test)]
mod tests {
    use stwo_verifier_utils::zip_eq::zip_eq;
    use crate::components;
    use crate::per_component::PerComponentTrait;
    use crate::preprocessed_columns::{
        BLAKE_G_GATE_INPUT_ADDR_A_IDX, EQ_IN0_ADDRESS_IDX, M_31_TO_U_32_INPUT_ADDR_IDX,
        OP_0_ADDR_IDX, TRIPLE_XOR_INPUT_ADDR_0_IDX,
    };
    use super::{COMPONENT_LOG_SIZES, PREPROCESSED_COLUMN_LOG_SIZES};

    /// Derives each component's log size from the preprocessed column log sizes, in the committed
    /// (size-sorted) order of `COMPONENT_LOG_SIZES`. Variable-size components read the log size of
    /// one of their preprocessed columns (all columns of a component share its log size);
    /// fixed-size components return their `LOG_SIZE` constant.
    fn derive_component_log_sizes(preprocessed_column_log_sizes: Span<u32>) -> Array<u32> {
        array![
            components::verify_bitwise_xor_4::LOG_SIZE, // verify_bitwise_xor_4
            components::verify_bitwise_xor_7::LOG_SIZE, // verify_bitwise_xor_7
            components::verify_bitwise_xor_8::LOG_SIZE, // verify_bitwise_xor_8
            components::range_check_16::LOG_SIZE, // range_check_16
            components::verify_bitwise_xor_9::LOG_SIZE, // verify_bitwise_xor_9
            *preprocessed_column_log_sizes.at(TRIPLE_XOR_INPUT_ADDR_0_IDX), // triple_xor
            *preprocessed_column_log_sizes.at(EQ_IN0_ADDRESS_IDX), // eq
            *preprocessed_column_log_sizes.at(M_31_TO_U_32_INPUT_ADDR_IDX), // m_31_to_u_32
            components::verify_bitwise_xor_12::LOG_SIZE, // verify_bitwise_xor_12
            *preprocessed_column_log_sizes.at(OP_0_ADDR_IDX), // qm31_ops
            *preprocessed_column_log_sizes.at(BLAKE_G_GATE_INPUT_ADDR_A_IDX) // blake_g_gate
        ]
    }

    /// Returns whether `values` is in non-decreasing order.
    fn is_sorted(values: Span<u32>) -> bool {
        let mut values_iter = values.into_iter();
        let Some(mut prev) = values_iter.next() else {
            return true;
        };

        for value in values_iter {
            if *prev > *value {
                return false;
            }
            prev = value;
        }
        true
    }

    /// The hardcoded `COMPONENT_LOG_SIZES` must equal the values derived from the preprocessed
    /// column log sizes.
    #[test]
    fn hardcoded_component_log_sizes_match_derived() {
        let derived = derive_component_log_sizes(PREPROCESSED_COLUMN_LOG_SIZES.span());
        for (expected, actual) in zip_eq(
            COMPONENT_LOG_SIZES.to_fixed_array().span(), derived.span(),
        ) {
            assert!(*expected == *actual);
        }
    }

    /// `COMPONENT_LOG_SIZES` is committed in size-sorted (ascending) order.
    #[test]
    fn component_log_sizes_are_sorted() {
        assert!(is_sorted(COMPONENT_LOG_SIZES.to_fixed_array().span()));
    }
}
