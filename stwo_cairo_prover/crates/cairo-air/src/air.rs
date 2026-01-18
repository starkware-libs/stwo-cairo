use std::collections::HashMap;
use std::iter::once;

use itertools::{chain, Itertools};
use serde::{Deserialize, Serialize};
use stwo::core::air::Component;
use stwo::core::channel::Channel;
use stwo::core::fields::m31::M31;
use stwo::core::fields::qm31::QM31;
use stwo::core::fields::FieldExpOps;
use stwo::core::proof::StarkProof;
use stwo::core::vcs::MerkleHasher;
use stwo::prover::backend::simd::SimdBackend;
use stwo::prover::ComponentProver;
use stwo_cairo_common::prover_types::cpu::CasmState;
use stwo_cairo_common::prover_types::felt::split_f252;
use stwo_cairo_serialize::{CairoDeserialize, CairoSerialize};
use stwo_constraint_framework::preprocessed_columns::PreProcessedColumnId;
use stwo_constraint_framework::{Relation, TraceLocationAllocator};

use super::blake::air::BlakeContextComponents;
use super::builtins_air::BuiltinComponents;
use super::components::indented_component_display;
use super::opcodes_air::OpcodeComponents;
use super::pedersen::air::PedersenContextComponents;
use super::poseidon::air::PoseidonContextComponents;
use super::range_checks_air::RangeChecksComponents;
use crate::claims::{CairoClaim, CairoInteractionClaim};
use crate::components::{
    memory_address_to_id, memory_id_to_big, verify_bitwise_xor_4, verify_bitwise_xor_7,
    verify_bitwise_xor_8, verify_bitwise_xor_9, verify_instruction,
};
use crate::relations::{
    self, CommonLookupElements, MEMORY_ADDRESS_TO_ID_RELATION_ID, MEMORY_ID_TO_BIG_RELATION_ID,
    OPCODES_RELATION_ID,
};
use crate::verifier::RelationUse;

#[derive(Serialize, Deserialize)]
pub struct CairoProof<H: MerkleHasher> {
    pub claim: CairoClaim,
    pub interaction_pow: u64,
    pub interaction_claim: CairoInteractionClaim,
    pub stark_proof: StarkProof<H>,
    /// Optional salt used in the channel initialization.
    pub channel_salt: Option<u64>,
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
            channel_salt,
        } = self;
        CairoSerialize::serialize(claim, output);
        CairoSerialize::serialize(interaction_pow, output);
        CairoSerialize::serialize(interaction_claim, output);
        CairoSerialize::serialize(stark_proof, output);
        CairoSerialize::serialize(channel_salt, output);
    }
}

impl<H: MerkleHasher> CairoDeserialize for CairoProof<H>
where
    H::Hash: CairoDeserialize,
{
    fn deserialize<'a>(data: &mut impl Iterator<Item = &'a starknet_ff::FieldElement>) -> Self {
        let claim = CairoDeserialize::deserialize(data);
        let interaction_pow = CairoDeserialize::deserialize(data);
        let interaction_claim = CairoDeserialize::deserialize(data);
        let stark_proof = CairoDeserialize::deserialize(data);
        let channel_salt = CairoDeserialize::deserialize(data);
        Self {
            claim,
            interaction_pow,
            interaction_claim,
            stark_proof,
            channel_salt,
        }
    }
}

pub type RelationUsesDict = HashMap<&'static str, u64>;

/// Accumulates the number of uses of each relation in a map.
pub fn accumulate_relation_uses<const N: usize>(
    relation_uses: &mut RelationUsesDict,
    relation_uses_per_row: [RelationUse; N],
    log_size: u32,
) {
    let component_size = 1 << log_size;
    for relation_use in relation_uses_per_row {
        let relation_uses_in_component = relation_use.uses.checked_mul(component_size).unwrap();
        let prev = relation_uses.entry(relation_use.relation_id).or_insert(0);
        *prev = prev.checked_add(relation_uses_in_component).unwrap();
    }
}

pub fn accumulate_relation_memory(
    relation_uses: &mut RelationUsesDict,
    claim: &Option<memory_id_to_big::Claim>,
) {
    let memory_id_to_value = claim.as_ref().expect("memory_id_to_value must be Some");

    // TODO(ShaharS): Look into the file name of memory_id_to_big.
    // memory_id_to_value has a big value component and a small value component.
    for log_size in &memory_id_to_value.big_log_sizes {
        accumulate_relation_uses(
            relation_uses,
            memory_id_to_big::RELATION_USES_PER_ROW_BIG,
            *log_size,
        );
    }
    accumulate_relation_uses(
        relation_uses,
        memory_id_to_big::RELATION_USES_PER_ROW_SMALL,
        memory_id_to_value.small_log_size,
    );
}

#[derive(Serialize, Deserialize, CairoSerialize, CairoDeserialize, Default)]
pub struct PublicData {
    pub public_memory: PublicMemory,
    pub initial_state: CasmState,
    pub final_state: CasmState,
}
impl PublicData {
    /// Sums the logup of the public data.
    pub fn logup_sum(&self, common_lookup_elements: &CommonLookupElements) -> QM31 {
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
                    <relations::CommonLookupElements as Relation<M31, QM31>>::combine(
                        common_lookup_elements,
                        &[
                            MEMORY_ADDRESS_TO_ID_RELATION_ID,
                            M31::from_u32_unchecked(addr),
                            M31::from_u32_unchecked(id),
                        ],
                    ),
                );
                values_to_inverse.push(
                    <relations::CommonLookupElements as Relation<M31, QM31>>::combine(
                        common_lookup_elements,
                        &[
                            [MEMORY_ID_TO_BIG_RELATION_ID, M31::from_u32_unchecked(id)].as_slice(),
                            split_f252(val).as_slice(),
                        ]
                        .concat(),
                    ),
                );
            });

        let final_state_tuple = once(OPCODES_RELATION_ID)
            .chain(self.final_state.values())
            .collect_vec();
        let initial_state_tuple = once(OPCODES_RELATION_ID)
            .chain(self.initial_state.values())
            .collect_vec();
        // Yield initial state and use the final.
        values_to_inverse.push(
            <relations::CommonLookupElements as Relation<M31, QM31>>::combine(
                common_lookup_elements,
                &final_state_tuple,
            ),
        );
        values_to_inverse.push(
            -<relations::CommonLookupElements as Relation<M31, QM31>>::combine(
                common_lookup_elements,
                &initial_state_tuple,
            ),
        );

        let inverted_values = QM31::batch_inverse(&values_to_inverse);
        inverted_values.iter().sum::<QM31>()
    }

    pub fn mix_into(&self, channel: &mut impl Channel) {
        let Self {
            public_memory,
            initial_state,
            final_state,
        } = self;
        public_memory.mix_into(channel);
        initial_state.mix_into(channel);
        final_state.mix_into(channel);
    }
}

// TODO(alonf) Change all the obscure types and structs to a meaningful struct system for the
// memory.
#[derive(Clone, Debug, Serialize, Deserialize, Copy, CairoSerialize, CairoDeserialize, Default)]
pub struct MemorySmallValue {
    pub id: u32,
    pub value: u32,
}
impl MemorySmallValue {
    pub fn mix_into(&self, channel: &mut impl Channel) {
        channel.mix_u64(self.id as u64);
        channel.mix_u64(self.value as u64);
    }
}

// TODO(alonf): Change this into a struct. Remove Pub prefix.
// (id, value)
pub type PubMemoryValue = (u32, [u32; 8]);

// TODO(alonf): Change this into a struct. Remove Pub prefix.
// (address, id, value)
pub type PubMemoryEntry = (u32, u32, [u32; 8]);

#[derive(Clone, Debug, Serialize, Deserialize, Copy, CairoSerialize, CairoDeserialize, Default)]
pub struct SegmentRange {
    pub start_ptr: MemorySmallValue,
    pub stop_ptr: MemorySmallValue,
}

impl SegmentRange {
    pub fn is_empty(&self) -> bool {
        self.start_ptr.value == self.stop_ptr.value
    }

    pub fn mix_into(&self, channel: &mut impl Channel) {
        self.start_ptr.mix_into(channel);
        self.stop_ptr.mix_into(channel);
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Copy, Default)]
pub struct PublicSegmentRanges {
    pub output: SegmentRange,
    pub pedersen: Option<SegmentRange>,
    pub range_check_128: Option<SegmentRange>,
    pub ecdsa: Option<SegmentRange>,
    pub bitwise: Option<SegmentRange>,
    pub ec_op: Option<SegmentRange>,
    pub keccak: Option<SegmentRange>,
    pub poseidon: Option<SegmentRange>,
    pub range_check_96: Option<SegmentRange>,
    pub add_mod: Option<SegmentRange>,
    pub mul_mod: Option<SegmentRange>,
}

/// Same as PublicSegmentRanges, but with all segments present, this serialization of the struct is
/// used by the Cairo1 verifier.
#[derive(Clone, Debug, Serialize, Deserialize, Copy, CairoSerialize, CairoDeserialize)]
pub struct FullSegmentRanges {
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

// The Cairo1 verifier currently requires all the segments to be present.
impl CairoSerialize for PublicSegmentRanges {
    fn serialize(&self, serialized: &mut Vec<starknet_ff::FieldElement>) {
        let Self {
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
        } = self;

        CairoSerialize::serialize(
            &FullSegmentRanges {
                output: *output,
                pedersen: pedersen.unwrap(),
                range_check_128: range_check_128.unwrap(),
                ecdsa: ecdsa.unwrap(),
                bitwise: bitwise.unwrap(),
                ec_op: ec_op.unwrap(),
                keccak: keccak.unwrap(),
                poseidon: poseidon.unwrap(),
                range_check_96: range_check_96.unwrap(),
                add_mod: add_mod.unwrap(),
                mul_mod: mul_mod.unwrap(),
            },
            serialized,
        );
    }
}

impl CairoDeserialize for PublicSegmentRanges {
    fn deserialize<'a>(data: &mut impl Iterator<Item = &'a starknet_ff::FieldElement>) -> Self {
        let FullSegmentRanges {
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
        } = CairoDeserialize::deserialize(data);

        Self {
            output,
            pedersen: Some(pedersen),
            range_check_128: Some(range_check_128),
            ecdsa: Some(ecdsa),
            bitwise: Some(bitwise),
            ec_op: Some(ec_op),
            keccak: Some(keccak),
            poseidon: Some(poseidon),
            range_check_96: Some(range_check_96),
            add_mod: Some(add_mod),
            mul_mod: Some(mul_mod),
        }
    }
}

impl PublicSegmentRanges {
    pub fn memory_entries(
        &self,
        initial_ap: u32,
        final_ap: u32,
    ) -> impl Iterator<Item = PubMemoryEntry> {
        let segments = self.present_segments();

        let n_segments = segments.len() as u32;

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

    pub fn mix_into(&self, channel: &mut impl Channel) {
        for segment in self.present_segments() {
            segment.mix_into(channel);
        }
    }

    pub fn present_segments(&self) -> Vec<SegmentRange> {
        let Self {
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
        vec![
            Some(output),
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
        .flatten()
        .collect_vec()
    }
}

pub type MemorySection = Vec<PubMemoryValue>;

#[derive(Serialize, Deserialize, CairoSerialize, CairoDeserialize, Default)]
pub struct PublicMemory {
    pub program: MemorySection,
    pub public_segments: PublicSegmentRanges,
    pub output: MemorySection,
    pub safe_call_ids: [u32; 2],
}

impl PublicMemory {
    /// Returns [`PubMemoryEntry`] for all public memory.
    pub fn get_entries(
        &self,
        initial_pc: u32,
        initial_ap: u32,
        final_ap: u32,
    ) -> impl Iterator<Item = PubMemoryEntry> {
        let [program, output] =
            [&self.program, &self.output].map(|section| section.clone().into_iter().enumerate());
        let program_iter = program.map(move |(i, (id, value))| (initial_pc + i as u32, id, value));
        let output_iter = output.map(move |(i, (id, value))| (final_ap + i as u32, id, value));

        let [safe_call_id0, safe_call_id1] = self.safe_call_ids;
        // The safe call area should be [initial_fp, 0] and initial_fp should be the same as
        // initial_ap.
        let safe_call_iter = [
            (
                initial_ap - 2,
                safe_call_id0,
                [initial_ap, 0, 0, 0, 0, 0, 0, 0],
            ),
            (initial_ap - 1, safe_call_id1, [0, 0, 0, 0, 0, 0, 0, 0]),
        ];
        let segment_ranges_iter = self.public_segments.memory_entries(initial_ap, final_ap);

        program_iter
            .chain(safe_call_iter)
            .chain(segment_ranges_iter)
            .chain(output_iter)
    }

    pub fn mix_into(&self, channel: &mut impl Channel) {
        let Self {
            program,
            public_segments,
            output,
            safe_call_ids,
        } = self;

        // Mix program memory section. All the ids are mixed first, then all the values, each of
        // them in the order it appears in the section.
        channel.mix_u32s(&program.iter().map(|(id, _)| *id).collect_vec());
        channel.mix_u32s(&program.iter().flat_map(|(_, value)| *value).collect_vec());

        // Mix public segments.
        public_segments.mix_into(channel);

        // Mix output memory section. All the ids are mixed first, then all the values, each of them
        // in the order it appears in the section.
        channel.mix_u32s(&output.iter().map(|(id, _)| *id).collect_vec());
        channel.mix_u32s(&output.iter().flat_map(|(_, value)| *value).collect_vec());

        // Mix safe_ids memory section.
        for id in safe_call_ids {
            channel.mix_u64(*id as u64);
        }
    }
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
        Vec<memory_id_to_big::BigComponent>,
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
        common_lookup_elements: &CommonLookupElements,
        interaction_claim: &CairoInteractionClaim,
        // Describes the structure of the preprocessed trace. Sensitive to order.
        preprocessed_column_ids: &[PreProcessedColumnId],
    ) -> Self {
        let tree_span_provider =
            &mut TraceLocationAllocator::new_with_preprocessed_columns(preprocessed_column_ids);

        let opcode_components = OpcodeComponents::new(
            tree_span_provider,
            &cairo_claim.add_opcode,
            &cairo_claim.add_opcode_small,
            &cairo_claim.add_ap_opcode,
            &cairo_claim.assert_eq_opcode,
            &cairo_claim.assert_eq_opcode_imm,
            &cairo_claim.assert_eq_opcode_double_deref,
            &cairo_claim.blake_compress_opcode,
            &cairo_claim.call_opcode_abs,
            &cairo_claim.call_opcode_rel_imm,
            &cairo_claim.generic_opcode,
            &cairo_claim.jnz_opcode_non_taken,
            &cairo_claim.jnz_opcode_taken,
            &cairo_claim.jump_opcode_abs,
            &cairo_claim.jump_opcode_double_deref,
            &cairo_claim.jump_opcode_rel,
            &cairo_claim.jump_opcode_rel_imm,
            &cairo_claim.mul_opcode,
            &cairo_claim.mul_opcode_small,
            &cairo_claim.qm_31_add_mul_opcode,
            &cairo_claim.ret_opcode,
            common_lookup_elements,
            &interaction_claim.add_opcode,
            &interaction_claim.add_opcode_small,
            &interaction_claim.add_ap_opcode,
            &interaction_claim.assert_eq_opcode,
            &interaction_claim.assert_eq_opcode_imm,
            &interaction_claim.assert_eq_opcode_double_deref,
            &interaction_claim.blake_compress_opcode,
            &interaction_claim.call_opcode_abs,
            &interaction_claim.call_opcode_rel_imm,
            &interaction_claim.generic_opcode,
            &interaction_claim.jnz_opcode_non_taken,
            &interaction_claim.jnz_opcode_taken,
            &interaction_claim.jump_opcode_abs,
            &interaction_claim.jump_opcode_double_deref,
            &interaction_claim.jump_opcode_rel,
            &interaction_claim.jump_opcode_rel_imm,
            &interaction_claim.mul_opcode,
            &interaction_claim.mul_opcode_small,
            &interaction_claim.qm_31_add_mul_opcode,
            &interaction_claim.ret_opcode,
        );

        let verify_instruction_component = verify_instruction::Component::new(
            tree_span_provider,
            verify_instruction::Eval {
                claim: cairo_claim.verify_instruction.unwrap(),
                common_lookup_elements: common_lookup_elements.clone(),
            },
            interaction_claim.verify_instruction.unwrap().claimed_sum,
        );

        let blake_context = BlakeContextComponents::new(
            tree_span_provider,
            &cairo_claim.blake_round,
            &cairo_claim.blake_g,
            &cairo_claim.blake_round_sigma,
            &cairo_claim.triple_xor_32,
            &cairo_claim.verify_bitwise_xor_12,
            common_lookup_elements,
            &interaction_claim.blake_round,
            &interaction_claim.blake_g,
            &interaction_claim.blake_round_sigma,
            &interaction_claim.triple_xor_32,
            &interaction_claim.verify_bitwise_xor_12,
        );
        let builtin_components = BuiltinComponents::new(
            tree_span_provider,
            &cairo_claim.add_mod_builtin,
            &cairo_claim.bitwise_builtin,
            &cairo_claim.mul_mod_builtin,
            &cairo_claim.pedersen_builtin,
            &cairo_claim.poseidon_builtin,
            &cairo_claim.range_check96_builtin,
            &cairo_claim.range_check_builtin,
            common_lookup_elements,
            &interaction_claim.add_mod_builtin,
            &interaction_claim.bitwise_builtin,
            &interaction_claim.mul_mod_builtin,
            &interaction_claim.pedersen_builtin,
            &interaction_claim.poseidon_builtin,
            &interaction_claim.range_check96_builtin,
            &interaction_claim.range_check_builtin,
        );
        let pedersen_context = PedersenContextComponents::new(
            tree_span_provider,
            &cairo_claim.pedersen_aggregator_window_bits_18,
            &cairo_claim.partial_ec_mul_window_bits_18,
            &cairo_claim.pedersen_points_table_window_bits_18,
            common_lookup_elements,
            &interaction_claim.pedersen_aggregator_window_bits_18,
            &interaction_claim.partial_ec_mul_window_bits_18,
            &interaction_claim.pedersen_points_table_window_bits_18,
        );
        let poseidon_context = PoseidonContextComponents::new(
            tree_span_provider,
            &cairo_claim.poseidon_aggregator,
            &cairo_claim.poseidon_3_partial_rounds_chain,
            &cairo_claim.poseidon_full_round_chain,
            &cairo_claim.cube_252,
            &cairo_claim.poseidon_round_keys,
            &cairo_claim.range_check_252_width_27,
            common_lookup_elements,
            &interaction_claim.poseidon_aggregator,
            &interaction_claim.poseidon_3_partial_rounds_chain,
            &interaction_claim.poseidon_full_round_chain,
            &interaction_claim.cube_252,
            &interaction_claim.poseidon_round_keys,
            &interaction_claim.range_check_252_width_27,
        );
        let memory_address_to_id_component = memory_address_to_id::Component::new(
            tree_span_provider,
            memory_address_to_id::Eval::new(
                cairo_claim.memory_address_to_id.as_ref().unwrap(),
                common_lookup_elements.clone(),
            ),
            interaction_claim.memory_address_to_id.unwrap().claimed_sum,
        );

        let memory_id_to_value_components = memory_id_to_big::big_components_from_claim(
            &cairo_claim.memory_id_to_big.as_ref().unwrap().big_log_sizes,
            &interaction_claim
                .memory_id_to_big
                .as_ref()
                .unwrap()
                .big_claimed_sums,
            &common_lookup_elements.clone(),
            tree_span_provider,
        );
        let small_memory_id_to_value_component = memory_id_to_big::SmallComponent::new(
            tree_span_provider,
            memory_id_to_big::SmallEval::new(
                cairo_claim.memory_id_to_big.as_ref().unwrap(),
                common_lookup_elements.clone(),
            ),
            interaction_claim
                .memory_id_to_big
                .as_ref()
                .unwrap()
                .small_claimed_sum,
        );
        let range_checks_component = RangeChecksComponents::new(
            tree_span_provider,
            common_lookup_elements,
            &interaction_claim.range_check_6,
            &interaction_claim.range_check_8,
            &interaction_claim.range_check_11,
            &interaction_claim.range_check_12,
            &interaction_claim.range_check_18,
            &interaction_claim.range_check_20,
            &interaction_claim.range_check_4_3,
            &interaction_claim.range_check_4_4,
            &interaction_claim.range_check_9_9,
            &interaction_claim.range_check_7_2_5,
            &interaction_claim.range_check_3_6_6_3,
            &interaction_claim.range_check_4_4_4_4,
            &interaction_claim.range_check_3_3_3_3_3,
        );
        let verify_bitwise_xor_4_component = verify_bitwise_xor_4::Component::new(
            tree_span_provider,
            verify_bitwise_xor_4::Eval {
                claim: cairo_claim.verify_bitwise_xor_4.unwrap(),
                common_lookup_elements: common_lookup_elements.clone(),
            },
            interaction_claim.verify_bitwise_xor_4.unwrap().claimed_sum,
        );
        let verify_bitwise_xor_7_component = verify_bitwise_xor_7::Component::new(
            tree_span_provider,
            verify_bitwise_xor_7::Eval {
                claim: cairo_claim.verify_bitwise_xor_7.unwrap(),
                common_lookup_elements: common_lookup_elements.clone(),
            },
            interaction_claim.verify_bitwise_xor_7.unwrap().claimed_sum,
        );
        let verify_bitwise_xor_8_component = verify_bitwise_xor_8::Component::new(
            tree_span_provider,
            verify_bitwise_xor_8::Eval {
                claim: cairo_claim.verify_bitwise_xor_8.unwrap(),
                common_lookup_elements: common_lookup_elements.clone(),
            },
            interaction_claim.verify_bitwise_xor_8.unwrap().claimed_sum,
        );
        let verify_bitwise_xor_9_component = verify_bitwise_xor_9::Component::new(
            tree_span_provider,
            verify_bitwise_xor_9::Eval {
                claim: cairo_claim.verify_bitwise_xor_9.unwrap(),
                common_lookup_elements: common_lookup_elements.clone(),
            },
            interaction_claim.verify_bitwise_xor_9.unwrap().claimed_sum,
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
                memory_id_to_value_components,
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
            [&self.memory_address_to_id as &dyn ComponentProver<SimdBackend>,],
            self.memory_id_to_value
                .0
                .iter()
                .map(|component| component as &dyn ComponentProver<SimdBackend>),
            [&self.memory_id_to_value.1 as &dyn ComponentProver<SimdBackend>,],
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
        for component in &self.memory_id_to_value.0 {
            writeln!(
                f,
                "MemoryIdToValue: {}",
                indented_component_display(component)
            )?;
        }
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

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::air::accumulate_relation_uses;
    use crate::verifier::RelationUse;

    #[test]
    fn test_accumulate_relation_uses() {
        let mut relation_uses = HashMap::from([("relation_1", 4), ("relation_2", 10)]);
        let log_size = 2;
        let relation_uses_per_row = [
            RelationUse {
                relation_id: "relation_1",
                uses: 2,
            },
            RelationUse {
                relation_id: "relation_2",
                uses: 4,
            },
        ];

        accumulate_relation_uses(&mut relation_uses, relation_uses_per_row, log_size);

        assert_eq!(relation_uses.len(), 2);
        assert_eq!(relation_uses.get("relation_1"), Some(&12));
        assert_eq!(relation_uses.get("relation_2"), Some(&26));
    }
}
