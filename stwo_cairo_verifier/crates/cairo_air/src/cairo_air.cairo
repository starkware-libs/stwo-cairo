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
use core::array::Span;
use core::box::BoxImpl;
use core::num::traits::Zero;
use core::traits::TryInto;
use stwo_cairo_air::blake::{BlakeContextComponents, BlakeContextComponentsImpl};
use stwo_cairo_air::builtins::{BuiltinComponents, BuiltinComponentsImpl};
use stwo_cairo_air::cairo_component::CairoComponent;
use stwo_cairo_air::claim::ClaimTrait;
use stwo_cairo_air::opcodes::{OpcodeComponents, OpcodeComponentsImpl, opcodes_log_sizes};
use stwo_verifier_core::fields::m31::M31;
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
#[cfg(or(not(feature: "poseidon252_verifier"), feature: "poseidon_outputs_packing"))]
use poseidon_context_imports::*;
use stwo_cairo_air::blake::blake_context_log_sizes;
use stwo_cairo_air::builtins::builtins_log_sizes;
use stwo_cairo_air::pedersen::pedersen_context_log_sizes;
use stwo_cairo_air::poseidon::poseidon_context_log_sizes;
use stwo_cairo_air::preprocessed_columns::PREPROCESSED_COLUMN_LOG_SIZE;
use stwo_cairo_air::range_checks::{
    RangeChecksComponents, RangeChecksComponentsImpl, range_checks_log_sizes,
};
use stwo_cairo_air::{PublicDataImpl, components, utils};
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

pub fn log_sizes(claim: @CairoClaim) -> TreeArray<Span<u32>> {
    let mut aggregated_log_sizes = utils::tree_array_concat_cols(
        array![
            opcodes_log_sizes(claim), claim.verify_instruction.unwrap().log_sizes(),
            blake_context_log_sizes(claim), builtins_log_sizes(claim),
            pedersen_context_log_sizes(claim), poseidon_context_log_sizes(claim),
            claim.memory_address_to_id.unwrap().log_sizes(),
            claim.memory_id_to_big.as_snap().unwrap().log_sizes(), range_checks_log_sizes(claim),
            claim.verify_bitwise_xor_4.unwrap().log_sizes(),
            claim.verify_bitwise_xor_7.unwrap().log_sizes(),
            claim.verify_bitwise_xor_8.unwrap().log_sizes(),
            claim.verify_bitwise_xor_9.unwrap().log_sizes(),
        ],
    );

    // Override the preprocessed trace log sizes, since they come from a global setting
    // rather than computed by concatenating preprocessed log sizes of the individual
    // components.
    // TODO(ilya): consider removing the generation of `_invalid_preprocessed_trace_log_sizes`.
    let boxed_triplet: Box<[Span<u32>; 3]> = *aggregated_log_sizes.span().try_into().unwrap();
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
            cairo_claim, common_lookup_elements, interaction_claim,
        );

        let blake_context_component = BlakeContextComponentsImpl::new(
            cairo_claim, common_lookup_elements, interaction_claim,
        );

        let builtins_components = BuiltinComponentsImpl::new(
            cairo_claim, common_lookup_elements, interaction_claim,
        );

        let pedersen_context_components = PedersenContextComponentsImpl::new(
            cairo_claim, common_lookup_elements, interaction_claim,
        );

        let poseidon_context_components = PoseidonContextComponentsImpl::new(
            cairo_claim, common_lookup_elements, interaction_claim,
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
            cairo_claim, common_lookup_elements, interaction_claim,
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
            );
        verify_instruction
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
            );
        blake_context
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
            );
        builtins
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
            );
        pedersen_context
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
            );
        poseidon_context
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
            );
        memory_address_to_id
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
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
                );
        }
        memory_id_to_value_small
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
            );

        range_checks
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
            );
        verify_bitwise_xor_4
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
            );
        verify_bitwise_xor_7
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
            );
        verify_bitwise_xor_8
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
            );
        verify_bitwise_xor_9
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
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
            cairo_claim, common_lookup_elements, interaction_claim,
        );

        let blake_context_component = BlakeContextComponentsImpl::new(
            cairo_claim, common_lookup_elements, interaction_claim,
        );

        let builtins_components = BuiltinComponentsImpl::new(
            cairo_claim, common_lookup_elements, interaction_claim,
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
            cairo_claim, common_lookup_elements, interaction_claim,
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
            );
        verify_instruction
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
            );
        blake_context
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
            );
        builtins
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
            );
        memory_address_to_id
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
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
                );
        }
        memory_id_to_value_small
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
            );
        range_checks
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
            );
        verify_bitwise_xor_4
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
            );
        verify_bitwise_xor_7
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
            );
        verify_bitwise_xor_8
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
            );
        verify_bitwise_xor_9
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
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
            cairo_claim, common_lookup_elements, interaction_claim,
        );

        let blake_context_component = BlakeContextComponentsImpl::new(
            cairo_claim, common_lookup_elements, interaction_claim,
        );

        let builtins_components = BuiltinComponentsImpl::new(
            cairo_claim, common_lookup_elements, interaction_claim,
        );

        let poseidon_context_components = PoseidonContextComponentsImpl::new(
            cairo_claim, common_lookup_elements, interaction_claim,
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
            cairo_claim, common_lookup_elements, interaction_claim,
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
            );
        verify_instruction
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
            );
        blake_context
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
            );
        builtins
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
            );
        poseidon_context
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
            );
        memory_address_to_id
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
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
                );
        }
        memory_id_to_value_small
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
            );
        range_checks
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
            );
        verify_bitwise_xor_4
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
            );
        verify_bitwise_xor_7
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
            );
        verify_bitwise_xor_8
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
            );
        verify_bitwise_xor_9
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
            );

        validate_mask_usage(
            preprocessed_mask_values, trace_mask_values, interaction_trace_mask_values,
        );
        sum
    }
}
