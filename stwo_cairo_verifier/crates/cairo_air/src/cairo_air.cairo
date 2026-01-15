use components::memory_address_to_id::InteractionClaimImpl as MemoryAddressToIdInteractionClaimImpl;
use components::memory_id_to_big::{
    InteractionClaimImpl as MemoryIdToBigInteractionClaimImpl, LARGE_MEMORY_VALUE_ID_BASE,
};
use components::triple_xor_32::InteractionClaimImpl as TripleXor32InteractionClaimImpl;
use components::verify_bitwise_xor_12::InteractionClaimImpl as VerifyBitwiseXor12InteractionClaimImpl;
use components::verify_bitwise_xor_4::InteractionClaimImpl as VerifyBitwiseXor4InteractionClaimImpl;
use components::verify_bitwise_xor_7::InteractionClaimImpl as VerifyBitwiseXor7InteractionClaimImpl;
use components::verify_bitwise_xor_8::InteractionClaimImpl as VerifyBitwiseXor8InteractionClaimImpl;
use components::verify_bitwise_xor_9::InteractionClaimImpl as VerifyBitwiseXor9InteractionClaimImpl;
use components::verify_instruction::InteractionClaimImpl as VerifyInstructionInteractionClaimImpl;
use core::box::BoxImpl;
use core::num::traits::Zero;
use core::traits::TryInto;
use stwo_cairo_air::blake::*;
use stwo_cairo_air::builtins::*;
use stwo_cairo_air::cairo_component::CairoComponent;
use stwo_cairo_air::opcodes::*;
use stwo_verifier_core::fields::m31::M31;
use stwo_verifier_core::utils::OptionExTrait;
use crate::P_U32;

#[cfg(not(feature: "poseidon252_verifier"))]
pub mod pedersen_context_imports {
    pub use stwo_cairo_air::pedersen::{PedersenContextComponents, PedersenContextComponentsImpl};
}
#[cfg(not(feature: "poseidon252_verifier"))]
use pedersen_context_imports::*;
#[cfg(or(not(feature: "poseidon252_verifier"), feature: "poseidon_outputs_packing"))]
pub mod poseidon_context_imports {
    pub use stwo_cairo_air::poseidon::{PoseidonContextComponents, PoseidonContextComponentsImpl};
}
use core::array::Span;
#[cfg(or(not(feature: "poseidon252_verifier"), feature: "poseidon_outputs_packing"))]
use poseidon_context_imports::*;
use stwo_cairo_air::blake::{BlakeContextComponents, BlakeContextComponentsImpl};
use stwo_cairo_air::preprocessed_columns::PREPROCESSED_COLUMN_LOG_SIZE;
use stwo_cairo_air::range_checks::{RangeChecksComponents, RangeChecksComponentsImpl};
use stwo_cairo_air::{PublicDataImpl, components};
use stwo_constraint_framework::{
    LookupElementsImpl, PreprocessedMaskValues, PreprocessedMaskValuesImpl,
};
use stwo_verifier_core::circle::CirclePoint;
use stwo_verifier_core::fields::qm31::QM31;
use stwo_verifier_core::pcs::verifier::CommitmentSchemeVerifierImpl;
use stwo_verifier_core::utils::{ArrayImpl, OptionImpl, pow2};
use stwo_verifier_core::verifier::Air;
use stwo_verifier_core::{ColumnSpan, TreeArray, TreeSpan};
use crate::claims::{CairoClaim, CairoInteractionClaim};

pub const OPCODES_RELATION_ID: M31 = M31 { inner: 428564188 };
pub const MEMORY_ADDRESS_TO_ID_RELATION_ID: M31 = M31 { inner: 1444891767 };
pub const MEMORY_ID_TO_VALUE_RELATION_ID: M31 = M31 { inner: 1662111297 };
pub const VERIFY_BITWISE_XOR_12_RELATION_ID: M31 = M31 { inner: 648362599 };


/// Validates that every `mask_value` provided in the proof (in `sampled_values`) is used by at
/// least one component.
///
/// Since `eval_composition_polynomial_at_point` is responsible for validating the *structure*
/// of `sampled_values` in the proof, it needs to ensure that all sampled preprocessed
/// mask values are actually used. Otherwise, the prover would have the freedom to
/// send a sample of a column even if it is unused, adding another term to the FRI quotients.
///
/// Additionally, there is a sanity check that the columns in the trace and interaction-trace were
/// consumed by the components.
/// This is not strictly necessary as the verifier generates the column indices on its own and only
/// access samples of columns for which it knows about.
fn validate_mask_usage(
    preprocessed_mask_values: PreprocessedMaskValues,
    trace_mask_values: ColumnSpan<Span<QM31>>,
    interaction_trace_mask_values: ColumnSpan<Span<QM31>>,
) {
    preprocessed_mask_values.validate_usage();
    assert!(trace_mask_values.is_empty());
    assert!(interaction_trace_mask_values.is_empty());
}

/// Override the preprocessed trace log sizes, since they come from a global setting
/// rather than computed by concatenating preprocessed log sizes of the individual
/// components.
/// TODO(ilya): consider removing the generation of `_invalid_preprocessed_trace_log_sizes`.
pub fn override_preprocessed_trace_log_sizes(
    aggregated_log_sizes: TreeArray<Span<u32>>,
) -> TreeArray<Span<u32>> {
    let boxed_triplet: @Box<[Span<u32>; 3]> = aggregated_log_sizes.span().try_into().unwrap();
    let [_invalid_preprocessed_trace_log_sizes, trace_log_sizes, interaction_log_sizes] =
        boxed_triplet
        .unbox();

    array![PREPROCESSED_COLUMN_LOG_SIZE.span(), trace_log_sizes, interaction_log_sizes]
}

#[derive(Drop)]
#[cfg(not(feature: "poseidon252_verifier"))]
pub struct CairoAir {
    opcodes: OpcodeComponents,
    verify_instruction: components::verify_instruction::Component,
    blake_context: BlakeContextComponents,
    builtins: BuiltinComponents,
    pedersen_context: PedersenContextComponents,
    poseidon_context: PoseidonContextComponents,
    memory_address_to_id: components::memory_address_to_id::Component,
    memory_id_to_value: (
        Array<components::memory_id_to_big::BigComponent>,
        components::memory_id_to_big::SmallComponent,
    ),
    range_checks: RangeChecksComponents,
    verify_bitwise_xor_4: components::verify_bitwise_xor_4::Component,
    verify_bitwise_xor_7: components::verify_bitwise_xor_7::Component,
    verify_bitwise_xor_8: components::verify_bitwise_xor_8::Component,
    verify_bitwise_xor_9: components::verify_bitwise_xor_9::Component,
}

#[generate_trait]
#[cfg(not(feature: "poseidon252_verifier"))]
pub impl CairoAirNewImpl of CairoAirNewTrait {
    fn new(
        cairo_claim: @CairoClaim,
        common_lookup_elements: @crate::CommonLookupElements,
        interaction_claim: @CairoInteractionClaim,
    ) -> CairoAir {
        let opcode_components = OpcodeComponentsImpl::new(
            cairo_claim.add_opcode,
            cairo_claim.add_opcode_small,
            cairo_claim.add_ap_opcode,
            cairo_claim.assert_eq_opcode,
            cairo_claim.assert_eq_opcode_imm,
            cairo_claim.assert_eq_opcode_double_deref,
            cairo_claim.blake_compress_opcode,
            cairo_claim.call_opcode_abs,
            cairo_claim.call_opcode_rel_imm,
            cairo_claim.generic_opcode,
            cairo_claim.jnz_opcode_non_taken,
            cairo_claim.jnz_opcode_taken,
            cairo_claim.jump_opcode_abs,
            cairo_claim.jump_opcode_double_deref,
            cairo_claim.jump_opcode_rel,
            cairo_claim.jump_opcode_rel_imm,
            cairo_claim.mul_opcode,
            cairo_claim.mul_opcode_small,
            cairo_claim.qm_31_add_mul_opcode,
            cairo_claim.ret_opcode,
            common_lookup_elements,
            interaction_claim.add_opcode,
            interaction_claim.add_opcode_small,
            interaction_claim.add_ap_opcode,
            interaction_claim.assert_eq_opcode,
            interaction_claim.assert_eq_opcode_imm,
            interaction_claim.assert_eq_opcode_double_deref,
            interaction_claim.blake_compress_opcode,
            interaction_claim.call_opcode_abs,
            interaction_claim.call_opcode_rel_imm,
            interaction_claim.generic_opcode,
            interaction_claim.jnz_opcode_non_taken,
            interaction_claim.jnz_opcode_taken,
            interaction_claim.jump_opcode_abs,
            interaction_claim.jump_opcode_double_deref,
            interaction_claim.jump_opcode_rel,
            interaction_claim.jump_opcode_rel_imm,
            interaction_claim.mul_opcode,
            interaction_claim.mul_opcode_small,
            interaction_claim.qm_31_add_mul_opcode,
            interaction_claim.ret_opcode,
        );

        let blake_context_component = BlakeContextComponentsImpl::new(
            cairo_claim.blake_round,
            cairo_claim.blake_g,
            cairo_claim.blake_round_sigma,
            cairo_claim.triple_xor_32,
            cairo_claim.verify_bitwise_xor_12,
            common_lookup_elements,
            interaction_claim.blake_round,
            interaction_claim.blake_g,
            interaction_claim.blake_round_sigma,
            interaction_claim.triple_xor_32,
            interaction_claim.verify_bitwise_xor_12,
        );

        let builtins_components = BuiltinComponentsImpl::new(
            cairo_claim.add_mod_builtin,
            cairo_claim.bitwise_builtin,
            cairo_claim.mul_mod_builtin,
            cairo_claim.pedersen_builtin,
            cairo_claim.poseidon_builtin,
            cairo_claim.range_check96_builtin,
            cairo_claim.range_check_builtin,
            common_lookup_elements,
            interaction_claim.add_mod_builtin,
            interaction_claim.bitwise_builtin,
            interaction_claim.mul_mod_builtin,
            interaction_claim.pedersen_builtin,
            interaction_claim.poseidon_builtin,
            interaction_claim.range_check96_builtin,
            interaction_claim.range_check_builtin,
        );

        let pedersen_context_components = PedersenContextComponentsImpl::new(
            cairo_claim.pedersen_aggregator_window_bits_18,
            cairo_claim.partial_ec_mul_window_bits_18,
            cairo_claim.pedersen_points_table_window_bits_18,
            common_lookup_elements,
            interaction_claim.pedersen_aggregator_window_bits_18,
            interaction_claim.partial_ec_mul_window_bits_18,
            interaction_claim.pedersen_points_table_window_bits_18,
        );

        let poseidon_context_components = PoseidonContextComponentsImpl::new(
            cairo_claim.poseidon_aggregator,
            cairo_claim.poseidon_3_partial_rounds_chain,
            cairo_claim.poseidon_full_round_chain,
            cairo_claim.cube_252,
            cairo_claim.poseidon_round_keys,
            cairo_claim.range_check_252_width_27,
            common_lookup_elements,
            interaction_claim.poseidon_aggregator,
            interaction_claim.poseidon_3_partial_rounds_chain,
            interaction_claim.poseidon_full_round_chain,
            interaction_claim.cube_252,
            interaction_claim.poseidon_round_keys,
            interaction_claim.range_check_252_width_27,
        );

        let verifyinstruction_component = components::verify_instruction::NewComponentImpl::new(
            cairo_claim.verify_instruction.as_snap().unwrap(),
            interaction_claim.verify_instruction.as_snap().unwrap(),
            common_lookup_elements,
        );

        let memory_address_to_id_component =
            components::memory_address_to_id::NewComponentImpl::new(
            cairo_claim.memory_address_to_id.as_snap().unwrap(),
            interaction_claim.memory_address_to_id.as_snap().unwrap(),
            common_lookup_elements,
        );

        let claim_memory_id_to_big = cairo_claim.memory_id_to_big.as_snap().unwrap();
        let interaction_claim_memory_id_to_big = interaction_claim
            .memory_id_to_big
            .as_snap()
            .unwrap();
        assert!(
            claim_memory_id_to_big
                .big_log_sizes
                .len() == interaction_claim_memory_id_to_big
                .big_claimed_sums
                .len(),
        );
        let mut memory_id_to_value_components = array![];
        let mut offset: u32 = LARGE_MEMORY_VALUE_ID_BASE;
        for i in 0..claim_memory_id_to_big.big_log_sizes.len() {
            let log_size = claim_memory_id_to_big.big_log_sizes[i];
            let claimed_sum = interaction_claim_memory_id_to_big.big_claimed_sums[i];
            memory_id_to_value_components
                .append(
                    components::memory_id_to_big::NewBigComponentImpl::new(
                        *log_size, offset, *claimed_sum, common_lookup_elements,
                    ),
                );
            offset = offset + pow2(*log_size);
        }
        // Check that IDs in (ID -> Value) do not overflow P.
        assert!(offset <= P_U32);

        let small_memory_id_to_value_component =
            components::memory_id_to_big::NewSmallComponentImpl::new(
            *claim_memory_id_to_big.small_log_size,
            *interaction_claim_memory_id_to_big.small_claimed_sum,
            common_lookup_elements,
        );

        let range_checks_components = RangeChecksComponentsImpl::new(
            cairo_claim.range_check_6,
            cairo_claim.range_check_8,
            cairo_claim.range_check_11,
            cairo_claim.range_check_12,
            cairo_claim.range_check_18,
            cairo_claim.range_check_20,
            cairo_claim.range_check_4_3,
            cairo_claim.range_check_4_4,
            cairo_claim.range_check_9_9,
            cairo_claim.range_check_7_2_5,
            cairo_claim.range_check_3_6_6_3,
            cairo_claim.range_check_4_4_4_4,
            cairo_claim.range_check_3_3_3_3_3,
            common_lookup_elements,
            interaction_claim.range_check_6,
            interaction_claim.range_check_8,
            interaction_claim.range_check_11,
            interaction_claim.range_check_12,
            interaction_claim.range_check_18,
            interaction_claim.range_check_20,
            interaction_claim.range_check_4_3,
            interaction_claim.range_check_4_4,
            interaction_claim.range_check_9_9,
            interaction_claim.range_check_7_2_5,
            interaction_claim.range_check_3_6_6_3,
            interaction_claim.range_check_4_4_4_4,
            interaction_claim.range_check_3_3_3_3_3,
        );

        let verify_bitwise_xor_4_component =
            components::verify_bitwise_xor_4::NewComponentImpl::new(
            cairo_claim.verify_bitwise_xor_4.as_snap().unwrap(),
            interaction_claim.verify_bitwise_xor_4.as_snap().unwrap(),
            common_lookup_elements,
        );

        let verify_bitwise_xor_7_component =
            components::verify_bitwise_xor_7::NewComponentImpl::new(
            cairo_claim.verify_bitwise_xor_7.as_snap().unwrap(),
            interaction_claim.verify_bitwise_xor_7.as_snap().unwrap(),
            common_lookup_elements,
        );

        let verify_bitwise_xor_8_component =
            components::verify_bitwise_xor_8::NewComponentImpl::new(
            cairo_claim.verify_bitwise_xor_8.as_snap().unwrap(),
            interaction_claim.verify_bitwise_xor_8.as_snap().unwrap(),
            common_lookup_elements,
        );

        let verify_bitwise_xor_9_component =
            components::verify_bitwise_xor_9::NewComponentImpl::new(
            cairo_claim.verify_bitwise_xor_9.as_snap().unwrap(),
            interaction_claim.verify_bitwise_xor_9.as_snap().unwrap(),
            common_lookup_elements,
        );

        CairoAir {
            opcodes: opcode_components,
            verify_instruction: verifyinstruction_component,
            blake_context: blake_context_component,
            builtins: builtins_components,
            pedersen_context: pedersen_context_components,
            poseidon_context: poseidon_context_components,
            memory_address_to_id: memory_address_to_id_component,
            memory_id_to_value: (memory_id_to_value_components, small_memory_id_to_value_component),
            range_checks: range_checks_components,
            verify_bitwise_xor_4: verify_bitwise_xor_4_component,
            verify_bitwise_xor_7: verify_bitwise_xor_7_component,
            verify_bitwise_xor_8: verify_bitwise_xor_8_component,
            verify_bitwise_xor_9: verify_bitwise_xor_9_component,
        }
    }
}

#[cfg(not(feature: "poseidon252_verifier"))]
pub impl CairoAirImpl of Air<CairoAir> {
    fn eval_composition_polynomial_at_point(
        self: @CairoAir,
        point: CirclePoint<QM31>,
        mask_values: TreeSpan<ColumnSpan<Span<QM31>>>,
        random_coeff: QM31,
    ) -> QM31 {
        let mut sum = Zero::zero();

        let [
            preprocessed_mask_values,
            mut trace_mask_values,
            mut interaction_trace_mask_values,
            _composition_trace_mask_values,
        ]: [ColumnSpan<Span<QM31>>; 4] =
            (*mask_values
            .try_into()
            .unwrap())
            .unbox();

        let mut preprocessed_mask_values = PreprocessedMaskValuesImpl::new(
            preprocessed_mask_values,
        );
        let CairoAir {
            opcodes,
            verify_instruction,
            blake_context,
            builtins,
            pedersen_context,
            poseidon_context,
            memory_address_to_id,
            memory_id_to_value,
            range_checks,
            verify_bitwise_xor_4,
            verify_bitwise_xor_7,
            verify_bitwise_xor_8,
            verify_bitwise_xor_9,
        } = self;

        opcodes
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        verify_instruction
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        blake_context
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        builtins
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        pedersen_context
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        poseidon_context
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        memory_address_to_id
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        let (memory_id_to_value_big, memory_id_to_value_small) = memory_id_to_value;
        for memory_id_to_value_big_component in memory_id_to_value_big.span() {
            memory_id_to_value_big_component
                .evaluate_constraints_at_point(
                    ref sum,
                    ref preprocessed_mask_values,
                    ref trace_mask_values,
                    ref interaction_trace_mask_values,
                    random_coeff,
                    point,
                );
        }
        memory_id_to_value_small
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );

        range_checks
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        verify_bitwise_xor_4
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        verify_bitwise_xor_7
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        verify_bitwise_xor_8
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        verify_bitwise_xor_9
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );

        validate_mask_usage(
            preprocessed_mask_values, trace_mask_values, interaction_trace_mask_values,
        );
        sum
    }
}

#[derive(Drop)]
#[cfg(and(feature: "poseidon252_verifier", not(feature: "poseidon_outputs_packing")))]
pub struct CairoAir {
    opcodes: OpcodeComponents,
    verify_instruction: components::verify_instruction::Component,
    blake_context: BlakeContextComponents,
    builtins: BuiltinComponents,
    memory_address_to_id: components::memory_address_to_id::Component,
    memory_id_to_value: (
        Array<components::memory_id_to_big::BigComponent>,
        components::memory_id_to_big::SmallComponent,
    ),
    range_checks: RangeChecksComponents,
    verify_bitwise_xor_4: components::verify_bitwise_xor_4::Component,
    verify_bitwise_xor_7: components::verify_bitwise_xor_7::Component,
    verify_bitwise_xor_8: components::verify_bitwise_xor_8::Component,
    verify_bitwise_xor_9: components::verify_bitwise_xor_9::Component,
}

#[generate_trait]
#[cfg(and(feature: "poseidon252_verifier", not(feature: "poseidon_outputs_packing")))]
pub impl CairoAirNewImpl of CairoAirNewTrait {
    fn new(
        cairo_claim: @CairoClaim,
        common_lookup_elements: @crate::CommonLookupElements,
        interaction_claim: @CairoInteractionClaim,
    ) -> CairoAir {
        let opcode_components = OpcodeComponentsImpl::new(
            cairo_claim.add_opcode,
            cairo_claim.add_opcode_small,
            cairo_claim.add_ap_opcode,
            cairo_claim.assert_eq_opcode,
            cairo_claim.assert_eq_opcode_imm,
            cairo_claim.assert_eq_opcode_double_deref,
            cairo_claim.blake_compress_opcode,
            cairo_claim.call_opcode_abs,
            cairo_claim.call_opcode_rel_imm,
            cairo_claim.generic_opcode,
            cairo_claim.jnz_opcode_non_taken,
            cairo_claim.jnz_opcode_taken,
            cairo_claim.jump_opcode_abs,
            cairo_claim.jump_opcode_double_deref,
            cairo_claim.jump_opcode_rel,
            cairo_claim.jump_opcode_rel_imm,
            cairo_claim.mul_opcode,
            cairo_claim.mul_opcode_small,
            cairo_claim.qm_31_add_mul_opcode,
            cairo_claim.ret_opcode,
            common_lookup_elements,
            interaction_claim.add_opcode,
            interaction_claim.add_opcode_small,
            interaction_claim.add_ap_opcode,
            interaction_claim.assert_eq_opcode,
            interaction_claim.assert_eq_opcode_imm,
            interaction_claim.assert_eq_opcode_double_deref,
            interaction_claim.blake_compress_opcode,
            interaction_claim.call_opcode_abs,
            interaction_claim.call_opcode_rel_imm,
            interaction_claim.generic_opcode,
            interaction_claim.jnz_opcode_non_taken,
            interaction_claim.jnz_opcode_taken,
            interaction_claim.jump_opcode_abs,
            interaction_claim.jump_opcode_double_deref,
            interaction_claim.jump_opcode_rel,
            interaction_claim.jump_opcode_rel_imm,
            interaction_claim.mul_opcode,
            interaction_claim.mul_opcode_small,
            interaction_claim.qm_31_add_mul_opcode,
            interaction_claim.ret_opcode,
        );

        let blake_context_component = BlakeContextComponentsImpl::new(
            cairo_claim.blake_round,
            cairo_claim.blake_g,
            cairo_claim.blake_round_sigma,
            cairo_claim.triple_xor_32,
            cairo_claim.verify_bitwise_xor_12,
            common_lookup_elements,
            interaction_claim.blake_round,
            interaction_claim.blake_g,
            interaction_claim.blake_round_sigma,
            interaction_claim.triple_xor_32,
            interaction_claim.verify_bitwise_xor_12,
        );

        let builtins_components = BuiltinComponentsImpl::new(
            cairo_claim.add_mod_builtin,
            cairo_claim.bitwise_builtin,
            cairo_claim.mul_mod_builtin,
            cairo_claim.pedersen_builtin,
            cairo_claim.poseidon_builtin,
            cairo_claim.range_check96_builtin,
            cairo_claim.range_check_builtin,
            common_lookup_elements,
            interaction_claim.add_mod_builtin,
            interaction_claim.bitwise_builtin,
            interaction_claim.mul_mod_builtin,
            interaction_claim.pedersen_builtin,
            interaction_claim.poseidon_builtin,
            interaction_claim.range_check96_builtin,
            interaction_claim.range_check_builtin,
        );

        let verifyinstruction_component = components::verify_instruction::NewComponentImpl::new(
            cairo_claim.verify_instruction.as_snap().unwrap(),
            interaction_claim.verify_instruction.as_snap().unwrap(),
            common_lookup_elements,
        );

        let memory_address_to_id_component =
            components::memory_address_to_id::NewComponentImpl::new(
            cairo_claim.memory_address_to_id.as_snap().unwrap(),
            interaction_claim.memory_address_to_id.as_snap().unwrap(),
            common_lookup_elements,
        );

        let claim_memory_id_to_big = cairo_claim.memory_id_to_big.as_snap().unwrap();
        let interaction_claim_memory_id_to_big = interaction_claim
            .memory_id_to_big
            .as_snap()
            .unwrap();
        assert!(
            claim_memory_id_to_big
                .big_log_sizes
                .len() == interaction_claim_memory_id_to_big
                .big_claimed_sums
                .len(),
        );
        let mut memory_id_to_value_components = array![];
        let mut offset: u32 = LARGE_MEMORY_VALUE_ID_BASE;
        for i in 0..claim_memory_id_to_big.big_log_sizes.len() {
            let log_size = claim_memory_id_to_big.big_log_sizes[i];
            let claimed_sum = interaction_claim_memory_id_to_big.big_claimed_sums[i];
            memory_id_to_value_components
                .append(
                    components::memory_id_to_big::NewBigComponentImpl::new(
                        *log_size, offset, *claimed_sum, common_lookup_elements,
                    ),
                );
            offset = offset + pow2(*log_size);
        }
        // Check that IDs in (ID -> Value) do not overflow P.
        assert!(offset <= P_U32);

        let small_memory_id_to_value_component =
            components::memory_id_to_big::NewSmallComponentImpl::new(
            *claim_memory_id_to_big.small_log_size,
            *interaction_claim_memory_id_to_big.small_claimed_sum,
            common_lookup_elements,
        );

        let range_checks_components = RangeChecksComponentsImpl::new(
            cairo_claim.range_check_6,
            cairo_claim.range_check_8,
            cairo_claim.range_check_11,
            cairo_claim.range_check_12,
            cairo_claim.range_check_18,
            cairo_claim.range_check_20,
            cairo_claim.range_check_4_3,
            cairo_claim.range_check_4_4,
            cairo_claim.range_check_9_9,
            cairo_claim.range_check_7_2_5,
            cairo_claim.range_check_3_6_6_3,
            cairo_claim.range_check_4_4_4_4,
            cairo_claim.range_check_3_3_3_3_3,
            common_lookup_elements,
            interaction_claim.range_check_6,
            interaction_claim.range_check_8,
            interaction_claim.range_check_11,
            interaction_claim.range_check_12,
            interaction_claim.range_check_18,
            interaction_claim.range_check_20,
            interaction_claim.range_check_4_3,
            interaction_claim.range_check_4_4,
            interaction_claim.range_check_9_9,
            interaction_claim.range_check_7_2_5,
            interaction_claim.range_check_3_6_6_3,
            interaction_claim.range_check_4_4_4_4,
            interaction_claim.range_check_3_3_3_3_3,
        );

        let verify_bitwise_xor_4_component =
            components::verify_bitwise_xor_4::NewComponentImpl::new(
            cairo_claim.verify_bitwise_xor_4.as_snap().unwrap(),
            interaction_claim.verify_bitwise_xor_4.as_snap().unwrap(),
            common_lookup_elements,
        );

        let verify_bitwise_xor_7_component =
            components::verify_bitwise_xor_7::NewComponentImpl::new(
            cairo_claim.verify_bitwise_xor_7.as_snap().unwrap(),
            interaction_claim.verify_bitwise_xor_7.as_snap().unwrap(),
            common_lookup_elements,
        );

        let verify_bitwise_xor_8_component =
            components::verify_bitwise_xor_8::NewComponentImpl::new(
            cairo_claim.verify_bitwise_xor_8.as_snap().unwrap(),
            interaction_claim.verify_bitwise_xor_8.as_snap().unwrap(),
            common_lookup_elements,
        );

        let verify_bitwise_xor_9_component =
            components::verify_bitwise_xor_9::NewComponentImpl::new(
            cairo_claim.verify_bitwise_xor_9.as_snap().unwrap(),
            interaction_claim.verify_bitwise_xor_9.as_snap().unwrap(),
            common_lookup_elements,
        );

        CairoAir {
            opcodes: opcode_components,
            verify_instruction: verifyinstruction_component,
            blake_context: blake_context_component,
            builtins: builtins_components,
            memory_address_to_id: memory_address_to_id_component,
            memory_id_to_value: (memory_id_to_value_components, small_memory_id_to_value_component),
            range_checks: range_checks_components,
            verify_bitwise_xor_4: verify_bitwise_xor_4_component,
            verify_bitwise_xor_7: verify_bitwise_xor_7_component,
            verify_bitwise_xor_8: verify_bitwise_xor_8_component,
            verify_bitwise_xor_9: verify_bitwise_xor_9_component,
        }
    }
}

#[cfg(and(feature: "poseidon252_verifier", not(feature: "poseidon_outputs_packing")))]
pub impl CairoAirImpl of Air<CairoAir> {
    fn eval_composition_polynomial_at_point(
        self: @CairoAir,
        point: CirclePoint<QM31>,
        mask_values: TreeSpan<ColumnSpan<Span<QM31>>>,
        random_coeff: QM31,
    ) -> QM31 {
        let mut sum = Zero::zero();

        let [
            preprocessed_mask_values,
            mut trace_mask_values,
            mut interaction_trace_mask_values,
            _composition_trace_mask_values,
        ]: [ColumnSpan<Span<QM31>>; 4] =
            (*mask_values
            .try_into()
            .unwrap())
            .unbox();

        let mut preprocessed_mask_values = PreprocessedMaskValuesImpl::new(
            preprocessed_mask_values,
        );

        let CairoAir {
            opcodes,
            verify_instruction,
            blake_context,
            builtins,
            memory_address_to_id,
            memory_id_to_value,
            range_checks,
            verify_bitwise_xor_4,
            verify_bitwise_xor_7,
            verify_bitwise_xor_8,
            verify_bitwise_xor_9,
        } = self;

        opcodes
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        verify_instruction
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        blake_context
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        builtins
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        memory_address_to_id
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        let (memory_id_to_value_big, memory_id_to_value_small) = memory_id_to_value;
        for memory_id_to_value_big_component in memory_id_to_value_big.span() {
            memory_id_to_value_big_component
                .evaluate_constraints_at_point(
                    ref sum,
                    ref preprocessed_mask_values,
                    ref trace_mask_values,
                    ref interaction_trace_mask_values,
                    random_coeff,
                    point,
                );
        }
        memory_id_to_value_small
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        range_checks
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        verify_bitwise_xor_4
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        verify_bitwise_xor_7
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        verify_bitwise_xor_8
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        verify_bitwise_xor_9
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );

        validate_mask_usage(
            preprocessed_mask_values, trace_mask_values, interaction_trace_mask_values,
        );
        sum
    }
}


#[derive(Drop)]
#[cfg(and(feature: "poseidon252_verifier", feature: "poseidon_outputs_packing"))]
pub struct CairoAir {
    opcodes: OpcodeComponents,
    verify_instruction: components::verify_instruction::Component,
    blake_context: BlakeContextComponents,
    builtins: BuiltinComponents,
    poseidon_context: PoseidonContextComponents,
    memory_address_to_id: components::memory_address_to_id::Component,
    memory_id_to_value: (
        Array<components::memory_id_to_big::BigComponent>,
        components::memory_id_to_big::SmallComponent,
    ),
    range_checks: RangeChecksComponents,
    verify_bitwise_xor_4: components::verify_bitwise_xor_4::Component,
    verify_bitwise_xor_7: components::verify_bitwise_xor_7::Component,
    verify_bitwise_xor_8: components::verify_bitwise_xor_8::Component,
    verify_bitwise_xor_9: components::verify_bitwise_xor_9::Component,
}

#[generate_trait]
#[cfg(and(feature: "poseidon252_verifier", feature: "poseidon_outputs_packing"))]
pub impl CairoAirNewImpl of CairoAirNewTrait {
    fn new(
        cairo_claim: @CairoClaim,
        common_lookup_elements: @crate::CommonLookupElements,
        interaction_claim: @CairoInteractionClaim,
    ) -> CairoAir {
        let opcode_components = OpcodeComponentsImpl::new(
            cairo_claim.add_opcode,
            cairo_claim.add_opcode_small,
            cairo_claim.add_ap_opcode,
            cairo_claim.assert_eq_opcode,
            cairo_claim.assert_eq_opcode_imm,
            cairo_claim.assert_eq_opcode_double_deref,
            cairo_claim.blake_compress_opcode,
            cairo_claim.call_opcode_abs,
            cairo_claim.call_opcode_rel_imm,
            cairo_claim.generic_opcode,
            cairo_claim.jnz_opcode_non_taken,
            cairo_claim.jnz_opcode_taken,
            cairo_claim.jump_opcode_abs,
            cairo_claim.jump_opcode_double_deref,
            cairo_claim.jump_opcode_rel,
            cairo_claim.jump_opcode_rel_imm,
            cairo_claim.mul_opcode,
            cairo_claim.mul_opcode_small,
            cairo_claim.qm_31_add_mul_opcode,
            cairo_claim.ret_opcode,
            common_lookup_elements,
            interaction_claim.add_opcode,
            interaction_claim.add_opcode_small,
            interaction_claim.add_ap_opcode,
            interaction_claim.assert_eq_opcode,
            interaction_claim.assert_eq_opcode_imm,
            interaction_claim.assert_eq_opcode_double_deref,
            interaction_claim.blake_compress_opcode,
            interaction_claim.call_opcode_abs,
            interaction_claim.call_opcode_rel_imm,
            interaction_claim.generic_opcode,
            interaction_claim.jnz_opcode_non_taken,
            interaction_claim.jnz_opcode_taken,
            interaction_claim.jump_opcode_abs,
            interaction_claim.jump_opcode_double_deref,
            interaction_claim.jump_opcode_rel,
            interaction_claim.jump_opcode_rel_imm,
            interaction_claim.mul_opcode,
            interaction_claim.mul_opcode_small,
            interaction_claim.qm_31_add_mul_opcode,
            interaction_claim.ret_opcode,
        );

        let blake_context_component = BlakeContextComponentsImpl::new(
            cairo_claim.blake_round,
            cairo_claim.blake_g,
            cairo_claim.blake_round_sigma,
            cairo_claim.triple_xor_32,
            cairo_claim.verify_bitwise_xor_12,
            common_lookup_elements,
            interaction_claim.blake_round,
            interaction_claim.blake_g,
            interaction_claim.blake_round_sigma,
            interaction_claim.triple_xor_32,
            interaction_claim.verify_bitwise_xor_12,
        );

        let builtins_components = BuiltinComponentsImpl::new(
            cairo_claim.add_mod_builtin,
            cairo_claim.bitwise_builtin,
            cairo_claim.mul_mod_builtin,
            cairo_claim.pedersen_builtin,
            cairo_claim.poseidon_builtin,
            cairo_claim.range_check96_builtin,
            cairo_claim.range_check_builtin,
            common_lookup_elements,
            interaction_claim.add_mod_builtin,
            interaction_claim.bitwise_builtin,
            interaction_claim.mul_mod_builtin,
            interaction_claim.pedersen_builtin,
            interaction_claim.poseidon_builtin,
            interaction_claim.range_check96_builtin,
            interaction_claim.range_check_builtin,
        );

        let poseidon_context_components = PoseidonContextComponentsImpl::new(
            cairo_claim.poseidon_aggregator,
            cairo_claim.poseidon_3_partial_rounds_chain,
            cairo_claim.poseidon_full_round_chain,
            cairo_claim.cube_252,
            cairo_claim.poseidon_round_keys,
            cairo_claim.range_check_252_width_27,
            common_lookup_elements,
            interaction_claim.poseidon_aggregator,
            interaction_claim.poseidon_3_partial_rounds_chain,
            interaction_claim.poseidon_full_round_chain,
            interaction_claim.cube_252,
            interaction_claim.poseidon_round_keys,
            interaction_claim.range_check_252_width_27,
        );

        let verifyinstruction_component = components::verify_instruction::NewComponentImpl::new(
            cairo_claim.verify_instruction.as_snap().unwrap(),
            interaction_claim.verify_instruction.as_snap().unwrap(),
            common_lookup_elements,
        );

        let memory_address_to_id_component =
            components::memory_address_to_id::NewComponentImpl::new(
            cairo_claim.memory_address_to_id.as_snap().unwrap(),
            interaction_claim.memory_address_to_id.as_snap().unwrap(),
            common_lookup_elements,
        );

        let claim_memory_id_to_big = cairo_claim.memory_id_to_big.as_snap().unwrap();
        let interaction_claim_memory_id_to_big = interaction_claim
            .memory_id_to_big
            .as_snap()
            .unwrap();
        assert!(
            claim_memory_id_to_big
                .big_log_sizes
                .len() == interaction_claim_memory_id_to_big
                .big_claimed_sums
                .len(),
        );
        let mut memory_id_to_value_components = array![];
        let mut offset: u32 = LARGE_MEMORY_VALUE_ID_BASE;
        for i in 0..claim_memory_id_to_big.big_log_sizes.len() {
            let log_size = claim_memory_id_to_big.big_log_sizes[i];
            let claimed_sum = interaction_claim_memory_id_to_big.big_claimed_sums[i];
            memory_id_to_value_components
                .append(
                    components::memory_id_to_big::NewBigComponentImpl::new(
                        *log_size, offset, *claimed_sum, common_lookup_elements,
                    ),
                );
            offset = offset + pow2(log_size);
        }
        // Check that IDs in (ID -> Value) do not overflow P.
        assert!(offset <= P_U32);

        let small_memory_id_to_value_component =
            components::memory_id_to_big::NewSmallComponentImpl::new(
            claim_memory_id_to_big.small_log_size,
            interaction_claim_memory_id_to_big.small_claimed_sum,
            common_lookup_elements,
        );

        let range_checks_components = RangeChecksComponentsImpl::new(
            cairo_claim.range_check_6,
            cairo_claim.range_check_8,
            cairo_claim.range_check_11,
            cairo_claim.range_check_12,
            cairo_claim.range_check_18,
            cairo_claim.range_check_20,
            cairo_claim.range_check_4_3,
            cairo_claim.range_check_4_4,
            cairo_claim.range_check_9_9,
            cairo_claim.range_check_7_2_5,
            cairo_claim.range_check_3_6_6_3,
            cairo_claim.range_check_4_4_4_4,
            cairo_claim.range_check_3_3_3_3_3,
            common_lookup_elements,
            interaction_claim.range_check_6,
            interaction_claim.range_check_8,
            interaction_claim.range_check_11,
            interaction_claim.range_check_12,
            interaction_claim.range_check_18,
            interaction_claim.range_check_20,
            interaction_claim.range_check_4_3,
            interaction_claim.range_check_4_4,
            interaction_claim.range_check_9_9,
            interaction_claim.range_check_7_2_5,
            interaction_claim.range_check_3_6_6_3,
            interaction_claim.range_check_4_4_4_4,
            interaction_claim.range_check_3_3_3_3_3,
        );

        let verify_bitwise_xor_4_component =
            components::verify_bitwise_xor_4::NewComponentImpl::new(
            cairo_claim.verify_bitwise_xor_4.as_snap().unwrap(),
            interaction_claim.verify_bitwise_xor_4.as_snap().unwrap(),
            common_lookup_elements,
        );

        let verify_bitwise_xor_7_component =
            components::verify_bitwise_xor_7::NewComponentImpl::new(
            cairo_claim.verify_bitwise_xor_7.as_snap().unwrap(),
            interaction_claim.verify_bitwise_xor_7.as_snap().unwrap(),
            common_lookup_elements,
        );

        let verify_bitwise_xor_8_component =
            components::verify_bitwise_xor_8::NewComponentImpl::new(
            cairo_claim.verify_bitwise_xor_8.as_snap().unwrap(),
            interaction_claim.verify_bitwise_xor_8.as_snap().unwrap(),
            common_lookup_elements,
        );

        let verify_bitwise_xor_9_component =
            components::verify_bitwise_xor_9::NewComponentImpl::new(
            cairo_claim.verify_bitwise_xor_9.as_snap().unwrap(),
            interaction_claim.verify_bitwise_xor_9.as_snap().unwrap(),
            common_lookup_elements,
        );

        CairoAir {
            opcodes: opcode_components,
            verify_instruction: verifyinstruction_component,
            blake_context: blake_context_component,
            builtins: builtins_components,
            poseidon_context: poseidon_context_components,
            memory_address_to_id: memory_address_to_id_component,
            memory_id_to_value: (memory_id_to_value_components, small_memory_id_to_value_component),
            range_checks: range_checks_components,
            verify_bitwise_xor_4: verify_bitwise_xor_4_component,
            verify_bitwise_xor_7: verify_bitwise_xor_7_component,
            verify_bitwise_xor_8: verify_bitwise_xor_8_component,
            verify_bitwise_xor_9: verify_bitwise_xor_9_component,
        }
    }
}

#[cfg(and(feature: "poseidon252_verifier", feature: "poseidon_outputs_packing"))]
pub impl CairoAirImpl of Air<CairoAir> {
    fn eval_composition_polynomial_at_point(
        self: @CairoAir,
        point: CirclePoint<QM31>,
        mask_values: TreeSpan<ColumnSpan<Span<QM31>>>,
        random_coeff: QM31,
    ) -> QM31 {
        let mut sum = Zero::zero();

        let [
            preprocessed_mask_values,
            mut trace_mask_values,
            mut interaction_trace_mask_values,
            _composition_trace_mask_values,
        ]: [ColumnSpan<Span<QM31>>; 4] =
            (*mask_values
            .try_into()
            .unwrap())
            .unbox();

        let mut preprocessed_mask_values = PreprocessedMaskValuesImpl::new(
            preprocessed_mask_values,
        );

        let CairoAir {
            opcodes,
            verify_instruction,
            blake_context,
            builtins,
            poseidon_context,
            memory_address_to_id,
            memory_id_to_value,
            range_checks,
            verify_bitwise_xor_4,
            verify_bitwise_xor_7,
            verify_bitwise_xor_8,
            verify_bitwise_xor_9,
        } = self;

        opcodes
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        verify_instruction
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        blake_context
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        builtins
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        poseidon_context
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        memory_address_to_id
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        let (memory_id_to_value_big, memory_id_to_value_small) = memory_id_to_value;
        for memory_id_to_value_big_component in memory_id_to_value_big.span() {
            memory_id_to_value_big_component
                .evaluate_constraints_at_point(
                    ref sum,
                    ref preprocessed_mask_values,
                    ref trace_mask_values,
                    ref interaction_trace_mask_values,
                    random_coeff,
                    point,
                );
        }
        memory_id_to_value_small
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        range_checks
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        verify_bitwise_xor_4
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        verify_bitwise_xor_7
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        verify_bitwise_xor_8
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        verify_bitwise_xor_9
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );

        validate_mask_usage(
            preprocessed_mask_values, trace_mask_values, interaction_trace_mask_values,
        );
        sum
    }
}
