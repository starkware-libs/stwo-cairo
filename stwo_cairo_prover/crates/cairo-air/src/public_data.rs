use itertools::Itertools;
use serde::{Deserialize, Serialize};
use stwo::core::channel::Channel;
use stwo::core::fields::m31::M31;
use stwo::core::fields::qm31::QM31;
use stwo_cairo_common::prover_types::cpu::CasmState;
use stwo_cairo_common::prover_types::felt::split_f252;
use stwo_cairo_serialize::{CairoDeserialize, CairoSerialize};
use stwo_constraint_framework::Relation;

use crate::air::CairoInteractionElements;
use crate::relations;

#[derive(Serialize, Deserialize, CairoSerialize, CairoDeserialize)]
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

        let inverted_values =
            <QM31 as stwo::core::fields::FieldExpOps>::batch_inverse(&values_to_inverse);
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
#[derive(Clone, Debug, Serialize, Deserialize, Copy, CairoSerialize, CairoDeserialize)]
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

#[derive(Clone, Debug, Serialize, Deserialize, Copy, CairoSerialize, CairoDeserialize)]
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

#[derive(Clone, Debug, Serialize, Deserialize, Copy)]
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

#[derive(Serialize, Deserialize, CairoSerialize, CairoDeserialize)]
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
