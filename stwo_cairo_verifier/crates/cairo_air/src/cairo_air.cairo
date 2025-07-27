use components::memory_address_to_id::{
    ClaimImpl as MemoryAddressToIdClaimImpl,
    InteractionClaimImpl as MemoryAddressToIdInteractionClaimImpl,
};
use components::memory_id_to_big::{
    ClaimImpl as MemoryIdToBigClaimImpl, InteractionClaimImpl as MemoryIdToBigInteractionClaimImpl,
};
use components::triple_xor_32::{
    ClaimImpl as TripleXor32ClaimImpl, InteractionClaimImpl as TripleXor32InteractionClaimImpl,
};
use components::verify_bitwise_xor_12::{
    ClaimImpl as VerifyBitwiseXor12ClaimImpl,
    InteractionClaimImpl as VerifyBitwiseXor12InteractionClaimImpl,
};
use components::verify_bitwise_xor_4::{
    ClaimImpl as VerifyBitwiseXor4ClaimImpl,
    InteractionClaimImpl as VerifyBitwiseXor4InteractionClaimImpl,
};
use components::verify_bitwise_xor_7::{
    ClaimImpl as VerifyBitwiseXor7ClaimImpl,
    InteractionClaimImpl as VerifyBitwiseXor7InteractionClaimImpl,
};
use components::verify_bitwise_xor_8::{
    ClaimImpl as VerifyBitwiseXor8ClaimImpl,
    InteractionClaimImpl as VerifyBitwiseXor8InteractionClaimImpl,
};
use components::verify_bitwise_xor_9::{
    ClaimImpl as VerifyBitwiseXor9ClaimImpl,
    InteractionClaimImpl as VerifyBitwiseXor9InteractionClaimImpl,
};
use components::verify_instruction::{
    ClaimImpl as VerifyInstructionClaimImpl,
    InteractionClaimImpl as VerifyInstructionInteractionClaimImpl,
};
use core::box::BoxImpl;
use core::num::traits::Zero;
use stwo_cairo_air::blake::*;
use stwo_cairo_air::builtins::*;
use stwo_cairo_air::cairo_component::CairoComponent;
use stwo_cairo_air::opcodes::*;
use stwo_cairo_air::pedersen::*;
use stwo_cairo_air::poseidon::*;
use stwo_cairo_air::preprocessed_columns::PREPROCESSED_COLUMNS;
use stwo_cairo_air::range_checks::*;
use stwo_cairo_air::{
    BlakeContextClaim, BlakeContextInteractionClaim, BuiltinsClaim, BuiltinsInteractionClaim,
    OpcodeClaim, OpcodeInteractionClaim, PedersenContextClaim, PedersenContextInteractionClaim,
    PoseidonContextClaim, PoseidonContextInteractionClaim, PublicData, PublicDataImpl,
    RangeChecksClaim, RangeChecksInteractionClaim, RelationUsesDict, accumulate_relation_uses,
    components, utils,
};
use stwo_constraint_framework::{
    LookupElements, LookupElementsImpl, PreprocessedColumnImpl, PreprocessedColumnKey,
    PreprocessedColumnSet, PreprocessedMaskValues, PreprocessedMaskValuesImpl,
};
use stwo_verifier_core::channel::{Channel, ChannelImpl};
use stwo_verifier_core::circle::CirclePoint;
use stwo_verifier_core::fields::qm31::QM31;
use stwo_verifier_core::pcs::verifier::CommitmentSchemeVerifierImpl;
use stwo_verifier_core::utils::{ArrayImpl, OptionImpl, pow2};
use stwo_verifier_core::verifier::Air;
use stwo_verifier_core::{ColumnArray, ColumnSpan, TreeArray, TreeSpan};


pub type Cube252Elements = LookupElements<20>;

pub type MemoryAddressToIdElements = LookupElements<2>;

pub type MemoryIdToBigElements = LookupElements<29>;

pub type OpcodesElements = LookupElements<3>;

pub type PartialEcMulElements = LookupElements<73>;

pub type PedersenPointsTableElements = LookupElements<57>;

pub type PoseidonFullRoundChainElements = LookupElements<32>;

pub type Poseidon3PartialRoundsChainElements = LookupElements<42>;

pub type PoseidonRoundKeysElements = LookupElements<31>;

pub type BlakeGElements = LookupElements<20>;

pub type BlakeRoundElements = LookupElements<35>;

pub type BlakeRoundSigmaElements = LookupElements<17>;

pub type TripleXor32Elements = LookupElements<8>;

pub type RangeCheckFelt252Width27Elements = LookupElements<10>;

pub type VerifyInstructionElements = LookupElements<7>;

pub type VerifyBitwiseXor_4Elements = LookupElements<3>;

pub type VerifyBitwiseXor_7Elements = LookupElements<3>;

pub type VerifyBitwiseXor_8Elements = LookupElements<3>;

pub type VerifyBitwiseXor_9Elements = LookupElements<3>;

pub type VerifyBitwiseXor_12Elements = LookupElements<3>;


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
    pub verify_bitwise_xor_9: components::verify_bitwise_xor_9::Claim,
    // ...
}

#[generate_trait]
pub impl CairoClaimImpl of CairoClaimTrait {
    fn log_sizes(self: @CairoClaim) -> TreeArray<Span<u32>> {
        let mut aggregated_log_sizes = utils::tree_array_concat_cols(
            array![
                self.opcodes.log_sizes(), self.verify_instruction.log_sizes(),
                self.blake_context.log_sizes(), self.builtins.log_sizes(),
                self.pedersen_context.log_sizes(), self.poseidon_context.log_sizes(),
                self.memory_address_to_id.log_sizes(), self.memory_id_to_value.log_sizes(),
                self.range_checks.log_sizes(), self.verify_bitwise_xor_4.log_sizes(),
                self.verify_bitwise_xor_7.log_sizes(), self.verify_bitwise_xor_8.log_sizes(),
                self.verify_bitwise_xor_9.log_sizes(),
            ],
        );

        // Overwrite the preprocessed trace log sizes.
        let _invalid_preprocessed_trace_log_sizes = aggregated_log_sizes.pop_front();

        let mut preprocessed_trace_log_sizes = array![];

        for preprocessed_column in PREPROCESSED_COLUMNS.span() {
            preprocessed_trace_log_sizes.append(preprocessed_column.log_size());
        }

        let trace_log_sizes = aggregated_log_sizes.pop_front().unwrap();
        let interaction_log_sizes = aggregated_log_sizes.pop_front().unwrap();
        assert!(aggregated_log_sizes.is_empty());

        array![preprocessed_trace_log_sizes.span(), trace_log_sizes, interaction_log_sizes]
    }

    fn mix_into(self: @CairoClaim, ref channel: Channel) {
        self.public_data.mix_into(ref channel);
        self.opcodes.mix_into(ref channel);
        self.verify_instruction.mix_into(ref channel);
        self.blake_context.mix_into(ref channel);
        self.builtins.mix_into(ref channel);
        self.pedersen_context.mix_into(ref channel);
        self.poseidon_context.mix_into(ref channel);
        self.memory_address_to_id.mix_into(ref channel);
        self.memory_id_to_value.mix_into(ref channel);
        self.range_checks.mix_into(ref channel);
        self.verify_bitwise_xor_4.mix_into(ref channel);
        self.verify_bitwise_xor_7.mix_into(ref channel);
        self.verify_bitwise_xor_8.mix_into(ref channel);
        self.verify_bitwise_xor_9.mix_into(ref channel);
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

        accumulate_relation_uses(
            ref relation_uses,
            components::verify_instruction::RELATION_USES_PER_ROW.span(),
            *verify_instruction.log_size,
        );
        for log_size in memory_id_to_value.big_log_sizes.span() {
            accumulate_relation_uses(
                ref relation_uses,
                components::memory_id_to_big::RELATION_USES_PER_ROW_BIG.span(),
                *log_size,
            );
        }
        accumulate_relation_uses(
            ref relation_uses,
            components::memory_id_to_big::RELATION_USES_PER_ROW_SMALL.span(),
            *memory_id_to_value.small_log_size,
        );
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
    pub verify_bitwise_xor_9: components::verify_bitwise_xor_9::InteractionClaim,
}

#[generate_trait]
pub impl CairoInteractionClaimImpl of CairoInteractionClaimTrace {
    fn mix_into(self: @CairoInteractionClaim, ref channel: Channel) {
        self.opcodes.mix_into(ref channel);
        self.verify_instruction.mix_into(ref channel);
        self.blake_context.mix_into(ref channel);
        self.builtins.mix_into(ref channel);
        self.pedersen_context.mix_into(ref channel);
        self.poseidon_context.mix_into(ref channel);
        self.memory_address_to_id.mix_into(ref channel);
        self.memory_id_to_value.mix_into(ref channel);
        self.range_checks.mix_into(ref channel);
        self.verify_bitwise_xor_4.mix_into(ref channel);
        self.verify_bitwise_xor_7.mix_into(ref channel);
        self.verify_bitwise_xor_8.mix_into(ref channel);
        self.verify_bitwise_xor_9.mix_into(ref channel);
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
    pub partial_ec_mul: PartialEcMulElements,
    pub pedersen_points_table: PedersenPointsTableElements,
    pub poseidon_full_round_chain: PoseidonFullRoundChainElements,
    pub poseidon_3_partial_rounds_chain: Poseidon3PartialRoundsChainElements,
    pub cube_252: Cube252Elements,
    pub poseidon_round_keys: PoseidonRoundKeysElements,
    pub range_check_felt_252_width_27: RangeCheckFelt252Width27Elements,
    pub memory_address_to_id: MemoryAddressToIdElements,
    pub memory_id_to_value: MemoryIdToBigElements,
    pub range_checks: RangeChecksInteractionElements,
    pub verify_bitwise_xor_4: VerifyBitwiseXor_4Elements,
    pub verify_bitwise_xor_7: VerifyBitwiseXor_7Elements,
    pub verify_bitwise_xor_8: VerifyBitwiseXor_8Elements,
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
            poseidon_3_partial_rounds_chain: LookupElementsImpl::draw(ref channel),
            poseidon_full_round_chain: LookupElementsImpl::draw(ref channel),
            cube_252: LookupElementsImpl::draw(ref channel),
            poseidon_round_keys: LookupElementsImpl::draw(ref channel),
            range_check_felt_252_width_27: LookupElementsImpl::draw(ref channel),
            partial_ec_mul: LookupElementsImpl::draw(ref channel),
            pedersen_points_table: LookupElementsImpl::draw(ref channel),
            memory_address_to_id: LookupElementsImpl::draw(ref channel),
            memory_id_to_value: LookupElementsImpl::draw(ref channel),
            range_checks: RangeChecksInteractionElementsImpl::draw(ref channel),
            verify_bitwise_xor_4: LookupElementsImpl::draw(ref channel),
            verify_bitwise_xor_7: LookupElementsImpl::draw(ref channel),
            verify_bitwise_xor_8: LookupElementsImpl::draw(ref channel),
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
    verify_bitwise_xor_9: components::verify_bitwise_xor_9::Component,
}

#[generate_trait]
#[cfg(not(feature: "poseidon252_verifier"))]
pub impl CairoAirNewImpl of CairoAirNewTrait {
    fn new(
        cairo_claim: @CairoClaim,
        interaction_elements: @CairoInteractionElements,
        interaction_claim: @CairoInteractionClaim,
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

        let verifyinstruction_component = components::verify_instruction::Component {
            claim: *cairo_claim.verify_instruction,
            interaction_claim: *interaction_claim.verify_instruction,
            memory_address_to_id_lookup_elements: interaction_elements.memory_address_to_id.clone(),
            memory_id_to_big_lookup_elements: interaction_elements.memory_id_to_value.clone(),
            range_check_4_3_lookup_elements: interaction_elements.range_checks.rc_4_3.clone(),
            range_check_7_2_5_lookup_elements: interaction_elements.range_checks.rc_7_2_5.clone(),
            verify_instruction_lookup_elements: interaction_elements.verify_instruction.clone(),
        };

        let memory_address_to_id_component = components::memory_address_to_id::Component {
            claim: *cairo_claim.memory_address_to_id,
            interaction_claim: *interaction_claim.memory_address_to_id,
            lookup_elements: interaction_elements.memory_address_to_id.clone(),
        };

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
        let mut offset = 0;
        for i in 0..cairo_claim.memory_id_to_value.big_log_sizes.len() {
            let log_size = *cairo_claim.memory_id_to_value.big_log_sizes[i];
            let claimed_sum = *interaction_claim.memory_id_to_value.big_claimed_sums[i];
            memory_id_to_value_components
                .append(
                    components::memory_id_to_big::BigComponent {
                        log_n_rows: log_size,
                        offset: offset,
                        claimed_sum: claimed_sum,
                        lookup_elements: interaction_elements.memory_id_to_value.clone(),
                        range_9_9_lookup_elements: interaction_elements.range_checks.rc_9_9.clone(),
                        range_9_9_b_lookup_elements: interaction_elements
                            .range_checks
                            .rc_9_9_b
                            .clone(),
                        range_9_9_c_lookup_elements: interaction_elements
                            .range_checks
                            .rc_9_9_c
                            .clone(),
                        range_9_9_d_lookup_elements: interaction_elements
                            .range_checks
                            .rc_9_9_d
                            .clone(),
                        range_9_9_e_lookup_elements: interaction_elements
                            .range_checks
                            .rc_9_9_e
                            .clone(),
                        range_9_9_f_lookup_elements: interaction_elements
                            .range_checks
                            .rc_9_9_f
                            .clone(),
                        range_9_9_g_lookup_elements: interaction_elements
                            .range_checks
                            .rc_9_9_g
                            .clone(),
                        range_9_9_h_lookup_elements: interaction_elements
                            .range_checks
                            .rc_9_9_h
                            .clone(),
                    },
                );
            offset = offset + pow2(log_size);
        }

        let small_memory_id_to_value_component = components::memory_id_to_big::SmallComponent {
            log_n_rows: *cairo_claim.memory_id_to_value.small_log_size,
            claimed_sum: *interaction_claim.memory_id_to_value.small_claimed_sum,
            lookup_elements: interaction_elements.memory_id_to_value.clone(),
            range_9_9_lookup_elements: interaction_elements.range_checks.rc_9_9.clone(),
            range_9_9_b_lookup_elements: interaction_elements.range_checks.rc_9_9_b.clone(),
            range_9_9_c_lookup_elements: interaction_elements.range_checks.rc_9_9_c.clone(),
            range_9_9_d_lookup_elements: interaction_elements.range_checks.rc_9_9_d.clone(),
        };

        let range_checks_components = RangeChecksComponentsImpl::new(
            cairo_claim.range_checks, interaction_elements, interaction_claim.range_checks,
        );

        let verify_bitwise_xor_4_component = components::verify_bitwise_xor_4::Component {
            claim: *cairo_claim.verify_bitwise_xor_4,
            interaction_claim: *interaction_claim.verify_bitwise_xor_4,
            verify_bitwise_xor_4_lookup_elements: interaction_elements.verify_bitwise_xor_4.clone(),
        };

        let verify_bitwise_xor_7_component = components::verify_bitwise_xor_7::Component {
            claim: *cairo_claim.verify_bitwise_xor_7,
            interaction_claim: *interaction_claim.verify_bitwise_xor_7,
            verify_bitwise_xor_7_lookup_elements: interaction_elements.verify_bitwise_xor_7.clone(),
        };

        let verify_bitwise_xor_8_component = components::verify_bitwise_xor_8::Component {
            claim: *cairo_claim.verify_bitwise_xor_8,
            interaction_claim: *interaction_claim.verify_bitwise_xor_8,
            verify_bitwise_xor_8_lookup_elements: interaction_elements.verify_bitwise_xor_8.clone(),
        };

        let verify_bitwise_xor_9_component = components::verify_bitwise_xor_9::Component {
            claim: *cairo_claim.verify_bitwise_xor_9,
            interaction_claim: *interaction_claim.verify_bitwise_xor_9,
            verify_bitwise_xor_9_lookup_elements: interaction_elements.verify_bitwise_xor_9.clone(),
        };

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
    fn composition_log_degree_bound(self: @CairoAir) -> u32 {
        let mut max_degree = self.opcodes.max_constraint_log_degree_bound();
        max_degree =
            core::cmp::max(max_degree, self.verify_instruction.max_constraint_log_degree_bound());
        max_degree =
            core::cmp::max(max_degree, self.blake_context.max_constraint_log_degree_bound());
        max_degree = core::cmp::max(max_degree, self.builtins.max_constraint_log_degree_bound());
        max_degree =
            core::cmp::max(max_degree, self.pedersen_context.max_constraint_log_degree_bound());
        max_degree =
            core::cmp::max(max_degree, self.poseidon_context.max_constraint_log_degree_bound());
        max_degree =
            core::cmp::max(max_degree, self.memory_address_to_id.max_constraint_log_degree_bound());
        let (memory_id_to_value_big, memory_id_to_value_small) = self.memory_id_to_value;
        for memory_id_to_value_big_component in memory_id_to_value_big.span() {
            max_degree =
                core::cmp::max(
                    max_degree, memory_id_to_value_big_component.max_constraint_log_degree_bound(),
                );
        }
        max_degree =
            core::cmp::max(max_degree, memory_id_to_value_small.max_constraint_log_degree_bound());
        max_degree =
            core::cmp::max(max_degree, self.range_checks.max_constraint_log_degree_bound());
        max_degree =
            core::cmp::max(max_degree, self.verify_bitwise_xor_4.max_constraint_log_degree_bound());
        max_degree =
            core::cmp::max(max_degree, self.verify_bitwise_xor_7.max_constraint_log_degree_bound());
        max_degree =
            core::cmp::max(max_degree, self.verify_bitwise_xor_8.max_constraint_log_degree_bound());
        max_degree =
            core::cmp::max(max_degree, self.verify_bitwise_xor_9.max_constraint_log_degree_bound());
        max_degree
    }

    fn mask_points(
        self: @CairoAir, point: CirclePoint<QM31>,
    ) -> TreeArray<ColumnArray<Array<CirclePoint<QM31>>>> {
        let mut preprocessed_column_set: PreprocessedColumnSet = Default::default();
        let mut trace_mask_points = array![];
        let mut interaction_trace_mask_points = array![];
        self
            .opcodes
            .mask_points(
                ref preprocessed_column_set,
                ref trace_mask_points,
                ref interaction_trace_mask_points,
                point,
            );
        self
            .verify_instruction
            .mask_points(
                ref preprocessed_column_set,
                ref trace_mask_points,
                ref interaction_trace_mask_points,
                point,
            );
        self
            .blake_context
            .mask_points(
                ref preprocessed_column_set,
                ref trace_mask_points,
                ref interaction_trace_mask_points,
                point,
            );
        self
            .builtins
            .mask_points(
                ref preprocessed_column_set,
                ref trace_mask_points,
                ref interaction_trace_mask_points,
                point,
            );
        self
            .pedersen_context
            .mask_points(
                ref preprocessed_column_set,
                ref trace_mask_points,
                ref interaction_trace_mask_points,
                point,
            );
        self
            .poseidon_context
            .mask_points(
                ref preprocessed_column_set,
                ref trace_mask_points,
                ref interaction_trace_mask_points,
                point,
            );
        self
            .memory_address_to_id
            .mask_points(
                ref preprocessed_column_set,
                ref trace_mask_points,
                ref interaction_trace_mask_points,
                point,
            );

        let (memory_id_to_value_big, memory_id_to_value_small) = self.memory_id_to_value;
        for memory_id_to_value_big_component in memory_id_to_value_big.span() {
            memory_id_to_value_big_component
                .mask_points(
                    ref preprocessed_column_set,
                    ref trace_mask_points,
                    ref interaction_trace_mask_points,
                    point,
                );
        }
        memory_id_to_value_small
            .mask_points(
                ref preprocessed_column_set,
                ref trace_mask_points,
                ref interaction_trace_mask_points,
                point,
            );
        self
            .range_checks
            .mask_points(
                ref preprocessed_column_set,
                ref trace_mask_points,
                ref interaction_trace_mask_points,
                point,
            );
        self
            .verify_bitwise_xor_4
            .mask_points(
                ref preprocessed_column_set,
                ref trace_mask_points,
                ref interaction_trace_mask_points,
                point,
            );
        self
            .verify_bitwise_xor_7
            .mask_points(
                ref preprocessed_column_set,
                ref trace_mask_points,
                ref interaction_trace_mask_points,
                point,
            );
        self
            .verify_bitwise_xor_8
            .mask_points(
                ref preprocessed_column_set,
                ref trace_mask_points,
                ref interaction_trace_mask_points,
                point,
            );
        self
            .verify_bitwise_xor_9
            .mask_points(
                ref preprocessed_column_set,
                ref trace_mask_points,
                ref interaction_trace_mask_points,
                point,
            );

        let preprocessed_trace_mask_points = preprocessed_trace_mask_points(
            preprocessed_column_set, point,
        );

        array![preprocessed_trace_mask_points, trace_mask_points, interaction_trace_mask_points]
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
            preprocessed_mask_values, PREPROCESSED_COLUMNS.span(),
        );

        self
            .opcodes
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        self
            .verify_instruction
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        self
            .blake_context
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        self
            .builtins
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        self
            .pedersen_context
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        self
            .poseidon_context
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        self
            .memory_address_to_id
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        let (memory_id_to_value_big, memory_id_to_value_small) = self.memory_id_to_value;
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
        self
            .range_checks
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        self
            .verify_bitwise_xor_4
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        self
            .verify_bitwise_xor_7
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        self
            .verify_bitwise_xor_8
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        self
            .verify_bitwise_xor_9
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        sum
    }
}

#[derive(Drop)]
#[cfg(feature: "poseidon252_verifier")]
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
#[cfg(feature: "poseidon252_verifier")]
pub impl CairoAirNewImpl of CairoAirNewTrait {
    fn new(
        cairo_claim: @CairoClaim,
        interaction_elements: @CairoInteractionElements,
        interaction_claim: @CairoInteractionClaim,
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

        let verifyinstruction_component = components::verify_instruction::Component {
            claim: *cairo_claim.verify_instruction,
            interaction_claim: *interaction_claim.verify_instruction,
            memory_address_to_id_lookup_elements: interaction_elements.memory_address_to_id.clone(),
            memory_id_to_big_lookup_elements: interaction_elements.memory_id_to_value.clone(),
            range_check_4_3_lookup_elements: interaction_elements.range_checks.rc_4_3.clone(),
            range_check_7_2_5_lookup_elements: interaction_elements.range_checks.rc_7_2_5.clone(),
            verify_instruction_lookup_elements: interaction_elements.verify_instruction.clone(),
        };

        let memory_address_to_id_component = components::memory_address_to_id::Component {
            claim: *cairo_claim.memory_address_to_id,
            interaction_claim: *interaction_claim.memory_address_to_id,
            lookup_elements: interaction_elements.memory_address_to_id.clone(),
        };

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
        let mut offset = 0;
        for i in 0..cairo_claim.memory_id_to_value.big_log_sizes.len() {
            let log_size = *cairo_claim.memory_id_to_value.big_log_sizes[i];
            let claimed_sum = *interaction_claim.memory_id_to_value.big_claimed_sums[i];
            memory_id_to_value_components
                .append(
                    components::memory_id_to_big::BigComponent {
                        log_n_rows: log_size,
                        offset: offset,
                        claimed_sum: claimed_sum,
                        lookup_elements: interaction_elements.memory_id_to_value.clone(),
                        range_9_9_lookup_elements: interaction_elements.range_checks.rc_9_9.clone(),
                        range_9_9_b_lookup_elements: interaction_elements
                            .range_checks
                            .rc_9_9_b
                            .clone(),
                        range_9_9_c_lookup_elements: interaction_elements
                            .range_checks
                            .rc_9_9_c
                            .clone(),
                        range_9_9_d_lookup_elements: interaction_elements
                            .range_checks
                            .rc_9_9_d
                            .clone(),
                        range_9_9_e_lookup_elements: interaction_elements
                            .range_checks
                            .rc_9_9_e
                            .clone(),
                        range_9_9_f_lookup_elements: interaction_elements
                            .range_checks
                            .rc_9_9_f
                            .clone(),
                        range_9_9_g_lookup_elements: interaction_elements
                            .range_checks
                            .rc_9_9_g
                            .clone(),
                        range_9_9_h_lookup_elements: interaction_elements
                            .range_checks
                            .rc_9_9_h
                            .clone(),
                    },
                );
            offset = offset + pow2(log_size);
        }

        let small_memory_id_to_value_component = components::memory_id_to_big::SmallComponent {
            log_n_rows: *cairo_claim.memory_id_to_value.small_log_size,
            claimed_sum: *interaction_claim.memory_id_to_value.small_claimed_sum,
            lookup_elements: interaction_elements.memory_id_to_value.clone(),
            range_9_9_lookup_elements: interaction_elements.range_checks.rc_9_9.clone(),
            range_9_9_b_lookup_elements: interaction_elements.range_checks.rc_9_9_b.clone(),
            range_9_9_c_lookup_elements: interaction_elements.range_checks.rc_9_9_c.clone(),
            range_9_9_d_lookup_elements: interaction_elements.range_checks.rc_9_9_d.clone(),
        };

        let range_checks_components = RangeChecksComponentsImpl::new(
            cairo_claim.range_checks, interaction_elements, interaction_claim.range_checks,
        );

        let verify_bitwise_xor_4_component = components::verify_bitwise_xor_4::Component {
            claim: *cairo_claim.verify_bitwise_xor_4,
            interaction_claim: *interaction_claim.verify_bitwise_xor_4,
            verify_bitwise_xor_4_lookup_elements: interaction_elements.verify_bitwise_xor_4.clone(),
        };

        let verify_bitwise_xor_7_component = components::verify_bitwise_xor_7::Component {
            claim: *cairo_claim.verify_bitwise_xor_7,
            interaction_claim: *interaction_claim.verify_bitwise_xor_7,
            verify_bitwise_xor_7_lookup_elements: interaction_elements.verify_bitwise_xor_7.clone(),
        };

        let verify_bitwise_xor_8_component = components::verify_bitwise_xor_8::Component {
            claim: *cairo_claim.verify_bitwise_xor_8,
            interaction_claim: *interaction_claim.verify_bitwise_xor_8,
            verify_bitwise_xor_8_lookup_elements: interaction_elements.verify_bitwise_xor_8.clone(),
        };

        let verify_bitwise_xor_9_component = components::verify_bitwise_xor_9::Component {
            claim: *cairo_claim.verify_bitwise_xor_9,
            interaction_claim: *interaction_claim.verify_bitwise_xor_9,
            verify_bitwise_xor_9_lookup_elements: interaction_elements.verify_bitwise_xor_9.clone(),
        };

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

#[cfg(feature: "poseidon252_verifier")]
pub impl CairoAirImpl of Air<CairoAir> {
    fn composition_log_degree_bound(self: @CairoAir) -> u32 {
        let mut max_degree = self.opcodes.max_constraint_log_degree_bound();
        max_degree =
            core::cmp::max(max_degree, self.verify_instruction.max_constraint_log_degree_bound());
        max_degree =
            core::cmp::max(max_degree, self.blake_context.max_constraint_log_degree_bound());
        max_degree = core::cmp::max(max_degree, self.builtins.max_constraint_log_degree_bound());
        max_degree =
            core::cmp::max(max_degree, self.memory_address_to_id.max_constraint_log_degree_bound());
        let (memory_id_to_value_big, memory_id_to_value_small) = self.memory_id_to_value;
        for memory_id_to_value_big_component in memory_id_to_value_big.span() {
            max_degree =
                core::cmp::max(
                    max_degree, memory_id_to_value_big_component.max_constraint_log_degree_bound(),
                );
        }
        max_degree =
            core::cmp::max(max_degree, memory_id_to_value_small.max_constraint_log_degree_bound());
        max_degree =
            core::cmp::max(max_degree, self.range_checks.max_constraint_log_degree_bound());
        max_degree =
            core::cmp::max(max_degree, self.verify_bitwise_xor_4.max_constraint_log_degree_bound());
        max_degree =
            core::cmp::max(max_degree, self.verify_bitwise_xor_7.max_constraint_log_degree_bound());
        max_degree =
            core::cmp::max(max_degree, self.verify_bitwise_xor_8.max_constraint_log_degree_bound());
        max_degree =
            core::cmp::max(max_degree, self.verify_bitwise_xor_9.max_constraint_log_degree_bound());
        max_degree
    }

    fn mask_points(
        self: @CairoAir, point: CirclePoint<QM31>,
    ) -> TreeArray<ColumnArray<Array<CirclePoint<QM31>>>> {
        let mut preprocessed_column_set: PreprocessedColumnSet = Default::default();
        let mut trace_mask_points = array![];
        let mut interaction_trace_mask_points = array![];
        self
            .opcodes
            .mask_points(
                ref preprocessed_column_set,
                ref trace_mask_points,
                ref interaction_trace_mask_points,
                point,
            );
        self
            .verify_instruction
            .mask_points(
                ref preprocessed_column_set,
                ref trace_mask_points,
                ref interaction_trace_mask_points,
                point,
            );
        self
            .blake_context
            .mask_points(
                ref preprocessed_column_set,
                ref trace_mask_points,
                ref interaction_trace_mask_points,
                point,
            );
        self
            .builtins
            .mask_points(
                ref preprocessed_column_set,
                ref trace_mask_points,
                ref interaction_trace_mask_points,
                point,
            );
        self
            .memory_address_to_id
            .mask_points(
                ref preprocessed_column_set,
                ref trace_mask_points,
                ref interaction_trace_mask_points,
                point,
            );

        let (memory_id_to_value_big, memory_id_to_value_small) = self.memory_id_to_value;
        for memory_id_to_value_big_component in memory_id_to_value_big.span() {
            memory_id_to_value_big_component
                .mask_points(
                    ref preprocessed_column_set,
                    ref trace_mask_points,
                    ref interaction_trace_mask_points,
                    point,
                );
        }
        memory_id_to_value_small
            .mask_points(
                ref preprocessed_column_set,
                ref trace_mask_points,
                ref interaction_trace_mask_points,
                point,
            );
        self
            .range_checks
            .mask_points(
                ref preprocessed_column_set,
                ref trace_mask_points,
                ref interaction_trace_mask_points,
                point,
            );
        self
            .verify_bitwise_xor_4
            .mask_points(
                ref preprocessed_column_set,
                ref trace_mask_points,
                ref interaction_trace_mask_points,
                point,
            );
        self
            .verify_bitwise_xor_7
            .mask_points(
                ref preprocessed_column_set,
                ref trace_mask_points,
                ref interaction_trace_mask_points,
                point,
            );
        self
            .verify_bitwise_xor_8
            .mask_points(
                ref preprocessed_column_set,
                ref trace_mask_points,
                ref interaction_trace_mask_points,
                point,
            );
        self
            .verify_bitwise_xor_9
            .mask_points(
                ref preprocessed_column_set,
                ref trace_mask_points,
                ref interaction_trace_mask_points,
                point,
            );

        let preprocessed_trace_mask_points = preprocessed_trace_mask_points(
            preprocessed_column_set, point,
        );

        array![preprocessed_trace_mask_points, trace_mask_points, interaction_trace_mask_points]
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
            preprocessed_mask_values, PREPROCESSED_COLUMNS.span(),
        );

        self
            .opcodes
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        self
            .verify_instruction
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        self
            .blake_context
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        self
            .builtins
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        self
            .memory_address_to_id
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        let (memory_id_to_value_big, memory_id_to_value_small) = self.memory_id_to_value;
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
        self
            .range_checks
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        self
            .verify_bitwise_xor_4
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        self
            .verify_bitwise_xor_7
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        self
            .verify_bitwise_xor_8
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        self
            .verify_bitwise_xor_9
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        sum
    }
}


fn preprocessed_trace_mask_points(
    preprocessed_column_set: PreprocessedColumnSet, point: CirclePoint<QM31>,
) -> ColumnArray<Array<CirclePoint<QM31>>> {
    let mut mask_points = array![];

    let PreprocessedColumnSet { values: original_values, mut contains } = preprocessed_column_set;

    for preprocessed_column in PREPROCESSED_COLUMNS.span() {
        let preprocessed_column_key = PreprocessedColumnKey::encode(preprocessed_column);

        if contains.get(preprocessed_column_key) {
            mask_points.append(array![point]);
            // Remove the item from the set.
            contains.insert(preprocessed_column_key, false);
        } else {
            mask_points.append(array![]);
        }
    }

    // Sanity check all the original values have been handled.
    for value in original_values {
        let column_key = PreprocessedColumnKey::encode(@value);
        assert!(!contains.get(column_key));
    }

    mask_points
}
