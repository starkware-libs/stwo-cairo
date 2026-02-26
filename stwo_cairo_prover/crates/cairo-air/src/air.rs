use std::collections::HashMap;
use std::iter::once;

use itertools::{chain, Itertools};
use serde::{Deserialize, Serialize};
use stwo::core::air::Component;
use stwo::core::channel::{Channel, MerkleChannel};
use stwo::core::fields::m31::M31;
use stwo::core::fields::qm31::QM31;
use stwo::core::fields::FieldExpOps;
use stwo::core::proof::{ExtendedStarkProof, StarkProof};
use stwo::core::vcs_lifted::MerkleHasherLifted;
use stwo_cairo_common::prover_types::cpu::{CasmState, FELT252_BITS_PER_WORD, FELT252_N_WORDS};
use stwo_cairo_common::prover_types::felt::{split, split_f252};
use stwo_cairo_serialize::{CairoDeserialize, CairoSerialize};
use stwo_constraint_framework::preprocessed_columns::PreProcessedColumnId;
use stwo_constraint_framework::{Relation, TraceLocationAllocator};

use super::blake::air::BlakeContextComponents;
use super::builtins_air::BuiltinComponents;
use super::components::indented_component_display;
use super::opcodes_air::OpcodeComponents;
use super::pedersen::air::PedersenContextComponents;
use super::pedersen_narrow_windows::air::PedersenContextComponents as PedersenNarrowWindowsContextComponents;
use super::poseidon::air::PoseidonContextComponents;
use super::range_checks_air::RangeChecksComponents;
use crate::claims::{CairoClaim, CairoInteractionClaim};
use crate::components::{
    memory_address_to_id, memory_id_to_big, memory_id_to_small, verify_bitwise_xor_4,
    verify_bitwise_xor_7, verify_bitwise_xor_8, verify_bitwise_xor_9, verify_instruction,
};
use crate::relations::{
    self, CommonLookupElements, MEMORY_ADDRESS_TO_ID_RELATION_ID, MEMORY_ID_TO_BIG_RELATION_ID,
    OPCODES_RELATION_ID,
};
use crate::utils::pack_into_secure_felts;
use crate::verifier::RelationUse;
use crate::PreProcessedTraceVariant;

/// The canonical proof format emitted by the Cairo prover.
///
/// This is the main proof struct that contains all the data of a Cairo proof. It serves as the
/// universal representation from which verifier-specific formats can be derived.
///
/// # Verifier Integration
///
/// Each verifier implementation should:
/// 1. Implement a conversion from `CairoProof` to its specific format (typically via the [`From`]
///    trait).
/// 2. Implement appropriate serialization for proof transport/storage.
///
/// # Available Verifier Formats
///
/// - **Rust verifier**: See [`CairoProofForRustVerifier`] - conversion via `From` trait, uses serde
///   for JSON serialization or bincode for binary serialization.
/// - **Cairo verifier**: Serialization via [`CairoSerialize`](stwo_cairo_serialize::CairoSerialize)
///   (see `serde_utils.rs`), which transforms the proof into a format compatible with the Cairo1
///   verifier.
#[derive(Clone, Serialize, Deserialize)]
pub struct CairoProof<H: MerkleHasherLifted> {
    pub claim: CairoClaim,
    pub interaction_pow: u64,
    pub interaction_claim: CairoInteractionClaim,
    pub extended_stark_proof: ExtendedStarkProof<H>,
    /// Salt used in the channel initialization.
    pub channel_salt: u32,
    pub preprocessed_trace_variant: PreProcessedTraceVariant,
}

/// Proof format optimized for the Rust verifier.
///
/// This struct contains the proof data in a format tailored to the Rust verifier's requirements.
///
/// The key difference from [`CairoProof`] is that this format uses [`StarkProof`] instead of
/// [`ExtendedStarkProof`], discarding auxiliary data not needed by the Rust verifier.
#[derive(Clone, Serialize, Deserialize)]
pub struct CairoProofForRustVerifier<H: MerkleHasherLifted> {
    pub claim: CairoClaim,
    pub interaction_pow: u64,
    pub interaction_claim: CairoInteractionClaim,
    pub stark_proof: StarkProof<H>,
    /// Salt used in the channel initialization.
    pub channel_salt: u32,
    pub preprocessed_trace_variant: PreProcessedTraceVariant,
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

#[derive(Serialize, Deserialize, CairoSerialize, CairoDeserialize, Default, Clone)]
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

    pub fn mix_into<MC: MerkleChannel>(&self, channel: &mut MC::C) {
        let (public_claim, output_claim, program_claim) = self.pack_into_u32s();
        channel.mix_felts(&pack_into_secure_felts(public_claim.into_iter()));
        let mut hasher = MC::H::default();
        hasher.update_leaf(
            output_claim
                .iter()
                .map(|x| M31::from_u32_unchecked(*x))
                .collect::<Vec<_>>()
                .as_slice(),
        );
        MC::mix_root(channel, hasher.finalize());

        let mut hasher = MC::H::default();
        hasher.update_leaf(
            program_claim
                .iter()
                .map(|x| M31::from_u32_unchecked(*x))
                .collect::<Vec<_>>()
                .as_slice(),
        );
        MC::mix_root(channel, hasher.finalize());
    }

    /// Converts public data to [u32], where each u32 is at most 2^31 - 1.
    /// Returns the output and program values separately.
    pub fn pack_into_u32s(&self) -> (Vec<u32>, Vec<u32>, Vec<u32>) {
        let PublicData {
            initial_state:
                CasmState {
                    pc: initial_pc,
                    ap: initial_ap,
                    fp: initial_fp,
                },
            final_state:
                CasmState {
                    pc: final_pc,
                    ap: final_ap,
                    fp: final_fp,
                },
            public_memory:
                PublicMemory {
                    public_segments,
                    output,
                    safe_call_ids,
                    program,
                },
        } = self;

        let mut public_claim = vec![
            initial_pc.0,
            initial_ap.0,
            initial_fp.0,
            final_pc.0,
            final_ap.0,
            final_fp.0,
        ];
        let PublicSegmentRanges {
            output: output_ranges,
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
        } = public_segments;
        Self::single_segment_range(Some(*output_ranges), &mut public_claim);
        Self::single_segment_range(*pedersen, &mut public_claim);
        Self::single_segment_range(*range_check_128, &mut public_claim);
        Self::single_segment_range(*ecdsa, &mut public_claim);
        Self::single_segment_range(*bitwise, &mut public_claim);
        Self::single_segment_range(*ec_op, &mut public_claim);
        Self::single_segment_range(*keccak, &mut public_claim);
        Self::single_segment_range(*poseidon, &mut public_claim);
        Self::single_segment_range(*range_check_96, &mut public_claim);
        Self::single_segment_range(*add_mod, &mut public_claim);
        Self::single_segment_range(*mul_mod, &mut public_claim);
        public_claim.extend(safe_call_ids);
        for (id, _) in output {
            public_claim.push(*id);
        }
        for (id, _) in program {
            public_claim.push(*id);
        }

        // Collect output values.
        let mut output_claim = vec![];
        for (_, value) in output {
            output_claim
                .extend::<[u32; FELT252_N_WORDS]>(split(*value, (1 << FELT252_BITS_PER_WORD) - 1));
        }

        // Collect program values.
        let mut program_claim = vec![];
        for (_, value) in program {
            program_claim
                .extend::<[u32; FELT252_N_WORDS]>(split(*value, (1 << FELT252_BITS_PER_WORD) - 1));
        }

        (public_claim, output_claim, program_claim)
    }

    fn single_segment_range(segment: Option<SegmentRange>, public_claim: &mut Vec<u32>) {
        if let Some(segment) = segment {
            public_claim.extend([
                segment.start_ptr.id,
                segment.start_ptr.value,
                segment.stop_ptr.id,
                segment.stop_ptr.value,
            ]);
        } else {
            public_claim.extend([0_u32; 4]);
        }
    }
}

// TODO(alonf) Change all the obscure types and structs to a meaningful struct system for the
// memory.
#[derive(Clone, Debug, Serialize, Deserialize, Copy, CairoSerialize, CairoDeserialize, Default)]
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

#[derive(Clone, Debug, Serialize, Deserialize, Copy, CairoSerialize, CairoDeserialize, Default)]
pub struct SegmentRange {
    pub start_ptr: MemorySmallValue,
    pub stop_ptr: MemorySmallValue,
}

impl SegmentRange {
    pub fn is_empty(&self) -> bool {
        self.start_ptr.value == self.stop_ptr.value
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

#[derive(Serialize, Deserialize, CairoSerialize, CairoDeserialize, Default, Clone)]
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
}

pub struct CairoComponents {
    pub opcodes: OpcodeComponents,
    pub verify_instruction: verify_instruction::Component,
    pub blake_context: BlakeContextComponents,
    pub builtins: BuiltinComponents,
    pub pedersen_context: PedersenContextComponents,
    pub pedersen_narrow_windows_context: PedersenNarrowWindowsContextComponents,
    pub poseidon_context: PoseidonContextComponents,
    pub memory_address_to_id: memory_address_to_id::Component,
    pub memory_id_to_big: Vec<memory_id_to_big::BigComponent>,
    pub memory_id_to_small: memory_id_to_small::Component,
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
            cairo_claim,
            common_lookup_elements,
            interaction_claim,
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
            cairo_claim,
            common_lookup_elements,
            interaction_claim,
        );
        let builtin_components = BuiltinComponents::new(
            tree_span_provider,
            cairo_claim,
            common_lookup_elements,
            interaction_claim,
        );
        let pedersen_context = PedersenContextComponents::new(
            tree_span_provider,
            cairo_claim,
            common_lookup_elements,
            interaction_claim,
        );
        let pedersen_narrow_windows_context = PedersenNarrowWindowsContextComponents::new(
            tree_span_provider,
            cairo_claim,
            common_lookup_elements,
            interaction_claim,
        );
        let poseidon_context = PoseidonContextComponents::new(
            tree_span_provider,
            cairo_claim,
            common_lookup_elements,
            interaction_claim,
        );
        let memory_address_to_id_component = memory_address_to_id::Component::new(
            tree_span_provider,
            memory_address_to_id::Eval::new(
                cairo_claim.memory_address_to_id.as_ref().unwrap(),
                common_lookup_elements.clone(),
            ),
            interaction_claim.memory_address_to_id.unwrap().claimed_sum,
        );

        let memory_id_to_big_components = memory_id_to_big::big_components_from_claim(
            &cairo_claim.memory_id_to_big.as_ref().unwrap().big_log_sizes,
            &interaction_claim
                .memory_id_to_big
                .as_ref()
                .unwrap()
                .big_claimed_sums,
            &common_lookup_elements.clone(),
            tree_span_provider,
        );
        let memory_id_to_small_component = memory_id_to_small::Component::new(
            tree_span_provider,
            memory_id_to_small::Eval {
                claim: cairo_claim.memory_id_to_small.unwrap(),
                common_lookup_elements: common_lookup_elements.clone(),
            },
            interaction_claim.memory_id_to_small.unwrap().claimed_sum,
        );
        let range_checks_component = RangeChecksComponents::new(
            tree_span_provider,
            common_lookup_elements,
            interaction_claim,
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
            pedersen_narrow_windows_context,
            poseidon_context,
            memory_address_to_id: memory_address_to_id_component,
            memory_id_to_big: memory_id_to_big_components,
            memory_id_to_small: memory_id_to_small_component,
            range_checks: range_checks_component,
            verify_bitwise_xor_4: verify_bitwise_xor_4_component,
            verify_bitwise_xor_7: verify_bitwise_xor_7_component,
            verify_bitwise_xor_8: verify_bitwise_xor_8_component,
            verify_bitwise_xor_9: verify_bitwise_xor_9_component,
        }
    }

    pub fn components(&self) -> Vec<&dyn Component> {
        chain!(
            self.opcodes.components(),
            [&self.verify_instruction as &dyn Component,],
            self.blake_context.components(),
            self.builtins.components(),
            self.pedersen_context.components(),
            self.pedersen_narrow_windows_context.components(),
            self.poseidon_context.components(),
            [&self.memory_address_to_id as &dyn Component,],
            self.memory_id_to_big
                .iter()
                .map(|component| component as &dyn Component),
            [&self.memory_id_to_small as &dyn Component,],
            self.range_checks.components(),
            [
                &self.verify_bitwise_xor_4 as &dyn Component,
                &self.verify_bitwise_xor_7 as &dyn Component,
                &self.verify_bitwise_xor_8 as &dyn Component,
                &self.verify_bitwise_xor_9 as &dyn Component,
            ]
        )
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
        writeln!(f, "PedersenNarrowWindowsContext: {}", self.pedersen_context)?;
        writeln!(f, "PoseidonContext: {}", self.poseidon_context)?;
        writeln!(
            f,
            "MemoryAddressToId: {}",
            indented_component_display(&self.memory_address_to_id)
        )?;
        for component in &self.memory_id_to_big {
            writeln!(
                f,
                "MemoryIdToValue: {}",
                indented_component_display(component)
            )?;
        }
        writeln!(
            f,
            "SmallMemoryIdToValue: {}",
            indented_component_display(&self.memory_id_to_small)
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

impl<H: MerkleHasherLifted> From<CairoProof<H>> for CairoProofForRustVerifier<H> {
    fn from(extended_cairo_proof: CairoProof<H>) -> Self {
        let CairoProof {
            claim,
            interaction_pow,
            interaction_claim,
            extended_stark_proof,
            channel_salt,
            preprocessed_trace_variant,
        } = extended_cairo_proof;

        let ExtendedStarkProof { proof, .. } = extended_stark_proof;

        Self {
            claim,
            interaction_pow,
            interaction_claim,
            stark_proof: proof,
            channel_salt,
            preprocessed_trace_variant,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use stwo::core::fields::cm31::CM31;
    use stwo::core::fields::m31::M31;
    use stwo::core::fields::qm31::QM31;
    use stwo_cairo_common::prover_types::cpu::CasmState;

    use crate::air::{
        accumulate_relation_uses, MemorySmallValue, PubMemoryValue, PublicData, PublicMemory,
        PublicSegmentRanges, SegmentRange,
    };
    use crate::relations::CommonLookupElements;
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

    #[test]
    fn test_public_data_logup_sum() {
        let program: Vec<PubMemoryValue> = vec![
            (0, [2147450879, 67600385, 0, 0, 0, 0, 0, 0]),
            (1, [11, 0, 0, 0, 0, 0, 0, 0]),
            (2, [2147581952, 285507585, 0, 0, 0, 0, 0, 0]),
            (3, [4, 0, 0, 0, 0, 0, 0, 0]),
            (4, [2147450879, 17268737, 0, 0, 0, 0, 0, 0]),
            (5, [0, 0, 0, 0, 0, 0, 0, 0]),
            (6, [2147450880, 1208647667, 0, 0, 0, 0, 0, 0]),
            (7, [2147450880, 1208647668, 0, 0, 0, 0, 0, 0]),
            (8, [2147450880, 1208647669, 0, 0, 0, 0, 0, 0]),
            (9, [2147450880, 1208647670, 0, 0, 0, 0, 0, 0]),
            (10, [2147450880, 1208647671, 0, 0, 0, 0, 0, 0]),
            (11, [2147450880, 1208647672, 0, 0, 0, 0, 0, 0]),
            (12, [2147450880, 1208647673, 0, 0, 0, 0, 0, 0]),
            (13, [2147450880, 1208647674, 0, 0, 0, 0, 0, 0]),
            (14, [2147450880, 1208647675, 0, 0, 0, 0, 0, 0]),
            (15, [2147450880, 1208647676, 0, 0, 0, 0, 0, 0]),
            (16, [2147450880, 1208647677, 0, 0, 0, 0, 0, 0]),
            (17, [2147450878, 546013183, 0, 0, 0, 0, 0, 0]),
        ];

        let dummy_lookup_elements = CommonLookupElements::dummy();
        let public_data = PublicData {
            public_memory: PublicMemory {
                program,
                public_segments: PublicSegmentRanges {
                    output: SegmentRange {
                        start_ptr: MemorySmallValue {
                            id: 228,
                            value: 2520,
                        },
                        stop_ptr: MemorySmallValue {
                            id: 228,
                            value: 2520,
                        },
                    },
                    pedersen: Some(SegmentRange {
                        start_ptr: MemorySmallValue {
                            id: 228,
                            value: 2520,
                        },
                        stop_ptr: MemorySmallValue {
                            id: 228,
                            value: 2520,
                        },
                    }),
                    range_check_128: Some(SegmentRange {
                        start_ptr: MemorySmallValue {
                            id: 228,
                            value: 2520,
                        },
                        stop_ptr: MemorySmallValue {
                            id: 228,
                            value: 2520,
                        },
                    }),
                    ecdsa: Some(SegmentRange {
                        start_ptr: MemorySmallValue { id: 5, value: 0 },
                        stop_ptr: MemorySmallValue { id: 5, value: 0 },
                    }),
                    bitwise: Some(SegmentRange {
                        start_ptr: MemorySmallValue {
                            id: 228,
                            value: 2520,
                        },
                        stop_ptr: MemorySmallValue {
                            id: 228,
                            value: 2520,
                        },
                    }),
                    ec_op: Some(SegmentRange {
                        start_ptr: MemorySmallValue { id: 5, value: 0 },
                        stop_ptr: MemorySmallValue { id: 5, value: 0 },
                    }),
                    keccak: Some(SegmentRange {
                        start_ptr: MemorySmallValue { id: 5, value: 0 },
                        stop_ptr: MemorySmallValue { id: 5, value: 0 },
                    }),
                    poseidon: Some(SegmentRange {
                        start_ptr: MemorySmallValue {
                            id: 228,
                            value: 2520,
                        },
                        stop_ptr: MemorySmallValue {
                            id: 228,
                            value: 2520,
                        },
                    }),
                    range_check_96: Some(SegmentRange {
                        start_ptr: MemorySmallValue {
                            id: 228,
                            value: 2520,
                        },
                        stop_ptr: MemorySmallValue {
                            id: 228,
                            value: 2520,
                        },
                    }),
                    add_mod: Some(SegmentRange {
                        start_ptr: MemorySmallValue {
                            id: 228,
                            value: 2520,
                        },
                        stop_ptr: MemorySmallValue {
                            id: 228,
                            value: 2520,
                        },
                    }),
                    mul_mod: Some(SegmentRange {
                        start_ptr: MemorySmallValue {
                            id: 228,
                            value: 2520,
                        },
                        stop_ptr: MemorySmallValue {
                            id: 228,
                            value: 2520,
                        },
                    }),
                },
                output: vec![],
                safe_call_ids: [227, 5],
            },
            initial_state: CasmState {
                pc: M31::from_u32_unchecked(1),
                ap: M31::from_u32_unchecked(1336),
                fp: M31::from_u32_unchecked(1336),
            },
            final_state: CasmState {
                pc: M31::from_u32_unchecked(5),
                ap: M31::from_u32_unchecked(2520),
                fp: M31::from_u32_unchecked(1336),
            },
        };

        let sum = public_data.logup_sum(&dummy_lookup_elements);

        // Expected value with the new program data:

        let expected = QM31(
            CM31(
                M31::from_u32_unchecked(908842852),
                M31::from_u32_unchecked(42171643),
            ),
            CM31(
                M31::from_u32_unchecked(313383432),
                M31::from_u32_unchecked(1019452808),
            ),
        );
        assert_eq!(
            sum, expected,
            "public_logup_sum result should match expected value with new program data"
        );
    }
}
