use stwo::core::channel::{Channel, MerkleChannel};
use stwo::core::vcs_lifted::MerkleHasherLifted;
use stwo_cairo_common::prover_types::cpu::{CasmState, FELT252_N_WORDS, M31};
use stwo_cairo_common::prover_types::felt::split_f252;

use crate::air::{PublicData, PublicSegmentRanges, SegmentRange};
use crate::utils::pack_into_secure_felts;

const N_SEGMENTS: usize = 11;
const MEMORY_VALUES_LIMBS: usize = FELT252_N_WORDS;

pub struct AuxData {
    pub initial_state: CasmState,
    pub final_state: CasmState,
    pub segment_ranges: [SegmentRange; N_SEGMENTS],
    pub safe_call_ids: [M31; 2],
    pub output_ids: Vec<M31>,
    pub program_ids: Vec<M31>,
}

impl AuxData {
    pub fn from_public_data(public_data: &PublicData) -> Self {
        let PublicData {
            public_memory,
            initial_state,
            final_state,
        } = public_data;
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
        } = public_memory.public_segments;
        Self {
            initial_state: *initial_state,
            final_state: *final_state,
            segment_ranges: [
                output,
                pedersen.unwrap_or_default(),
                range_check_128.unwrap_or_default(),
                ecdsa.unwrap_or_default(),
                bitwise.unwrap_or_default(),
                ec_op.unwrap_or_default(),
                keccak.unwrap_or_default(),
                poseidon.unwrap_or_default(),
                range_check_96.unwrap_or_default(),
                add_mod.unwrap_or_default(),
                mul_mod.unwrap_or_default(),
            ],
            safe_call_ids: public_memory.safe_call_ids.map(M31::from_u32_unchecked),
            output_ids: public_memory
                .output
                .iter()
                .map(|(id, _)| M31::from_u32_unchecked(*id))
                .collect(),
            program_ids: public_memory
                .program
                .iter()
                .map(|(id, _)| M31::from_u32_unchecked(*id))
                .collect(),
        }
    }

    /// Converts auxiliary public data to [u32], where each u32 is at most 2^31 - 1.
    pub fn pack_into_u32s(&self) -> Vec<u32> {
        let mut public_claim = vec![
            self.initial_state.pc.0,
            self.initial_state.ap.0,
            self.initial_state.fp.0,
            self.final_state.pc.0,
            self.final_state.ap.0,
            self.final_state.fp.0,
        ];
        for segment in self.segment_ranges {
            Self::append_segment_range(segment, &mut public_claim);
        }
        public_claim.extend(self.safe_call_ids.map(|id| id.0));
        public_claim.extend(self.output_ids.iter().map(|id| id.0));
        public_claim.extend(self.program_ids.iter().map(|id| id.0));
        public_claim
    }

    fn append_segment_range(segment: SegmentRange, public_claim: &mut Vec<u32>) {
        public_claim.extend([
            segment.start_ptr.id,
            segment.start_ptr.value,
            segment.stop_ptr.id,
            segment.stop_ptr.value,
        ]);
    }

    pub fn mix_into<MC: MerkleChannel>(&self, channel: &mut MC::C) {
        channel.mix_felts(&pack_into_secure_felts(self.pack_into_u32s().into_iter()));
    }
}

pub struct FlatClaim {
    pub component_enable_bits: Vec<bool>,
    // TODO(ilya): move `component_log_sizes` to `aux_data`
    pub component_log_sizes: Vec<u32>,
    pub aux_data: AuxData,
    outputs: Vec<[M31; MEMORY_VALUES_LIMBS]>,
    program: Vec<[M31; MEMORY_VALUES_LIMBS]>,
}

impl FlatClaim {
    pub fn new(
        component_enable_bits: Vec<bool>,
        component_log_sizes: Vec<u32>,
        public_data: &PublicData,
    ) -> Self {
        let aux_data = AuxData::from_public_data(public_data);
        let outputs = public_data
            .public_memory
            .output
            .iter()
            .map(|(_, value)| split_f252(*value))
            .collect();
        let program = public_data
            .public_memory
            .program
            .iter()
            .map(|(_, value)| split_f252(*value))
            .collect();
        Self {
            component_enable_bits,
            component_log_sizes,
            aux_data,
            outputs,
            program,
        }
    }

    pub fn mix_into<MC: MerkleChannel>(&self, channel: &mut MC::C) {
        channel.mix_felts(&pack_into_secure_felts(
            [self.component_enable_bits.len() as u32].into_iter(),
        ));
        channel.mix_felts(&pack_into_secure_felts(
            enable_bits_to_u32s(&self.component_enable_bits).into_iter(),
        ));
        channel.mix_felts(&pack_into_secure_felts(
            self.component_log_sizes.iter().cloned(),
        ));
        channel.mix_felts(&pack_into_secure_felts(
            [self.program.len() as u32].into_iter(),
        ));
        self.aux_data.mix_into::<MC>(channel);
        mix_memory_values_into::<MC>(channel, &self.outputs);
        mix_memory_values_into::<MC>(channel, &self.program);
    }
}

fn mix_memory_values_into<MC: MerkleChannel>(
    channel: &mut MC::C,
    values: &[[M31; MEMORY_VALUES_LIMBS]],
) {
    let mut hasher = MC::H::default();
    hasher.update_leaf(
        values
            .iter()
            .flat_map(|value| value.iter().copied())
            .collect::<Vec<_>>()
            .as_slice(),
    );
    MC::mix_root(channel, hasher.finalize());
}

/// Converts enable bits to [u32], where each u32 is at most 2^31 - 1.
fn enable_bits_to_u32s(enable_bits: &[bool]) -> Vec<u32> {
    enable_bits.iter().map(|&b| if b { 1 } else { 0 }).collect()
}
