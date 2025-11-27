use components::memory_address_to_id::InteractionClaimImpl as MemoryAddressToIdInteractionClaimImpl;
use components::memory_id_to_big::{
    InteractionClaimImpl as MemoryIdToBigInteractionClaimImpl, LARGE_MEMORY_VALUE_ID_BASE,
};
use components::triple_xor_32::InteractionClaimImpl as TripleXor32InteractionClaimImpl;
use components::verify_bitwise_xor_12::InteractionClaimImpl as VerifyBitwiseXor12InteractionClaimImpl;
use components::verify_bitwise_xor_4::InteractionClaimImpl as VerifyBitwiseXor4InteractionClaimImpl;
use components::verify_bitwise_xor_7::InteractionClaimImpl as VerifyBitwiseXor7InteractionClaimImpl;
use components::verify_bitwise_xor_8::InteractionClaimImpl as VerifyBitwiseXor8InteractionClaimImpl;
use components::verify_bitwise_xor_8_b::InteractionClaimImpl as VerifyBitwiseXor8BInteractionClaimImpl;
use components::verify_bitwise_xor_9::InteractionClaimImpl as VerifyBitwiseXor9InteractionClaimImpl;
use components::verify_instruction::InteractionClaimImpl as VerifyInstructionInteractionClaimImpl;
use core::box::BoxImpl;
use core::num::traits::Zero;
use stwo_cairo_air::blake::*;
use stwo_cairo_air::builtins::*;
use stwo_cairo_air::cairo_component::CairoComponent;
use stwo_cairo_air::claim::ClaimTrait;
use stwo_cairo_air::opcodes::*;
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
use stwo_cairo_air::blake::{
    BlakeContextClaim, BlakeContextComponents, BlakeContextComponentsImpl,
    BlakeContextInteractionClaim, BlakeContextInteractionClaimImpl,
};
use stwo_cairo_air::builtins::{
    BuiltinsClaim, BuiltinsInteractionClaim, BuiltinsInteractionClaimImpl,
};
use stwo_cairo_air::pedersen::{
    PedersenContextClaim, PedersenContextInteractionClaim, PedersenContextInteractionClaimImpl,
};
use stwo_cairo_air::poseidon::{
    PoseidonContextClaim, PoseidonContextInteractionClaim, PoseidonContextInteractionClaimImpl,
};
use stwo_cairo_air::preprocessed_columns::PREPROCESSED_COLUMN_LOG_SIZE;
use stwo_cairo_air::range_checks::{
    RangeChecksClaim, RangeChecksComponents, RangeChecksComponentsImpl, RangeChecksInteractionClaim,
    RangeChecksInteractionClaimImpl, RangeChecksInteractionElements,
    RangeChecksInteractionElementsImpl,
};
use stwo_cairo_air::{PublicData, PublicDataImpl, RelationUsesDict, components, utils};
use stwo_constraint_framework::{
    LookupElements, LookupElementsImpl, PreprocessedMaskValues, PreprocessedMaskValuesImpl,
};
use stwo_verifier_core::channel::Channel;
use stwo_verifier_core::circle::CirclePoint;
use stwo_verifier_core::fields::qm31::QM31;
use stwo_verifier_core::pcs::verifier::CommitmentSchemeVerifierImpl;
use stwo_verifier_core::utils::{ArrayImpl, OptionImpl, pow2};
use stwo_verifier_core::verifier::Air;
use stwo_verifier_core::{ColumnSpan, TreeArray, TreeSpan};


pub type Cube252Elements = LookupElements<20>;

pub type MemoryAddressToIdElements = LookupElements<2>;

pub type MemoryIdToBigElements = LookupElements<29>;

pub type OpcodesElements = LookupElements<3>;

pub type PedersenAggregatorElements = LookupElements<3>;

pub type PartialEcMulElements = LookupElements<72>;

pub type PedersenPointsTableElements = LookupElements<57>;

pub type PoseidonFullRoundChainElements = LookupElements<32>;

pub type Poseidon3PartialRoundsChainElements = LookupElements<42>;

pub type PoseidonAggregatorElements = LookupElements<6>;

pub type PoseidonRoundKeysElements = LookupElements<31>;

pub type BlakeGElements = LookupElements<20>;

pub type BlakeRoundElements = LookupElements<35>;

pub type BlakeRoundSigmaElements = LookupElements<17>;

pub type TripleXor32Elements = LookupElements<8>;

pub type RangeCheck252Width27Elements = LookupElements<10>;

pub type VerifyInstructionElements = LookupElements<7>;

pub type VerifyBitwiseXor_4Elements = LookupElements<3>;

pub type VerifyBitwiseXor_7Elements = LookupElements<3>;

pub type VerifyBitwiseXor_8Elements = LookupElements<3>;

pub type VerifyBitwiseXor_8_BElements = LookupElements<3>;

pub type VerifyBitwiseXor_9Elements = LookupElements<3>;

pub type VerifyBitwiseXor_12Elements = LookupElements<3>;

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

#[derive(Drop, Serde)]
pub struct CairoClaim {
    pub public_data: PublicData,
    pub opcodes: OpcodeClaim,
    pub verify_instruction: components::verify_instruction::Claim,
    pub blake_context: BlakeContextClaim,
    pub builtins: BuiltinsClaim,
    pub pedersen_context: PedersenContextClaim,
    pub poseidon_context: PoseidonContextClaim,
    pub memory_address_to_id: components::memory_address_to_id::Claim,
    pub memory_id_to_value: components::memory_id_to_big::Claim,
    pub range_checks: RangeChecksClaim,
    pub verify_bitwise_xor_4: components::verify_bitwise_xor_4::Claim,
    pub verify_bitwise_xor_7: components::verify_bitwise_xor_7::Claim,
    pub verify_bitwise_xor_8: components::verify_bitwise_xor_8::Claim,
    pub verify_bitwise_xor_8_b: components::verify_bitwise_xor_8_b::Claim,
    pub verify_bitwise_xor_9: components::verify_bitwise_xor_9::Claim,
    // ...
}

pub impl CairoClaimImpl of ClaimTrait<CairoClaim> {
    fn log_sizes(self: @CairoClaim) -> TreeArray<Span<u32>> {
        let mut aggregated_log_sizes = utils::tree_array_concat_cols(
            array![
                self.opcodes.log_sizes(), self.verify_instruction.log_sizes(),
                self.blake_context.log_sizes(), self.builtins.log_sizes(),
                self.pedersen_context.log_sizes(), self.poseidon_context.log_sizes(),
                self.memory_address_to_id.log_sizes(), self.memory_id_to_value.log_sizes(),
                self.range_checks.log_sizes(), self.verify_bitwise_xor_4.log_sizes(),
                self.verify_bitwise_xor_7.log_sizes(), self.verify_bitwise_xor_8.log_sizes(),
                self.verify_bitwise_xor_8_b.log_sizes(), self.verify_bitwise_xor_9.log_sizes(),
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

    fn mix_into(self: @CairoClaim, ref channel: Channel) {
        let CairoClaim {
            public_data,
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
            verify_bitwise_xor_8_b,
            verify_bitwise_xor_9,
        } = self;

        public_data.mix_into(ref channel);
        opcodes.mix_into(ref channel);
        verify_instruction.mix_into(ref channel);
        blake_context.mix_into(ref channel);
        builtins.mix_into(ref channel);
        pedersen_context.mix_into(ref channel);
        poseidon_context.mix_into(ref channel);
        memory_address_to_id.mix_into(ref channel);
        memory_id_to_value.mix_into(ref channel);
        range_checks.mix_into(ref channel);
        verify_bitwise_xor_4.mix_into(ref channel);
        verify_bitwise_xor_7.mix_into(ref channel);
        verify_bitwise_xor_8.mix_into(ref channel);
        verify_bitwise_xor_8_b.mix_into(ref channel);
        verify_bitwise_xor_9.mix_into(ref channel);
    }

    fn accumulate_relation_uses(self: @CairoClaim, ref relation_uses: RelationUsesDict) {
        let CairoClaim {
            public_data: _,
            opcodes,
            verify_instruction,
            blake_context,
            builtins,
            pedersen_context,
            poseidon_context,
            memory_address_to_id: _,
            memory_id_to_value,
            range_checks: _,
            verify_bitwise_xor_4: _,
            verify_bitwise_xor_7: _,
            verify_bitwise_xor_8: _,
            verify_bitwise_xor_8_b: _,
            verify_bitwise_xor_9: _,
        } = self;
        // NOTE: The following components do not USE relations:
        // - range_checks
        // - verify_bitwise_xor_*
        // - memory_address_to_id

        opcodes.accumulate_relation_uses(ref relation_uses);
        blake_context.accumulate_relation_uses(ref relation_uses);
        builtins.accumulate_relation_uses(ref relation_uses);
        pedersen_context.accumulate_relation_uses(ref relation_uses);
        poseidon_context.accumulate_relation_uses(ref relation_uses);
        verify_instruction.accumulate_relation_uses(ref relation_uses);
        memory_id_to_value.accumulate_relation_uses(ref relation_uses);
    }
}


#[derive(Drop, Serde)]
pub struct CairoInteractionClaim {
    pub opcodes: OpcodeInteractionClaim,
    pub verify_instruction: components::verify_instruction::InteractionClaim,
    pub blake_context: BlakeContextInteractionClaim,
    pub builtins: BuiltinsInteractionClaim,
    pub pedersen_context: PedersenContextInteractionClaim,
    pub poseidon_context: PoseidonContextInteractionClaim,
    pub memory_address_to_id: components::memory_address_to_id::InteractionClaim,
    pub memory_id_to_value: components::memory_id_to_big::InteractionClaim,
    pub range_checks: RangeChecksInteractionClaim,
    pub verify_bitwise_xor_4: components::verify_bitwise_xor_4::InteractionClaim,
    pub verify_bitwise_xor_7: components::verify_bitwise_xor_7::InteractionClaim,
    pub verify_bitwise_xor_8: components::verify_bitwise_xor_8::InteractionClaim,
    pub verify_bitwise_xor_8_b: components::verify_bitwise_xor_8_b::InteractionClaim,
    pub verify_bitwise_xor_9: components::verify_bitwise_xor_9::InteractionClaim,
}

#[generate_trait]
pub impl CairoInteractionClaimImpl of CairoInteractionClaimTrace {
    fn mix_into(self: @CairoInteractionClaim, ref channel: Channel) {
        let CairoInteractionClaim {
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
            verify_bitwise_xor_8_b,
            verify_bitwise_xor_9,
        } = self;

        opcodes.mix_into(ref channel);
        verify_instruction.mix_into(ref channel);
        blake_context.mix_into(ref channel);
        builtins.mix_into(ref channel);
        pedersen_context.mix_into(ref channel);
        poseidon_context.mix_into(ref channel);
        memory_address_to_id.mix_into(ref channel);
        memory_id_to_value.mix_into(ref channel);
        range_checks.mix_into(ref channel);
        verify_bitwise_xor_4.mix_into(ref channel);
        verify_bitwise_xor_7.mix_into(ref channel);
        verify_bitwise_xor_8.mix_into(ref channel);
        verify_bitwise_xor_8_b.mix_into(ref channel);
        verify_bitwise_xor_9.mix_into(ref channel);
    }
}

#[derive(Drop)]
pub struct CairoInteractionElements {
    pub opcodes: OpcodesElements,
    pub verify_instruction: VerifyInstructionElements,
    pub blake_round: BlakeRoundElements,
    pub blake_g: BlakeGElements,
    pub blake_round_sigma: BlakeRoundSigmaElements,
    pub triple_xor_32: TripleXor32Elements,
    pub poseidon_aggregator: PoseidonAggregatorElements,
    pub poseidon_3_partial_rounds_chain: Poseidon3PartialRoundsChainElements,
    pub poseidon_full_round_chain: PoseidonFullRoundChainElements,
    pub cube_252: Cube252Elements,
    pub poseidon_round_keys: PoseidonRoundKeysElements,
    pub range_check_252_width_27: RangeCheck252Width27Elements,
    pub pedersen_aggregator: PedersenAggregatorElements,
    pub partial_ec_mul: PartialEcMulElements,
    pub pedersen_points_table: PedersenPointsTableElements,
    pub memory_address_to_id: MemoryAddressToIdElements,
    pub memory_id_to_value: MemoryIdToBigElements,
    pub range_checks: RangeChecksInteractionElements,
    pub verify_bitwise_xor_4: VerifyBitwiseXor_4Elements,
    pub verify_bitwise_xor_7: VerifyBitwiseXor_7Elements,
    pub verify_bitwise_xor_8: VerifyBitwiseXor_8Elements,
    pub verify_bitwise_xor_8_b: VerifyBitwiseXor_8_BElements,
    pub verify_bitwise_xor_9: VerifyBitwiseXor_9Elements,
    pub verify_bitwise_xor_12: VerifyBitwiseXor_12Elements,
}

#[generate_trait]
pub impl CairoInteractionElementsImpl of CairoInteractionElementsTrait {
    fn draw(ref channel: Channel) -> CairoInteractionElements {
        CairoInteractionElements {
            opcodes: LookupElementsImpl::draw(ref channel),
            verify_instruction: LookupElementsImpl::draw(ref channel),
            blake_round: LookupElementsImpl::draw(ref channel),
            blake_g: LookupElementsImpl::draw(ref channel),
            blake_round_sigma: LookupElementsImpl::draw(ref channel),
            triple_xor_32: LookupElementsImpl::draw(ref channel),
            poseidon_aggregator: LookupElementsImpl::draw(ref channel),
            poseidon_3_partial_rounds_chain: LookupElementsImpl::draw(ref channel),
            poseidon_full_round_chain: LookupElementsImpl::draw(ref channel),
            cube_252: LookupElementsImpl::draw(ref channel),
            poseidon_round_keys: LookupElementsImpl::draw(ref channel),
            range_check_252_width_27: LookupElementsImpl::draw(ref channel),
            pedersen_aggregator: LookupElementsImpl::draw(ref channel),
            partial_ec_mul: LookupElementsImpl::draw(ref channel),
            pedersen_points_table: LookupElementsImpl::draw(ref channel),
            memory_address_to_id: LookupElementsImpl::draw(ref channel),
            memory_id_to_value: LookupElementsImpl::draw(ref channel),
            range_checks: RangeChecksInteractionElementsImpl::draw(ref channel),
            verify_bitwise_xor_4: LookupElementsImpl::draw(ref channel),
            verify_bitwise_xor_7: LookupElementsImpl::draw(ref channel),
            verify_bitwise_xor_8: LookupElementsImpl::draw(ref channel),
            verify_bitwise_xor_8_b: LookupElementsImpl::draw(ref channel),
            verify_bitwise_xor_9: LookupElementsImpl::draw(ref channel),
            verify_bitwise_xor_12: LookupElementsImpl::draw(ref channel),
        }
    }
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
    verify_bitwise_xor_8_b: components::verify_bitwise_xor_8_b::Component,
    verify_bitwise_xor_9: components::verify_bitwise_xor_9::Component,
    /// The degree bound of the cairo air.
    composition_log_degree_bound: u32,
}

#[generate_trait]
#[cfg(not(feature: "poseidon252_verifier"))]
pub impl CairoAirNewImpl of CairoAirNewTrait {
    fn new(
        cairo_claim: @CairoClaim,
        interaction_elements: @CairoInteractionElements,
        interaction_claim: @CairoInteractionClaim,
        composition_log_degree_bound: u32,
    ) -> CairoAir {
        let opcode_components = OpcodeComponentsImpl::new(
            cairo_claim.opcodes, interaction_elements, interaction_claim.opcodes,
        );

        let blake_context_component = BlakeContextComponentsImpl::new(
            cairo_claim.blake_context, interaction_elements, interaction_claim.blake_context,
        );

        let builtins_components = BuiltinComponentsImpl::new(
            cairo_claim.builtins, interaction_elements, interaction_claim.builtins,
        );

        let pedersen_context_components = PedersenContextComponentsImpl::new(
            cairo_claim.pedersen_context, interaction_elements, interaction_claim.pedersen_context,
        );

        let poseidon_context_components = PoseidonContextComponentsImpl::new(
            cairo_claim.poseidon_context, interaction_elements, interaction_claim.poseidon_context,
        );

        let verifyinstruction_component = components::verify_instruction::NewComponentImpl::new(
            cairo_claim.verify_instruction,
            interaction_claim.verify_instruction,
            interaction_elements,
        );

        let memory_address_to_id_component =
            components::memory_address_to_id::NewComponentImpl::new(
            cairo_claim.memory_address_to_id,
            interaction_claim.memory_address_to_id,
            interaction_elements,
        );

        assert!(
            cairo_claim
                .memory_id_to_value
                .big_log_sizes
                .len() == interaction_claim
                .memory_id_to_value
                .big_claimed_sums
                .len(),
        );
        let mut memory_id_to_value_components = array![];
        let mut offset: u32 = LARGE_MEMORY_VALUE_ID_BASE;
        for i in 0..cairo_claim.memory_id_to_value.big_log_sizes.len() {
            let log_size = *cairo_claim.memory_id_to_value.big_log_sizes[i];
            let claimed_sum = *interaction_claim.memory_id_to_value.big_claimed_sums[i];
            memory_id_to_value_components
                .append(
                    components::memory_id_to_big::NewBigComponentImpl::new(
                        log_size, offset, claimed_sum, interaction_elements,
                    ),
                );
            offset = offset + pow2(log_size);
        }
        // Check that IDs in (ID -> Value) do not overflow P.
        assert!(offset <= P_U32);

        let small_memory_id_to_value_component =
            components::memory_id_to_big::NewSmallComponentImpl::new(
            *cairo_claim.memory_id_to_value.small_log_size,
            *interaction_claim.memory_id_to_value.small_claimed_sum,
            interaction_elements,
        );

        let range_checks_components = RangeChecksComponentsImpl::new(
            cairo_claim.range_checks, interaction_elements, interaction_claim.range_checks,
        );

        let verify_bitwise_xor_4_component =
            components::verify_bitwise_xor_4::NewComponentImpl::new(
            cairo_claim.verify_bitwise_xor_4,
            interaction_claim.verify_bitwise_xor_4,
            interaction_elements,
        );

        let verify_bitwise_xor_7_component =
            components::verify_bitwise_xor_7::NewComponentImpl::new(
            cairo_claim.verify_bitwise_xor_7,
            interaction_claim.verify_bitwise_xor_7,
            interaction_elements,
        );

        let verify_bitwise_xor_8_component =
            components::verify_bitwise_xor_8::NewComponentImpl::new(
            cairo_claim.verify_bitwise_xor_8,
            interaction_claim.verify_bitwise_xor_8,
            interaction_elements,
        );

        let verify_bitwise_xor_8_b_component =
            components::verify_bitwise_xor_8_b::NewComponentImpl::new(
            cairo_claim.verify_bitwise_xor_8_b,
            interaction_claim.verify_bitwise_xor_8_b,
            interaction_elements,
        );

        let verify_bitwise_xor_9_component =
            components::verify_bitwise_xor_9::NewComponentImpl::new(
            cairo_claim.verify_bitwise_xor_9,
            interaction_claim.verify_bitwise_xor_9,
            interaction_elements,
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
            verify_bitwise_xor_8_b: verify_bitwise_xor_8_b_component,
            verify_bitwise_xor_9: verify_bitwise_xor_9_component,
            composition_log_degree_bound,
        }
    }
}

#[cfg(not(feature: "poseidon252_verifier"))]
pub impl CairoAirImpl of Air<CairoAir> {
    fn composition_log_degree_bound(self: @CairoAir) -> u32 {
        *self.composition_log_degree_bound
    }

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
            verify_bitwise_xor_8_b,
            verify_bitwise_xor_9,
            composition_log_degree_bound: _,
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
        verify_bitwise_xor_8_b
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
    verify_bitwise_xor_8_b: components::verify_bitwise_xor_8_b::Component,
    verify_bitwise_xor_9: components::verify_bitwise_xor_9::Component,
    /// The degree bound of the cairo air.
    composition_log_degree_bound: u32,
}

#[generate_trait]
#[cfg(and(feature: "poseidon252_verifier", not(feature: "poseidon_outputs_packing")))]
pub impl CairoAirNewImpl of CairoAirNewTrait {
    fn new(
        cairo_claim: @CairoClaim,
        interaction_elements: @CairoInteractionElements,
        interaction_claim: @CairoInteractionClaim,
        composition_log_degree_bound: u32,
    ) -> CairoAir {
        let opcode_components = OpcodeComponentsImpl::new(
            cairo_claim.opcodes, interaction_elements, interaction_claim.opcodes,
        );

        let blake_context_component = BlakeContextComponentsImpl::new(
            cairo_claim.blake_context, interaction_elements, interaction_claim.blake_context,
        );

        let builtins_components = BuiltinComponentsImpl::new(
            cairo_claim.builtins, interaction_elements, interaction_claim.builtins,
        );

        let verifyinstruction_component = components::verify_instruction::NewComponentImpl::new(
            cairo_claim.verify_instruction,
            interaction_claim.verify_instruction,
            interaction_elements,
        );

        let memory_address_to_id_component =
            components::memory_address_to_id::NewComponentImpl::new(
            cairo_claim.memory_address_to_id,
            interaction_claim.memory_address_to_id,
            interaction_elements,
        );

        assert!(
            cairo_claim
                .memory_id_to_value
                .big_log_sizes
                .len() == interaction_claim
                .memory_id_to_value
                .big_claimed_sums
                .len(),
        );
        let mut memory_id_to_value_components = array![];
        let mut offset: u32 = LARGE_MEMORY_VALUE_ID_BASE;
        for i in 0..cairo_claim.memory_id_to_value.big_log_sizes.len() {
            let log_size = *cairo_claim.memory_id_to_value.big_log_sizes[i];
            let claimed_sum = *interaction_claim.memory_id_to_value.big_claimed_sums[i];
            memory_id_to_value_components
                .append(
                    components::memory_id_to_big::NewBigComponentImpl::new(
                        log_size, offset, claimed_sum, interaction_elements,
                    ),
                );
            offset = offset + pow2(log_size);
        }
        // Check that IDs in (ID -> Value) do not overflow P.
        assert!(offset <= P_U32);

        let small_memory_id_to_value_component =
            components::memory_id_to_big::NewSmallComponentImpl::new(
            *cairo_claim.memory_id_to_value.small_log_size,
            *interaction_claim.memory_id_to_value.small_claimed_sum,
            interaction_elements,
        );

        let range_checks_components = RangeChecksComponentsImpl::new(
            cairo_claim.range_checks, interaction_elements, interaction_claim.range_checks,
        );

        let verify_bitwise_xor_4_component =
            components::verify_bitwise_xor_4::NewComponentImpl::new(
            cairo_claim.verify_bitwise_xor_4,
            interaction_claim.verify_bitwise_xor_4,
            interaction_elements,
        );

        let verify_bitwise_xor_7_component =
            components::verify_bitwise_xor_7::NewComponentImpl::new(
            cairo_claim.verify_bitwise_xor_7,
            interaction_claim.verify_bitwise_xor_7,
            interaction_elements,
        );

        let verify_bitwise_xor_8_component =
            components::verify_bitwise_xor_8::NewComponentImpl::new(
            cairo_claim.verify_bitwise_xor_8,
            interaction_claim.verify_bitwise_xor_8,
            interaction_elements,
        );

        let verify_bitwise_xor_8_b_component =
            components::verify_bitwise_xor_8_b::NewComponentImpl::new(
            cairo_claim.verify_bitwise_xor_8_b,
            interaction_claim.verify_bitwise_xor_8_b,
            interaction_elements,
        );

        let verify_bitwise_xor_9_component =
            components::verify_bitwise_xor_9::NewComponentImpl::new(
            cairo_claim.verify_bitwise_xor_9,
            interaction_claim.verify_bitwise_xor_9,
            interaction_elements,
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
            verify_bitwise_xor_8_b: verify_bitwise_xor_8_b_component,
            verify_bitwise_xor_9: verify_bitwise_xor_9_component,
            composition_log_degree_bound,
        }
    }
}

#[cfg(and(feature: "poseidon252_verifier", not(feature: "poseidon_outputs_packing")))]
pub impl CairoAirImpl of Air<CairoAir> {
    fn composition_log_degree_bound(self: @CairoAir) -> u32 {
        *self.composition_log_degree_bound
    }

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
            verify_bitwise_xor_8_b,
            verify_bitwise_xor_9,
            composition_log_degree_bound: _,
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
        verify_bitwise_xor_8_b
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
    verify_bitwise_xor_8_b: components::verify_bitwise_xor_8_b::Component,
    verify_bitwise_xor_9: components::verify_bitwise_xor_9::Component,
    /// The degree bound of the cairo air.
    composition_log_degree_bound: u32,
}

#[generate_trait]
#[cfg(and(feature: "poseidon252_verifier", feature: "poseidon_outputs_packing"))]
pub impl CairoAirNewImpl of CairoAirNewTrait {
    fn new(
        cairo_claim: @CairoClaim,
        interaction_elements: @CairoInteractionElements,
        interaction_claim: @CairoInteractionClaim,
        composition_log_degree_bound: u32,
    ) -> CairoAir {
        let opcode_components = OpcodeComponentsImpl::new(
            cairo_claim.opcodes, interaction_elements, interaction_claim.opcodes,
        );

        let blake_context_component = BlakeContextComponentsImpl::new(
            cairo_claim.blake_context, interaction_elements, interaction_claim.blake_context,
        );

        let builtins_components = BuiltinComponentsImpl::new(
            cairo_claim.builtins, interaction_elements, interaction_claim.builtins,
        );

        let poseidon_context_components = PoseidonContextComponentsImpl::new(
            cairo_claim.poseidon_context, interaction_elements, interaction_claim.poseidon_context,
        );

        let verifyinstruction_component = components::verify_instruction::NewComponentImpl::new(
            cairo_claim.verify_instruction,
            interaction_claim.verify_instruction,
            interaction_elements,
        );

        let memory_address_to_id_component =
            components::memory_address_to_id::NewComponentImpl::new(
            cairo_claim.memory_address_to_id,
            interaction_claim.memory_address_to_id,
            interaction_elements,
        );

        assert!(
            cairo_claim
                .memory_id_to_value
                .big_log_sizes
                .len() == interaction_claim
                .memory_id_to_value
                .big_claimed_sums
                .len(),
        );
        let mut memory_id_to_value_components = array![];
        let mut offset: u32 = LARGE_MEMORY_VALUE_ID_BASE;
        for i in 0..cairo_claim.memory_id_to_value.big_log_sizes.len() {
            let log_size = *cairo_claim.memory_id_to_value.big_log_sizes[i];
            let claimed_sum = *interaction_claim.memory_id_to_value.big_claimed_sums[i];
            memory_id_to_value_components
                .append(
                    components::memory_id_to_big::NewBigComponentImpl::new(
                        log_size, offset, claimed_sum, interaction_elements,
                    ),
                );
            offset = offset + pow2(log_size);
        }
        // Check that IDs in (ID -> Value) do not overflow P.
        assert!(offset <= P_U32);

        let small_memory_id_to_value_component =
            components::memory_id_to_big::NewSmallComponentImpl::new(
            *cairo_claim.memory_id_to_value.small_log_size,
            *interaction_claim.memory_id_to_value.small_claimed_sum,
            interaction_elements,
        );

        let range_checks_components = RangeChecksComponentsImpl::new(
            cairo_claim.range_checks, interaction_elements, interaction_claim.range_checks,
        );

        let verify_bitwise_xor_4_component =
            components::verify_bitwise_xor_4::NewComponentImpl::new(
            cairo_claim.verify_bitwise_xor_4,
            interaction_claim.verify_bitwise_xor_4,
            interaction_elements,
        );

        let verify_bitwise_xor_7_component =
            components::verify_bitwise_xor_7::NewComponentImpl::new(
            cairo_claim.verify_bitwise_xor_7,
            interaction_claim.verify_bitwise_xor_7,
            interaction_elements,
        );

        let verify_bitwise_xor_8_component =
            components::verify_bitwise_xor_8::NewComponentImpl::new(
            cairo_claim.verify_bitwise_xor_8,
            interaction_claim.verify_bitwise_xor_8,
            interaction_elements,
        );

        let verify_bitwise_xor_8_b_component =
            components::verify_bitwise_xor_8_b::NewComponentImpl::new(
            cairo_claim.verify_bitwise_xor_8_b,
            interaction_claim.verify_bitwise_xor_8_b,
            interaction_elements,
        );

        let verify_bitwise_xor_9_component =
            components::verify_bitwise_xor_9::NewComponentImpl::new(
            cairo_claim.verify_bitwise_xor_9,
            interaction_claim.verify_bitwise_xor_9,
            interaction_elements,
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
            verify_bitwise_xor_8_b: verify_bitwise_xor_8_b_component,
            verify_bitwise_xor_9: verify_bitwise_xor_9_component,
            composition_log_degree_bound,
        }
    }
}

#[cfg(and(feature: "poseidon252_verifier", feature: "poseidon_outputs_packing"))]
pub impl CairoAirImpl of Air<CairoAir> {
    fn composition_log_degree_bound(self: @CairoAir) -> u32 {
        *self.composition_log_degree_bound
    }

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
            verify_bitwise_xor_8_b,
            verify_bitwise_xor_9,
            composition_log_degree_bound: _,
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
        verify_bitwise_xor_8_b
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
