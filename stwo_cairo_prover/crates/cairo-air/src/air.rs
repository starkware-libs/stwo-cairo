use itertools::{chain, Itertools};
use num_traits::Zero;
use serde::{Deserialize, Serialize};
use stwo_cairo_common::prover_types::cpu::CasmState;
use stwo_cairo_common::prover_types::felt::split_f252;
use stwo_cairo_serialize::CairoSerialize;
use stwo_prover::constraint_framework::preprocessed_columns::PreProcessedColumnId;
use stwo_prover::constraint_framework::{Relation, TraceLocationAllocator};
use stwo_prover::core::air::{Component, ComponentProver};
use stwo_prover::core::backend::simd::SimdBackend;
use stwo_prover::core::channel::Channel;
use stwo_prover::core::fields::m31::M31;
use stwo_prover::core::fields::qm31::{SecureField, QM31};
use stwo_prover::core::fields::FieldExpOps;
use stwo_prover::core::pcs::TreeVec;
use stwo_prover::core::prover::StarkProof;
use stwo_prover::core::vcs::ops::MerkleHasher;

use super::blake::air::{BlakeContextClaim, BlakeContextComponents, BlakeContextInteractionClaim};
use super::builtins_air::{BuiltinComponents, BuiltinsClaim, BuiltinsInteractionClaim};
use super::components::indented_component_display;
use super::opcodes_air::{OpcodeClaim, OpcodeComponents, OpcodeInteractionClaim};
use super::pedersen::air::{
    PedersenContextClaim, PedersenContextComponents, PedersenContextInteractionClaim,
};
use super::poseidon::air::{
    PoseidonContextClaim, PoseidonContextComponents, PoseidonContextInteractionClaim,
};
use super::range_checks_air::{
    RangeChecksClaim, RangeChecksComponents, RangeChecksInteractionClaim,
    RangeChecksInteractionElements,
};
use crate::components::{
    memory_address_to_id, memory_id_to_big, verify_bitwise_xor_4, verify_bitwise_xor_7,
    verify_bitwise_xor_8, verify_bitwise_xor_9, verify_instruction,
};
use crate::relations;

#[derive(Serialize, Deserialize)]
pub struct CairoProof<H: MerkleHasher> {
    pub claim: CairoClaim,
    pub interaction_pow: u64,
    pub interaction_claim: CairoInteractionClaim,
    pub stark_proof: StarkProof<H>,
}

impl<H: MerkleHasher> CairoSerialize for CairoProof<H>
where
    H::Hash: CairoSerialize,
{
    fn serialize(&self, output: &mut Vec<starknet_ff::FieldElement>) {
        let Self {
            claim,
            interaction_pow,
            interaction_claim,
            stark_proof,
        } = self;
        CairoSerialize::serialize(claim, output);
        CairoSerialize::serialize(interaction_pow, output);
        CairoSerialize::serialize(interaction_claim, output);
        CairoSerialize::serialize(stark_proof, output);
    }
}

#[derive(Serialize, Deserialize, CairoSerialize)]
pub struct CairoClaim {
    pub public_data: PublicData,
    pub opcodes: OpcodeClaim,
    pub verify_instruction: verify_instruction::Claim,
    pub blake_context: BlakeContextClaim,
    pub builtins: BuiltinsClaim,
    pub pedersen_context: PedersenContextClaim,
    pub poseidon_context: PoseidonContextClaim,
    pub memory_address_to_id: memory_address_to_id::Claim,
    pub memory_id_to_value: memory_id_to_big::Claim,
    pub range_checks: RangeChecksClaim,
    pub verify_bitwise_xor_4: verify_bitwise_xor_4::Claim,
    pub verify_bitwise_xor_7: verify_bitwise_xor_7::Claim,
    pub verify_bitwise_xor_8: verify_bitwise_xor_8::Claim,
    pub verify_bitwise_xor_9: verify_bitwise_xor_9::Claim,
    // ...
}

impl CairoClaim {
    pub fn mix_into(&self, channel: &mut impl Channel) {
        // TODO(spapini): Add common values.
        self.opcodes.mix_into(channel);
        self.verify_instruction.mix_into(channel);
        self.blake_context.mix_into(channel);
        self.builtins.mix_into(channel);
        self.pedersen_context.mix_into(channel);
        self.poseidon_context.mix_into(channel);
        self.memory_address_to_id.mix_into(channel);
        self.memory_id_to_value.mix_into(channel);
        self.range_checks.mix_into(channel);
        self.verify_bitwise_xor_4.mix_into(channel);
        self.verify_bitwise_xor_7.mix_into(channel);
        self.verify_bitwise_xor_8.mix_into(channel);
        self.verify_bitwise_xor_9.mix_into(channel);
    }

    /// Returns the log sizes of the components.
    /// Does not include the preprocessed trace log sizes.
    pub fn log_sizes(&self) -> TreeVec<Vec<u32>> {
        let log_sizes_list = vec![
            self.opcodes.log_sizes(),
            self.verify_instruction.log_sizes(),
            self.blake_context.log_sizes(),
            self.builtins.log_sizes(),
            self.pedersen_context.log_sizes(),
            self.poseidon_context.log_sizes(),
            self.memory_address_to_id.log_sizes(),
            self.memory_id_to_value.log_sizes(),
            self.range_checks.log_sizes(),
            self.verify_bitwise_xor_4.log_sizes(),
            self.verify_bitwise_xor_7.log_sizes(),
            self.verify_bitwise_xor_8.log_sizes(),
            self.verify_bitwise_xor_9.log_sizes(),
        ];

        TreeVec::concat_cols(log_sizes_list.into_iter())
    }
}

#[derive(Serialize, Deserialize, CairoSerialize)]
pub struct PublicData {
    pub public_memory: PublicMemory,
    pub initial_state: CasmState,
    pub final_state: CasmState,
}
impl PublicData {
    /// Sums the logup of the public data.
    pub fn logup_sum(&self, lookup_elements: &CairoInteractionElements) -> QM31 {
        let mut values_to_inverse = vec![];
        // Use public memory in the memory relations.
        self.public_memory
            .get_entries(
                self.initial_state.pc.0,
                self.initial_state.ap.0,
                self.final_state.ap.0,
            )
            .for_each(|(addr, id, val)| {
                values_to_inverse.push(
                    <relations::MemoryAddressToId as Relation<M31, QM31>>::combine(
                        &lookup_elements.memory_address_to_id,
                        &[M31::from_u32_unchecked(addr), M31::from_u32_unchecked(id)],
                    ),
                );
                values_to_inverse.push(<relations::MemoryIdToBig as Relation<M31, QM31>>::combine(
                    &lookup_elements.memory_id_to_value,
                    &[
                        [M31::from_u32_unchecked(id)].as_slice(),
                        split_f252(val).as_slice(),
                    ]
                    .concat(),
                ));
            });

        // Yield initial state and use the final.
        values_to_inverse.push(<relations::Opcodes as Relation<M31, QM31>>::combine(
            &lookup_elements.opcodes,
            &self.final_state.values(),
        ));
        values_to_inverse.push(-<relations::Opcodes as Relation<M31, QM31>>::combine(
            &lookup_elements.opcodes,
            &self.initial_state.values(),
        ));

        let inverted_values = QM31::batch_inverse(&values_to_inverse);
        inverted_values.iter().sum::<QM31>()
    }
}

// TODO(alonf) Change all the obscure types and structs to a meaninful struct system for the memory.
#[derive(Clone, Debug, Serialize, Deserialize, Copy, CairoSerialize)]
pub struct MemorySmallValue {
    pub id: u32,
    pub value: u32,
}

// TODO(alonf): Change this into a struct. Remove Pub prefix.
// (id, value)
pub type PubMemoryValue = (u32, [u32; 8]);

// TODO(alonf): Change this into a struct. Remove Pub prefix.
// (address, id, value)
pub type PubMemoryEntry = (u32, u32, [u32; 8]);

#[derive(Clone, Debug, Serialize, Deserialize, Copy, CairoSerialize)]
pub struct SegmentRange {
    pub start_ptr: MemorySmallValue,
    pub stop_ptr: MemorySmallValue,
}

#[derive(Clone, Debug, Serialize, Deserialize, Copy, CairoSerialize)]
pub struct PublicSegmentRanges {
    pub output: SegmentRange,
    pub pedersen: SegmentRange,
    pub range_check_128: SegmentRange,
    pub ecdsa: SegmentRange,
    pub bitwise: SegmentRange,
    pub ec_op: SegmentRange,
    pub keccak: SegmentRange,
    pub poseidon: SegmentRange,
    pub range_check_96: SegmentRange,
    pub add_mod: SegmentRange,
    pub mul_mod: SegmentRange,
}

impl PublicSegmentRanges {
    pub fn memory_entries(
        &self,
        initial_ap: u32,
        final_ap: u32,
    ) -> impl Iterator<Item = PubMemoryEntry> {
        let PublicSegmentRanges {
            output,
            pedersen,
            range_check_128,
            ecdsa,
            bitwise,
            ec_op,
            keccak,
            poseidon,
            range_check_96,
            add_mod,
            mul_mod,
        } = *self;
        let segments = [
            output,
            pedersen,
            range_check_128,
            ecdsa,
            bitwise,
            ec_op,
            keccak,
            poseidon,
            range_check_96,
            add_mod,
            mul_mod,
        ]
        .into_iter()
        .collect_vec();

        let n_segments = segments.len() as u32;
        assert_eq!(n_segments, 11);

        segments
            .into_iter()
            .enumerate()
            .flat_map(
                move |(
                    i,
                    SegmentRange {
                        start_ptr,
                        stop_ptr,
                    },
                )| {
                    let start_address = initial_ap + i as u32;
                    let stop_address = final_ap - n_segments + i as u32;
                    [
                        (start_address, start_ptr.id, start_ptr.value),
                        (stop_address, stop_ptr.id, stop_ptr.value),
                    ]
                },
            )
            .map(|(addr, id, value)| (addr, id, [value, 0, 0, 0, 0, 0, 0, 0]))
    }
}

pub type MemorySection = Vec<PubMemoryValue>;

// TODO(alonf): Perform all public data validations.
#[derive(Serialize, Deserialize, CairoSerialize)]
pub struct PublicMemory {
    pub program: MemorySection,
    pub public_segments: PublicSegmentRanges,
    pub output: MemorySection,
    pub safe_call: MemorySection,
}

impl PublicMemory {
    /// Returns [`PubMemoryEntry`] for all public memory.
    pub fn get_entries(
        &self,
        initial_pc: u32,
        initial_ap: u32,
        final_ap: u32,
    ) -> impl Iterator<Item = PubMemoryEntry> {
        let [program, safe_call, output] = [&self.program, &self.safe_call, &self.output]
            .map(|section| section.clone().into_iter().enumerate());
        let program_iter = program.map(move |(i, (id, value))| (initial_pc + i as u32, id, value));
        let output_iter = output.map(move |(i, (id, value))| (final_ap + i as u32, id, value));
        let safe_call_iter =
            safe_call.map(move |(i, (id, value))| (initial_ap - 2 + i as u32, id, value));
        let segment_ranges_iter = self.public_segments.memory_entries(initial_ap, final_ap);

        program_iter
            .chain(safe_call_iter)
            .chain(segment_ranges_iter)
            .chain(output_iter)
    }
}

pub struct CairoInteractionElements {
    pub opcodes: relations::Opcodes,
    pub verify_instruction: relations::VerifyInstruction,
    pub blake_round: relations::BlakeRound,
    pub blake_g: relations::BlakeG,
    pub blake_sigma: relations::BlakeRoundSigma,
    pub triple_xor_32: relations::TripleXor32,
    pub partial_ec_mul: relations::PartialEcMul,
    pub pedersen_points_table: relations::PedersenPointsTable,
    pub poseidon_3_partial_rounds_chain: relations::Poseidon3PartialRoundsChain,
    pub poseidon_full_round_chain: relations::PoseidonFullRoundChain,
    pub cube_252: relations::Cube252,
    pub poseidon_round_keys: relations::PoseidonRoundKeys,
    pub range_check_felt_252_width_27: relations::RangeCheckFelt252Width27,
    pub memory_address_to_id: relations::MemoryAddressToId,
    pub memory_id_to_value: relations::MemoryIdToBig,
    pub range_checks: RangeChecksInteractionElements,
    pub verify_bitwise_xor_4: relations::VerifyBitwiseXor_4,
    pub verify_bitwise_xor_7: relations::VerifyBitwiseXor_7,
    pub verify_bitwise_xor_8: relations::VerifyBitwiseXor_8,
    pub verify_bitwise_xor_9: relations::VerifyBitwiseXor_9,
    pub verify_bitwise_xor_12: relations::VerifyBitwiseXor_12,
}
impl CairoInteractionElements {
    pub fn draw(channel: &mut impl Channel) -> CairoInteractionElements {
        CairoInteractionElements {
            opcodes: relations::Opcodes::draw(channel),
            verify_instruction: relations::VerifyInstruction::draw(channel),
            blake_round: relations::BlakeRound::draw(channel),
            blake_g: relations::BlakeG::draw(channel),
            blake_sigma: relations::BlakeRoundSigma::draw(channel),
            triple_xor_32: relations::TripleXor32::draw(channel),
            poseidon_3_partial_rounds_chain: relations::Poseidon3PartialRoundsChain::draw(channel),
            poseidon_full_round_chain: relations::PoseidonFullRoundChain::draw(channel),
            cube_252: relations::Cube252::draw(channel),
            poseidon_round_keys: relations::PoseidonRoundKeys::draw(channel),
            range_check_felt_252_width_27: relations::RangeCheckFelt252Width27::draw(channel),
            partial_ec_mul: relations::PartialEcMul::draw(channel),
            pedersen_points_table: relations::PedersenPointsTable::draw(channel),
            memory_address_to_id: relations::MemoryAddressToId::draw(channel),
            memory_id_to_value: relations::MemoryIdToBig::draw(channel),
            range_checks: RangeChecksInteractionElements::draw(channel),
            verify_bitwise_xor_4: relations::VerifyBitwiseXor_4::draw(channel),
            verify_bitwise_xor_7: relations::VerifyBitwiseXor_7::draw(channel),
            verify_bitwise_xor_8: relations::VerifyBitwiseXor_8::draw(channel),
            verify_bitwise_xor_9: relations::VerifyBitwiseXor_9::draw(channel),
            verify_bitwise_xor_12: relations::VerifyBitwiseXor_12::draw(channel),
        }
    }
}

#[derive(Serialize, Deserialize, CairoSerialize)]
pub struct CairoInteractionClaim {
    pub opcodes: OpcodeInteractionClaim,
    pub verify_instruction: verify_instruction::InteractionClaim,
    pub blake_context: BlakeContextInteractionClaim,
    pub builtins: BuiltinsInteractionClaim,
    pub pedersen_context: PedersenContextInteractionClaim,
    pub poseidon_context: PoseidonContextInteractionClaim,
    pub memory_address_to_id: memory_address_to_id::InteractionClaim,
    pub memory_id_to_value: memory_id_to_big::InteractionClaim,
    pub range_checks: RangeChecksInteractionClaim,
    pub verify_bitwise_xor_4: verify_bitwise_xor_4::InteractionClaim,
    pub verify_bitwise_xor_7: verify_bitwise_xor_7::InteractionClaim,
    pub verify_bitwise_xor_8: verify_bitwise_xor_8::InteractionClaim,
    pub verify_bitwise_xor_9: verify_bitwise_xor_9::InteractionClaim,
}
impl CairoInteractionClaim {
    pub fn mix_into(&self, channel: &mut impl Channel) {
        self.opcodes.mix_into(channel);
        self.verify_instruction.mix_into(channel);
        self.blake_context.mix_into(channel);
        self.builtins.mix_into(channel);
        self.pedersen_context.mix_into(channel);
        self.poseidon_context.mix_into(channel);
        self.memory_address_to_id.mix_into(channel);
        self.memory_id_to_value.mix_into(channel);
        self.range_checks.mix_into(channel);
        self.verify_bitwise_xor_4.mix_into(channel);
        self.verify_bitwise_xor_7.mix_into(channel);
        self.verify_bitwise_xor_8.mix_into(channel);
        self.verify_bitwise_xor_9.mix_into(channel);
    }
}

pub fn lookup_sum(
    claim: &CairoClaim,
    elements: &CairoInteractionElements,
    interaction_claim: &CairoInteractionClaim,
) -> SecureField {
    let mut sum = QM31::zero();
    sum += claim.public_data.logup_sum(elements);

    // If the table is padded, take the sum of the non-padded values.
    // Otherwise, the claimed_sum is the total_sum.
    sum += interaction_claim.opcodes.sum();
    sum += interaction_claim.verify_instruction.claimed_sum;
    sum += interaction_claim.blake_context.sum();
    sum += interaction_claim.builtins.sum();
    sum += interaction_claim.pedersen_context.sum();
    sum += interaction_claim.poseidon_context.sum();
    sum += interaction_claim.memory_address_to_id.claimed_sum;
    sum += interaction_claim.memory_id_to_value.big_claimed_sum;
    sum += interaction_claim.memory_id_to_value.small_claimed_sum;
    sum += interaction_claim.range_checks.sum();
    sum += interaction_claim.verify_bitwise_xor_4.claimed_sum;
    sum += interaction_claim.verify_bitwise_xor_7.claimed_sum;
    sum += interaction_claim.verify_bitwise_xor_8.claimed_sum;
    sum += interaction_claim.verify_bitwise_xor_9.claimed_sum;

    sum
}

pub struct CairoComponents {
    pub opcodes: OpcodeComponents,
    pub verify_instruction: verify_instruction::Component,
    pub blake_context: BlakeContextComponents,
    pub builtins: BuiltinComponents,
    pub pedersen_context: PedersenContextComponents,
    pub poseidon_context: PoseidonContextComponents,
    pub memory_address_to_id: memory_address_to_id::Component,
    pub memory_id_to_value: (
        memory_id_to_big::BigComponent,
        memory_id_to_big::SmallComponent,
    ),
    pub range_checks: RangeChecksComponents,
    pub verify_bitwise_xor_4: verify_bitwise_xor_4::Component,
    pub verify_bitwise_xor_7: verify_bitwise_xor_7::Component,
    pub verify_bitwise_xor_8: verify_bitwise_xor_8::Component,
    pub verify_bitwise_xor_9: verify_bitwise_xor_9::Component,
    // ...
}
impl CairoComponents {
    pub fn new(
        cairo_claim: &CairoClaim,
        interaction_elements: &CairoInteractionElements,
        interaction_claim: &CairoInteractionClaim,
        // Describes the structure of the preprocessed trace. Sensitive to order.
        preprocessed_column_ids: &[PreProcessedColumnId],
    ) -> Self {
        let tree_span_provider =
            &mut TraceLocationAllocator::new_with_preproccessed_columns(preprocessed_column_ids);

        let opcode_components = OpcodeComponents::new(
            tree_span_provider,
            &cairo_claim.opcodes,
            interaction_elements,
            &interaction_claim.opcodes,
        );

        let verify_instruction_component = verify_instruction::Component::new(
            tree_span_provider,
            verify_instruction::Eval {
                claim: cairo_claim.verify_instruction,
                memory_address_to_id_lookup_elements: interaction_elements
                    .memory_address_to_id
                    .clone(),
                verify_instruction_lookup_elements: interaction_elements.verify_instruction.clone(),
                memory_id_to_big_lookup_elements: interaction_elements.memory_id_to_value.clone(),
                range_check_4_3_lookup_elements: interaction_elements.range_checks.rc_4_3.clone(),
                range_check_7_2_5_lookup_elements: interaction_elements
                    .range_checks
                    .rc_7_2_5
                    .clone(),
            },
            interaction_claim.verify_instruction.claimed_sum,
        );

        let blake_context = BlakeContextComponents::new(
            tree_span_provider,
            &cairo_claim.blake_context,
            interaction_elements,
            &interaction_claim.blake_context,
        );
        let builtin_components = BuiltinComponents::new(
            tree_span_provider,
            &cairo_claim.builtins,
            interaction_elements,
            &interaction_claim.builtins,
        );
        let pedersen_context = PedersenContextComponents::new(
            tree_span_provider,
            &cairo_claim.pedersen_context,
            interaction_elements,
            &interaction_claim.pedersen_context,
        );
        let poseidon_context = PoseidonContextComponents::new(
            tree_span_provider,
            &cairo_claim.poseidon_context,
            interaction_elements,
            &interaction_claim.poseidon_context,
        );
        let memory_address_to_id_component = memory_address_to_id::Component::new(
            tree_span_provider,
            memory_address_to_id::Eval::new(
                cairo_claim.memory_address_to_id.clone(),
                interaction_elements.memory_address_to_id.clone(),
            ),
            interaction_claim.memory_address_to_id.clone().claimed_sum,
        );

        let memory_id_to_value_component = memory_id_to_big::BigComponent::new(
            tree_span_provider,
            memory_id_to_big::BigEval::new(
                cairo_claim.memory_id_to_value.clone(),
                interaction_elements.memory_id_to_value.clone(),
                interaction_elements.range_checks.rc_9_9.clone(),
            ),
            interaction_claim.memory_id_to_value.clone().big_claimed_sum,
        );
        let small_memory_id_to_value_component = memory_id_to_big::SmallComponent::new(
            tree_span_provider,
            memory_id_to_big::SmallEval::new(
                cairo_claim.memory_id_to_value.clone(),
                interaction_elements.memory_id_to_value.clone(),
                interaction_elements.range_checks.rc_9_9.clone(),
            ),
            interaction_claim
                .memory_id_to_value
                .clone()
                .small_claimed_sum,
        );
        let range_checks_component = RangeChecksComponents::new(
            tree_span_provider,
            &interaction_elements.range_checks,
            &interaction_claim.range_checks,
        );
        let verify_bitwise_xor_4_component = verify_bitwise_xor_4::Component::new(
            tree_span_provider,
            verify_bitwise_xor_4::Eval {
                claim: cairo_claim.verify_bitwise_xor_4,
                verify_bitwise_xor_4_lookup_elements: interaction_elements
                    .verify_bitwise_xor_4
                    .clone(),
            },
            interaction_claim.verify_bitwise_xor_4.claimed_sum,
        );
        let verify_bitwise_xor_7_component = verify_bitwise_xor_7::Component::new(
            tree_span_provider,
            verify_bitwise_xor_7::Eval {
                claim: cairo_claim.verify_bitwise_xor_7,
                verify_bitwise_xor_7_lookup_elements: interaction_elements
                    .verify_bitwise_xor_7
                    .clone(),
            },
            interaction_claim.verify_bitwise_xor_7.claimed_sum,
        );
        let verify_bitwise_xor_8_component = verify_bitwise_xor_8::Component::new(
            tree_span_provider,
            verify_bitwise_xor_8::Eval {
                claim: cairo_claim.verify_bitwise_xor_8,
                verify_bitwise_xor_8_lookup_elements: interaction_elements
                    .verify_bitwise_xor_8
                    .clone(),
            },
            interaction_claim.verify_bitwise_xor_8.claimed_sum,
        );
        let verify_bitwise_xor_9_component = verify_bitwise_xor_9::Component::new(
            tree_span_provider,
            verify_bitwise_xor_9::Eval {
                claim: cairo_claim.verify_bitwise_xor_9,
                verify_bitwise_xor_9_lookup_elements: interaction_elements
                    .verify_bitwise_xor_9
                    .clone(),
            },
            interaction_claim.verify_bitwise_xor_9.claimed_sum,
        );
        Self {
            opcodes: opcode_components,
            verify_instruction: verify_instruction_component,
            blake_context,
            builtins: builtin_components,
            pedersen_context,
            poseidon_context,
            memory_address_to_id: memory_address_to_id_component,
            memory_id_to_value: (
                memory_id_to_value_component,
                small_memory_id_to_value_component,
            ),
            range_checks: range_checks_component,
            verify_bitwise_xor_4: verify_bitwise_xor_4_component,
            verify_bitwise_xor_7: verify_bitwise_xor_7_component,
            verify_bitwise_xor_8: verify_bitwise_xor_8_component,
            verify_bitwise_xor_9: verify_bitwise_xor_9_component,
        }
    }

    pub fn provers(&self) -> Vec<&dyn ComponentProver<SimdBackend>> {
        chain!(
            self.opcodes.provers(),
            [&self.verify_instruction as &dyn ComponentProver<SimdBackend>,],
            self.blake_context.provers(),
            self.builtins.provers(),
            self.pedersen_context.provers(),
            self.poseidon_context.provers(),
            [
                &self.memory_address_to_id as &dyn ComponentProver<SimdBackend>,
                &self.memory_id_to_value.0 as &dyn ComponentProver<SimdBackend>,
                &self.memory_id_to_value.1 as &dyn ComponentProver<SimdBackend>,
            ],
            self.range_checks.provers(),
            [
                &self.verify_bitwise_xor_4 as &dyn ComponentProver<SimdBackend>,
                &self.verify_bitwise_xor_7 as &dyn ComponentProver<SimdBackend>,
                &self.verify_bitwise_xor_8 as &dyn ComponentProver<SimdBackend>,
                &self.verify_bitwise_xor_9 as &dyn ComponentProver<SimdBackend>,
            ]
        )
        .collect()
    }

    pub fn components(&self) -> Vec<&dyn Component> {
        self.provers()
            .into_iter()
            .map(|component| component as &dyn Component)
            .collect()
    }
}

impl std::fmt::Display for CairoComponents {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "CairoComponents")?;
        writeln!(f, "Opcodes: {}", self.opcodes)?;
        writeln!(
            f,
            "VerifyInstruction: {}",
            indented_component_display(&self.verify_instruction)
        )?;
        writeln!(f, "BlakeContext: {}", self.blake_context)?;
        writeln!(f, "Builtins: {}", self.builtins)?;
        writeln!(f, "PedersenContext: {}", self.pedersen_context)?;
        writeln!(f, "PoseidonContext: {}", self.poseidon_context)?;
        writeln!(
            f,
            "MemoryAddressToId: {}",
            indented_component_display(&self.memory_address_to_id)
        )?;
        writeln!(
            f,
            "MemoryIdToValue: {}",
            indented_component_display(&self.memory_id_to_value.0)
        )?;
        writeln!(
            f,
            "SmallMemoryIdToValue: {}",
            indented_component_display(&self.memory_id_to_value.1)
        )?;
        writeln!(f, "RangeChecks: {}", self.range_checks)?;
        writeln!(
            f,
            "VerifyBitwiseXor4: {}",
            indented_component_display(&self.verify_bitwise_xor_4)
        )?;
        writeln!(
            f,
            "VerifyBitwiseXor7: {}",
            indented_component_display(&self.verify_bitwise_xor_7)
        )?;
        writeln!(
            f,
            "VerifyBitwiseXor8: {}",
            indented_component_display(&self.verify_bitwise_xor_8)
        )?;
        writeln!(
            f,
            "VerifyBitwiseXor9: {}",
            indented_component_display(&self.verify_bitwise_xor_9)
        )?;
        Ok(())
    }
}
