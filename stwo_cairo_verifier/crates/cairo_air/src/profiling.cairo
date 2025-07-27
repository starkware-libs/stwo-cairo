//! This module is inteded for functions that we want to be able to profile.
//! It is not intended to be used in production code, but rather for debugging and
//! performance analysis during development.

use bounded_int::{ConstValue, upcast};
use core::num::traits::Zero;
use core::num::traits::one::One;
use stwo_constraint_framework::LookupElements;
use stwo_verifier_core::channel::{Channel, ChannelImpl, ChannelTrait};
use stwo_verifier_core::fields::m31::{M31, M31Trait};
use stwo_verifier_core::fields::qm31::qm31_const;
use stwo_verifier_core::utils::ArrayImpl;
use super::test_utils::{dummy_interaction_lookup_elements, mock_public_memory_with_outputs};
use super::{
    CairoInteractionElements, CasmState, MemorySmallValue, PublicData, PublicDataImpl, PublicMemory,
    PublicMemoryTrait, PublicSegmentRanges, RangeChecksInteractionElements, RelationUsesDict,
    SegmentRange, accumulate_relation_uses, hash_memory_section,
};

const REGISTERS_START: ConstValue<1000> = 1000;
const REGISTERS_END: ConstValue<2000> = 2000;

#[test]
fn test_output_mixing() {
    let mut channel: Channel = Default::default();
    let public_memory = mock_public_memory_with_outputs(1000);
    public_memory.mix_into(ref channel);
}
#[test]
fn test_output_logup_sum() {
    let public_memory = mock_public_memory_with_outputs(1000);
    let initial_state = CasmState {
        pc: M31Trait::new(upcast(REGISTERS_START)),
        ap: M31Trait::new(upcast(REGISTERS_START)),
        fp: M31Trait::new(upcast(REGISTERS_START)),
    };
    let final_state = CasmState {
        pc: M31Trait::new(upcast(REGISTERS_END)),
        ap: M31Trait::new(upcast(REGISTERS_END)),
        fp: M31Trait::new(upcast(REGISTERS_END)),
    };
    let public_data = PublicData { public_memory, initial_state, final_state };
    let mut lookup_elements = dummy_interaction_lookup_elements();
    let sum = public_data.logup_sum(@lookup_elements);
}
