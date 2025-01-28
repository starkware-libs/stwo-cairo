use components::CairoComponent;
use components::addr_to_id::{
    ClaimImpl as AddrToIdClaimImpl, InteractionClaimImpl as AddrToIdInteractionClaimImpl,
};
use components::genericopcode::{
    ClaimImpl as GenericOpcodeClaimImpl, InteractionClaimImpl as GenericOpcodeInteractionClaimImpl,
};
use components::id_to_f252::{
    ClaimImpl as IdToF252ClaimImpl, InteractionClaimImpl as IdToF252InteractionClaimImpl,
};
use components::jump_t_t_f_opcode::{
    ClaimImpl as JumpFTClaimImpl, InteractionClaimImpl as JumpFTInteractionClaimImpl,
};
use components::range_check::{
    ClaimImpl as RangeCheckClaimImpl, InteractionClaimImpl as RangeCheckInteractionClaimImpl,
};
use components::ret_opcode::{
    ClaimImpl as RetOpcodeClaimImpl, InteractionClaimImpl as RetOpcodeInteractionClaimImpl,
};
use components::verify_instruction::{
    ClaimImpl as VerifyInstructionClaimImpl,
    InteractionClaimImpl as VerifyInstructionInteractionClaimImpl,
};
use core::num::traits::Zero;
use stwo_constraint_framework::{
    LookupElements, LookupElementsImpl, PreprocessedColumn, PreprocessedColumnKey,
    PreprocessedColumnSet, PreprocessedMaskValues, PreprocessedMaskValuesImpl,
};
use stwo_verifier_core::channel::{Channel, ChannelImpl};
use stwo_verifier_core::circle::CirclePoint;
use stwo_verifier_core::fields::Invertible;
use stwo_verifier_core::fields::m31::M31;
use stwo_verifier_core::fields::qm31::{QM31, QM31Zero};
use stwo_verifier_core::fri::FriConfig;
use stwo_verifier_core::pcs::PcsConfig;
use stwo_verifier_core::pcs::verifier::CommitmentSchemeVerifierImpl;
use stwo_verifier_core::utils::ArrayImpl;
use stwo_verifier_core::verifier::Air;
use stwo_verifier_core::verifier::{StarkProof, VerificationError, verify};
use stwo_verifier_core::{ColumnArray, ColumnSpan, TreeArray};

pub mod components;
pub mod utils;

const PREPROCESSED_COLUMNS_LOG_SIZES: [u32; 19] = [
    22, 21, 20, 19, 18, 17, 16, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4,
];

// (Address, Id, Value)
pub type PublicMemory = Array<(u32, u32, [u32; 8])>;

type OpcodeElements = LookupElements<3>;

type VerifyInstructionElements = LookupElements<29>;

type MemoryAddressToIdElements = LookupElements<2>;

type MemoryIdToBigElements = LookupElements<29>;

type RangeCheck3BitElements = LookupElements<1>;

type RangeCheck6BitElements = LookupElements<1>;

type RangeCheck18BitElements = LookupElements<1>;

type RangeCheck19BitElements = LookupElements<1>;

type RangeCheck9Bit9BitElements = LookupElements<2>;

type RangeCheck7Bit2Bit5BitElements = LookupElements<3>;

type RangeCheck4Bit3BitElements = LookupElements<2>;


#[derive(Drop, Serde)]
pub struct CairoProof {
    pub claim: CairoClaim,
    pub interaction_claim: CairoInteractionClaim,
    pub stark_proof: StarkProof,
}

/// The output of a verification.
#[derive(Drop, Serde)]
pub struct VerificationOutput {
    program_hash: felt252,
    output_hash: felt252,
}

pub fn verify_cairo(proof: CairoProof) -> Result<VerificationOutput, CairoVerificationError> {
    let CairoProof { claim, interaction_claim, stark_proof } = proof;

    // Verify.
    let config = PcsConfig {
        pow_bits: 0,
        fri_config: FriConfig {
            log_blowup_factor: 1, log_last_layer_degree_bound: 2, n_queries: 15,
        },
    };
    let mut channel = ChannelImpl::new(0);
    let mut commitment_scheme = CommitmentSchemeVerifierImpl::new(config);

    let log_sizes = claim.log_sizes();

    // Preprocessed trace.
    commitment_scheme
        .commit(*stark_proof.commitment_scheme_proof.commitments[0], *log_sizes[0], ref channel);
    claim.mix_into(ref channel);

    commitment_scheme
        .commit(*stark_proof.commitment_scheme_proof.commitments[1], *log_sizes[1], ref channel);
    let interaction_elements = CairoInteractionElementsImpl::draw(ref channel);

    if lookup_sum(@claim, @interaction_elements, @interaction_claim).is_non_zero() {
        return Result::Err(CairoVerificationError::InvalidLogupSum);
    }

    interaction_claim.mix_into(ref channel);
    commitment_scheme
        .commit(*stark_proof.commitment_scheme_proof.commitments[2], *log_sizes[2], ref channel);

    let cairo_air = CairoAirNewImpl::new(@claim, @interaction_elements, @interaction_claim);
    if let Result::Err(err) = verify(cairo_air, ref channel, stark_proof, ref commitment_scheme) {
        return Result::Err(CairoVerificationError::Stark(err));
    }

    // TODO(alonf): return real values
    Result::Ok(VerificationOutput { program_hash: 0, output_hash: 0 })
}

pub fn lookup_sum(
    claim: @CairoClaim,
    elements: @CairoInteractionElements,
    interaction_claim: @CairoInteractionClaim,
) -> QM31 {
    let mut sum = claim.public_data.logup_sum(elements);

    // If the table is padded, take the sum of the non-padded values.
    // Otherwise, the claimed_sum is the total_sum.
    // TODO(Ohad): hide this logic behind `InteractionClaim`, and only sum here.

    sum += interaction_claim.opcodes.sum();
    sum += *interaction_claim.verify_instruction.claimed_sum;
    sum += *interaction_claim.range_check_19.claimed_sum;
    sum += *interaction_claim.range_check_9_9.claimed_sum;
    sum += *interaction_claim.range_check_7_2_5.claimed_sum;
    sum += *interaction_claim.range_check_4_3.claimed_sum;
    sum += *interaction_claim.memory_addr_to_id.claimed_sum;
    sum += *interaction_claim.memory_id_to_value.big_claimed_sum;
    sum += *interaction_claim.memory_id_to_value.small_claimed_sum;

    sum
}

#[derive(Drop)]
struct CairoInteractionElements {
    pub opcodes: OpcodeElements,
    pub verify_instruction: VerifyInstructionElements,
    pub memory_addr_to_id: MemoryAddressToIdElements,
    pub memory_id_to_value: MemoryIdToBigElements,
    pub range_check_19: RangeCheck19BitElements,
    pub range_check_9_9: RangeCheck9Bit9BitElements,
    pub range_check_7_2_5: RangeCheck7Bit2Bit5BitElements,
    pub range_check_4_3: RangeCheck4Bit3BitElements,
}

#[generate_trait]
impl CairoInteractionElementsImpl of CairoInteractionElementsTrait {
    fn draw(ref channel: Channel) -> CairoInteractionElements {
        CairoInteractionElements {
            opcodes: LookupElementsImpl::draw(ref channel),
            verify_instruction: LookupElementsImpl::draw(ref channel),
            memory_addr_to_id: LookupElementsImpl::draw(ref channel),
            memory_id_to_value: LookupElementsImpl::draw(ref channel),
            range_check_19: LookupElementsImpl::draw(ref channel),
            range_check_9_9: LookupElementsImpl::draw(ref channel),
            range_check_7_2_5: LookupElementsImpl::draw(ref channel),
            range_check_4_3: LookupElementsImpl::draw(ref channel),
        }
    }
}

#[derive(Drop, Serde)]
pub struct CairoClaim {
    pub public_data: PublicData,
    pub opcodes: OpcodeClaim,
    pub memory_addr_to_id: components::addr_to_id::Claim,
    pub memory_id_to_value: components::id_to_f252::Claim,
    pub verify_instruction: components::verify_instruction::Claim,
    pub range_check_19: components::range_check::Claim,
    pub range_check_9_9: components::range_check::Claim,
    pub range_check_7_2_5: components::range_check::Claim,
    pub range_check_4_3: components::range_check::Claim,
}

#[generate_trait]
impl CairoClaimImpl of CairoClaimTrait {
    fn log_sizes(self: @CairoClaim) -> TreeArray<Span<u32>> {
        let mut aggregated_log_sizes = utils::tree_array_concat_cols(
            array![
                self.opcodes.log_sizes(),
                self.verify_instruction.log_sizes(),
                self.memory_addr_to_id.log_sizes(),
                self.memory_id_to_value.log_sizes(),
                self.range_check_19.log_sizes(),
                self.range_check_9_9.log_sizes(),
                self.range_check_7_2_5.log_sizes(),
                self.range_check_4_3.log_sizes(),
            ],
        );

        // Overwrite the preprocessed trace log sizes.
        let _invalid_preprocessed_trace_log_sizes = aggregated_log_sizes.pop_front();

        let mut preprocessed_trace_log_sizes = array![];
        for log_size in PREPROCESSED_COLUMNS_LOG_SIZES.span() {
            preprocessed_trace_log_sizes.append(*log_size);
            preprocessed_trace_log_sizes.append(*log_size);
        };

        let trace_log_sizes = aggregated_log_sizes.pop_front().unwrap();
        let interaction_log_sizes = aggregated_log_sizes.pop_front().unwrap();
        assert!(aggregated_log_sizes.is_empty());
        array![preprocessed_trace_log_sizes.span(), trace_log_sizes, interaction_log_sizes]
    }

    fn mix_into(self: @CairoClaim, ref channel: Channel) {
        self.opcodes.mix_into(ref channel);
        self.memory_addr_to_id.mix_into(ref channel);
        self.memory_id_to_value.mix_into(ref channel);
    }
}

#[derive(Drop, Serde)]
pub struct CairoInteractionClaim {
    pub opcodes: OpcodeInteractionClaim,
    pub verify_instruction: components::verify_instruction::InteractionClaim,
    pub memory_addr_to_id: components::addr_to_id::InteractionClaim,
    pub memory_id_to_value: components::id_to_f252::InteractionClaim,
    pub range_check_19: components::range_check::InteractionClaim,
    pub range_check_9_9: components::range_check::InteractionClaim,
    pub range_check_7_2_5: components::range_check::InteractionClaim,
    pub range_check_4_3: components::range_check::InteractionClaim,
}

#[generate_trait]
impl CairoInteractionClaimImpl of CairoInteractionClaimTrace {
    fn mix_into(self: @CairoInteractionClaim, ref channel: Channel) {
        self.opcodes.mix_into(ref channel);
        self.memory_addr_to_id.mix_into(ref channel);
        self.memory_id_to_value.mix_into(ref channel);
    }
}

#[derive(Drop, Serde)]
pub struct OpcodeInteractionClaim {
    add_f_f: Array::<PlaceHolder>,
    add_f_t: Array::<PlaceHolder>,
    add_t_f: Array::<PlaceHolder>,
    add_t_t: Array::<PlaceHolder>,
    add_ap_f_f: Array::<PlaceHolder>,
    add_ap_f_t: Array::<PlaceHolder>,
    add_ap_t_f: Array::<PlaceHolder>,
    assert_eq_f_f: Array::<PlaceHolder>,
    assert_eq_f_t: Array::<PlaceHolder>,
    assert_eq_t_f: Array::<PlaceHolder>,
    call_f_f: Array::<PlaceHolder>,
    call_f_t: Array::<PlaceHolder>,
    call_t_f: Array::<PlaceHolder>,
    generic: Array::<components::genericopcode::InteractionClaim>,
    jnz_f_f: Array::<PlaceHolder>,
    jnz_f_t: Array::<PlaceHolder>,
    jnz_t_f: Array::<PlaceHolder>,
    jnz_t_t: Array::<PlaceHolder>,
    jump_f_f_f: Array::<PlaceHolder>,
    jump_f_f_t: Array::<PlaceHolder>,
    jump_t_f_f: Array::<PlaceHolder>,
    jump_t_t_f: Array::<components::jump_t_t_f_opcode::InteractionClaim>,
    mul_f_f: Array::<PlaceHolder>,
    mul_f_t: Array::<PlaceHolder>,
    ret: Array::<components::ret_opcode::InteractionClaim>,
}

#[generate_trait]
impl OpcodeInteractionClaimImpl of OpcodeInteractionClaimTrait {
    fn mix_into(self: @OpcodeInteractionClaim, ref channel: Channel) {
        for interaction_claim in self.generic.span() {
            interaction_claim.mix_into(ref channel);
        };

        for interaction_claim in self.jump_t_t_f.span() {
            interaction_claim.mix_into(ref channel);
        };

        for interaction_claim in self.ret.span() {
            interaction_claim.mix_into(ref channel);
        };
    }

    fn sum(self: @OpcodeInteractionClaim) -> QM31 {
        let mut sum = QM31Zero::zero();

        for interaction_claim in self.generic.span() {
            sum += match interaction_claim.claimed_sum {
                Option::Some((claimed_sum, _)) => *claimed_sum,
                Option::None => *interaction_claim.total_sum,
            };
        };

        for interaction_claim in self.jump_t_t_f.span() {
            sum += match interaction_claim.claimed_sum {
                Option::Some((claimed_sum, _)) => *claimed_sum,
                Option::None => *interaction_claim.total_sum,
            };
        };

        for interaction_claim in self.ret.span() {
            sum += match interaction_claim.claimed_sum {
                Option::Some((claimed_sum, _)) => *claimed_sum,
                Option::None => *interaction_claim.total_sum,
            };
        };

        sum
    }
}

#[derive(Drop, Serde)]
pub struct PublicData {
    pub public_memory: PublicMemory,
    pub initial_state: CasmState,
    pub final_state: CasmState,
}

#[generate_trait]
impl PublicDataImpl of PublicDataTrait {
    fn logup_sum(self: @PublicData, lookup_elements: @CairoInteractionElements) -> QM31 {
        // TODO(Ohad): Optimized inverse.
        let mut sum = QM31Zero::zero();

        for entry in self.public_memory.span() {
            let (addr, id, val) = *entry;

            let addr_m31 = addr.try_into().unwrap();
            let id_m31 = id.try_into().unwrap();
            let addr_to_id = lookup_elements
                .memory_addr_to_id
                .combine([addr_m31, id_m31])
                .inverse();

            let mut elements = array![id_m31];
            elements.append_span(utils::split_f252(val).span());
            let id_to_value = lookup_elements
                .memory_id_to_value
                .combine((*elements.span().try_into().unwrap()).unbox())
                .inverse();

            sum += addr_to_id + id_to_value;
        };

        // Yield initial state and use the final.
        let CasmState { pc, ap, fp } = *self.final_state;
        sum += lookup_elements.opcodes.combine([pc, ap, fp]).inverse();
        let CasmState { pc, ap, fp } = *self.initial_state;
        sum -= lookup_elements.opcodes.combine([pc, ap, fp]).inverse();

        sum
    }
}

#[derive(Drop, Serde, Copy)]
pub struct CasmState {
    pub pc: M31,
    pub ap: M31,
    pub fp: M31,
}

type PlaceHolder = u32;

#[derive(Drop, Serde)]
pub struct OpcodeClaim {
    pub add_f_f: Array::<PlaceHolder>,
    pub add_f_t: Array::<PlaceHolder>,
    pub add_t_f: Array::<PlaceHolder>,
    pub add_t_t: Array::<PlaceHolder>,
    pub add_ap_f_f: Array::<PlaceHolder>,
    pub add_ap_f_t: Array::<PlaceHolder>,
    pub add_ap_t_f: Array::<PlaceHolder>,
    pub assert_eq_f_f: Array::<PlaceHolder>,
    pub assert_eq_f_t: Array::<PlaceHolder>,
    pub assert_eq_t_f: Array::<PlaceHolder>,
    pub call_f_f: Array::<PlaceHolder>,
    pub call_f_t: Array::<PlaceHolder>,
    pub call_t_f: Array::<PlaceHolder>,
    pub generic: Array::<components::genericopcode::Claim>,
    pub jnz_f_f: Array::<PlaceHolder>,
    pub jnz_f_t: Array::<PlaceHolder>,
    pub jnz_t_f: Array::<PlaceHolder>,
    pub jnz_t_t: Array::<PlaceHolder>,
    pub jump_f_f_f: Array::<PlaceHolder>,
    pub jump_f_f_t: Array::<PlaceHolder>,
    pub jump_t_f_f: Array::<PlaceHolder>,
    pub jump_t_t_f: Array::<components::jump_t_t_f_opcode::Claim>,
    pub mul_f_f: Array::<PlaceHolder>,
    pub mul_f_t: Array::<PlaceHolder>,
    pub ret: Array::<components::ret_opcode::Claim>,
}

#[generate_trait]
impl OpcodeClaimImpl of OpcodeClaimTrait {
    fn mix_into(self: @OpcodeClaim, ref channel: Channel) {
        for generic_opcode_claim in self.generic.span() {
            generic_opcode_claim.mix_into(ref channel);
        };

        for jump_t_t_f_opcode_claim in self.jump_t_t_f.span() {
            jump_t_t_f_opcode_claim.mix_into(ref channel);
        };

        for ret_opcode_claim in self.ret.span() {
            ret_opcode_claim.mix_into(ref channel);
        };
    }

    fn log_sizes(self: @OpcodeClaim) -> TreeArray<Span<u32>> {
        let mut log_sizes = array![];

        for ret_opcode_claim in self.ret.span() {
            log_sizes.append(ret_opcode_claim.log_sizes());
        };

        for generic_opcode_claim in self.generic.span() {
            log_sizes.append(generic_opcode_claim.log_sizes());
        };

        for jump_t_t_f_opcode_claim in self.jump_t_t_f.span() {
            log_sizes.append(jump_t_t_f_opcode_claim.log_sizes());
        };

        utils::tree_array_concat_cols(log_sizes)
    }
}

#[derive(Drop, Debug)]
pub enum CairoVerificationError {
    InvalidLogupSum,
    Stark: VerificationError,
}

#[derive(Drop)]
pub struct CairoAir {
    opcodes: OpcodeComponents,
    verify_instruction: components::verify_instruction::Component,
    memory_addr_to_id: components::addr_to_id::Component,
    memory_id_to_value: (
        components::id_to_f252::BigComponent, components::id_to_f252::SmallComponent,
    ),
    range_check_19: components::range_check::Rc19BitComponent,
    range_check_9_9: components::range_check::Rc9Bit9BitComponent,
    range_check_7_2_5: components::range_check::Rc7Bit2Bit5BitComponent,
    range_check_4_3: components::range_check::Rc4Bit3BitComponent,
    // ...
    preprocessed_columns: Array<PreprocessedColumn>,
}

#[generate_trait]
impl CairoAirNewImpl of CairoAirNewTrait {
    fn new(
        cairo_claim: @CairoClaim,
        interaction_elements: @CairoInteractionElements,
        interaction_claim: @CairoInteractionClaim,
    ) -> CairoAir {
        let mut preprocessed_columns = array![];

        // TODO: This could be a constant.
        for is_first_log_size in PREPROCESSED_COLUMNS_LOG_SIZES.span() {
            preprocessed_columns.append(PreprocessedColumn::IsFirst(*is_first_log_size));
            preprocessed_columns.append(PreprocessedColumn::Seq(*is_first_log_size));
        };

        let opcode_components = OpcodeComponentsImpl::new(
            cairo_claim.opcodes, interaction_elements, interaction_claim.opcodes,
        );

        let verifyinstruction_component = components::verify_instruction::Component {
            claim: *cairo_claim.verify_instruction,
            interaction_claim: *interaction_claim.verify_instruction,
            memoryaddresstoid_lookup_elements: interaction_elements.memory_addr_to_id.clone(),
            memoryidtobig_lookup_elements: interaction_elements.memory_id_to_value.clone(),
            rangecheck_4_3_lookup_elements: interaction_elements.range_check_4_3.clone(),
            range_check_7_2_5_lookup_elements: interaction_elements.range_check_7_2_5.clone(),
            verifyinstruction_lookup_elements: interaction_elements.verify_instruction.clone(),
        };

        let memory_addr_to_id_component = components::addr_to_id::Component {
            claim: *cairo_claim.memory_addr_to_id,
            interaction_claim: *interaction_claim.memory_addr_to_id,
            lookup_elements: interaction_elements.memory_addr_to_id.clone(),
        };

        let memory_id_to_value_component = components::id_to_f252::BigComponent {
            log_n_rows: *cairo_claim.memory_id_to_value.big_log_size,
            interaction_claim: *interaction_claim.memory_id_to_value,
            lookup_elements: interaction_elements.memory_id_to_value.clone(),
            range_9_9_lookup_elements: interaction_elements.range_check_9_9.clone(),
        };

        let small_memory_id_to_value_component = components::id_to_f252::SmallComponent {
            log_n_rows: *cairo_claim.memory_id_to_value.small_log_size,
            interaction_claim: *interaction_claim.memory_id_to_value,
            lookup_elements: interaction_elements.memory_id_to_value.clone(),
            range_9_9_lookup_elements: interaction_elements.range_check_9_9.clone(),
        };

        let range_check_19_component = components::range_check::Rc19BitComponent {
            lookup_elements: interaction_elements.range_check_19.clone(),
            interaction_claim: interaction_claim.range_check_19.clone(),
        };

        let range_check_9_9_component = components::range_check::Rc9Bit9BitComponent {
            lookup_elements: interaction_elements.range_check_9_9.clone(),
            interaction_claim: interaction_claim.range_check_9_9.clone(),
        };

        let range_check_7_2_5_component = components::range_check::Rc7Bit2Bit5BitComponent {
            lookup_elements: interaction_elements.range_check_7_2_5.clone(),
            interaction_claim: interaction_claim.range_check_7_2_5.clone(),
        };

        let range_check_4_3_component = components::range_check::Rc4Bit3BitComponent {
            lookup_elements: interaction_elements.range_check_4_3.clone(),
            interaction_claim: interaction_claim.range_check_4_3.clone(),
        };

        CairoAir {
            opcodes: opcode_components,
            verify_instruction: verifyinstruction_component,
            memory_addr_to_id: memory_addr_to_id_component,
            memory_id_to_value: (memory_id_to_value_component, small_memory_id_to_value_component),
            range_check_19: range_check_19_component,
            range_check_9_9: range_check_9_9_component,
            range_check_7_2_5: range_check_7_2_5_component,
            range_check_4_3: range_check_4_3_component,
            preprocessed_columns,
        }
    }
}

impl CairoAirImpl of Air<CairoAir> {
    fn composition_log_degree_bound(self: @CairoAir) -> u32 {
        let composition_log_degree_bound = self.opcodes.max_constraint_log_degree_bound();
        // TODO: ...
        //composition_log_degree_bound
        20
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
            .memory_addr_to_id
            .mask_points(
                ref preprocessed_column_set,
                ref trace_mask_points,
                ref interaction_trace_mask_points,
                point,
            );

        let (memory_id_to_value_big, memory_id_to_value_small) = self.memory_id_to_value;
        memory_id_to_value_big
            .mask_points(
                ref preprocessed_column_set,
                ref trace_mask_points,
                ref interaction_trace_mask_points,
                point,
            );
        memory_id_to_value_small
            .mask_points(
                ref preprocessed_column_set,
                ref trace_mask_points,
                ref interaction_trace_mask_points,
                point,
            );
        self
            .range_check_19
            .mask_points(
                ref preprocessed_column_set,
                ref trace_mask_points,
                ref interaction_trace_mask_points,
                point,
            );
        self
            .range_check_9_9
            .mask_points(
                ref preprocessed_column_set,
                ref trace_mask_points,
                ref interaction_trace_mask_points,
                point,
            );
        self
            .range_check_7_2_5
            .mask_points(
                ref preprocessed_column_set,
                ref trace_mask_points,
                ref interaction_trace_mask_points,
                point,
            );
        self
            .range_check_4_3
            .mask_points(
                ref preprocessed_column_set,
                ref trace_mask_points,
                ref interaction_trace_mask_points,
                point,
            );

        let preprocessed_trace_mask_points = preprocessed_trace_mask_points(
            preprocessed_column_set,
        );

        array![preprocessed_trace_mask_points, trace_mask_points, interaction_trace_mask_points]
    }

    fn eval_composition_polynomial_at_point(
        self: @CairoAir,
        point: CirclePoint<QM31>,
        mask_values: @TreeArray<ColumnArray<Array<QM31>>>,
        random_coeff: QM31,
    ) -> QM31 {
        let mut sum = QM31Zero::zero();

        let mut preprocessed_mask_values = PreprocessedMaskValuesImpl::new(
            mask_values[0].span(), self.preprocessed_columns.span(),
        );

        let mut trace_mask_values = mask_values[1].span();
        let mut interaction_trace_mask_values = mask_values[2].span();

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
            .memory_addr_to_id
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );

        let (memory_id_to_value_big, memory_id_to_value_small) = self.memory_id_to_value;

        memory_id_to_value_big
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );

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
            .range_check_19
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        self
            .range_check_9_9
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        self
            .range_check_7_2_5
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        self
            .range_check_4_3
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
    preprocessed_column_set: PreprocessedColumnSet,
) -> ColumnArray<Array<CirclePoint<QM31>>> {
    let mut mask_points = array![];

    let PreprocessedColumnSet { values: original_values, mut contains } = preprocessed_column_set;

    for log_size in PREPROCESSED_COLUMNS_LOG_SIZES.span() {
        let is_first = PreprocessedColumn::IsFirst(*log_size);
        let seq = PreprocessedColumn::Seq(*log_size);

        // After adding the mask points, remove the item from the set.
        contains.insert(PreprocessedColumnKey::encode(@is_first), false);
        contains.insert(PreprocessedColumnKey::encode(@seq), false);
    };

    // Check all the original values have been handled.
    for value in original_values {
        let column_key = PreprocessedColumnKey::encode(@value);
        assert!(!contains.get(column_key));
    };

    mask_points
}

#[derive(Drop)]
pub struct OpcodeComponents {
    generic: Array<components::genericopcode::Component>,
    ret: Array<components::ret_opcode::Component>,
    jump_t_t_f: Array<components::jump_t_t_f_opcode::Component>,
}

#[generate_trait]
impl OpcodeComponentsImpl of OpcodeComponentsTrait {
    fn new(
        claim: @OpcodeClaim,
        interaction_elements: @CairoInteractionElements,
        interaction_claim: @OpcodeInteractionClaim,
    ) -> OpcodeComponents {
        // TODO: Handle dynamic number of components.
        assert!(claim.generic.len() == 0);
        assert!(claim.jump_t_t_f.len() == 1);
        assert!(claim.ret.len() == 0);
        assert!(interaction_claim.generic.len() == 0);
        assert!(interaction_claim.jump_t_t_f.len() == 1);
        assert!(interaction_claim.ret.len() == 0);
        // let generic_opcode_component = components::genericopcode::Component {
        //     claim: *claim.generic[0],
        //     interaction_claim: *interaction_claim.generic[0],
        //     memoryaddresstoid_lookup_elements: interaction_elements.memory_addr_to_id.clone(),
        //     memoryidtobig_lookup_elements: interaction_elements.memory_id_to_value.clone(),
        //     verifyinstruction_lookup_elements: interaction_elements.verify_instruction.clone(),
        //     opcodes_lookup_elements: interaction_elements.opcodes.clone(),
        //     range_check_19_lookup_elements: interaction_elements.range_check_19.clone(),
        //     range_check_9_9_lookup_elements: interaction_elements.range_check_9_9.clone(),
        // };
        // let ret_opcode_component = components::ret_opcode::Component {
        //     claim: *claim.ret[0],
        //     interaction_claim: *interaction_claim.ret[0],
        //     memoryaddresstoid_lookup_elements: interaction_elements.memory_addr_to_id.clone(),
        //     memoryidtobig_lookup_elements: interaction_elements.memory_id_to_value.clone(),
        //     verifyinstruction_lookup_elements: interaction_elements.verify_instruction.clone(),
        //     opcodes_lookup_elements: interaction_elements.opcodes.clone(),
        // };

        let jump_t_t_f_opcode_component = components::jump_t_t_f_opcode::Component {
            claim: *claim.jump_t_t_f[0],
            interaction_claim: *interaction_claim.jump_t_t_f[0],
            memoryaddresstoid_lookup_elements: interaction_elements.memory_addr_to_id.clone(),
            memoryidtobig_lookup_elements: interaction_elements.memory_id_to_value.clone(),
            verifyinstruction_lookup_elements: interaction_elements.verify_instruction.clone(),
            opcodes_lookup_elements: interaction_elements.opcodes.clone(),
        };

        OpcodeComponents {
            generic: array![], ret: array![], jump_t_t_f: array![jump_t_t_f_opcode_component],
        }
    }

    fn mask_points(
        self: @OpcodeComponents,
        ref preprocessed_column_set: PreprocessedColumnSet,
        ref trace_mask_points: Array<Array<CirclePoint<QM31>>>,
        ref interaction_trace_mask_points: Array<Array<CirclePoint<QM31>>>,
        point: CirclePoint<QM31>,
    ) {
        for component in self.generic.span() {
            component
                .mask_points(
                    ref preprocessed_column_set,
                    ref trace_mask_points,
                    ref interaction_trace_mask_points,
                    point,
                );
        };

        for component in self.ret.span() {
            component
                .mask_points(
                    ref preprocessed_column_set,
                    ref trace_mask_points,
                    ref interaction_trace_mask_points,
                    point,
                );
        };

        for component in self.jump_t_t_f.span() {
            component
                .mask_points(
                    ref preprocessed_column_set,
                    ref trace_mask_points,
                    ref interaction_trace_mask_points,
                    point,
                );
        };
    }

    fn max_constraint_log_degree_bound(self: @OpcodeComponents) -> u32 {
        let mut max_degree = 0;

        for component in self.generic.span() {
            max_degree = core::cmp::max(max_degree, component.max_constraint_log_degree_bound());
        };

        for component in self.ret.span() {
            max_degree = core::cmp::max(max_degree, component.max_constraint_log_degree_bound());
        };

        max_degree
    }

    fn evaluate_constraints_at_point(
        self: @OpcodeComponents,
        ref sum: QM31,
        ref preprocessed_mask_values: PreprocessedMaskValues,
        ref trace_mask_values: ColumnSpan<Array<QM31>>,
        ref interaction_trace_mask_values: ColumnSpan<Array<QM31>>,
        random_coeff: QM31,
        point: CirclePoint<QM31>,
    ) {
        for component in self.generic.span() {
            component
                .evaluate_constraints_at_point(
                    ref sum,
                    ref preprocessed_mask_values,
                    ref trace_mask_values,
                    ref interaction_trace_mask_values,
                    random_coeff,
                    point,
                );
        };

        for component in self.ret.span() {
            component
                .evaluate_constraints_at_point(
                    ref sum,
                    ref preprocessed_mask_values,
                    ref trace_mask_values,
                    ref interaction_trace_mask_values,
                    random_coeff,
                    point,
                );
        };

        for component in self.jump_t_t_f.span() {
            component
                .evaluate_constraints_at_point(
                    ref sum,
                    ref preprocessed_mask_values,
                    ref trace_mask_values,
                    ref interaction_trace_mask_values,
                    random_coeff,
                    point,
                );
        };
    }
}
