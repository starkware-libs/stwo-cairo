//! This module is intended for functions that we want to be able to profile.
//! It is not intended to be used in production code, but rather for debugging and
//! performance analysis during development.

use stwo_verifier_core::channel::{Channel, ChannelImpl};
use stwo_verifier_core::utils::ArrayImpl;
use stwo_verifier_utils::encode_and_hash_memory_section;
use crate::sum_public_memory_entries;
use super::test_utils::{dummy_interaction_lookup_elements, mock_public_memory_with_outputs};
use super::{PublicDataImpl, PublicMemoryTrait};

#[test]
fn test_output_encoding() {
    let public_memory = mock_public_memory_with_outputs(1000);
    encode_and_hash_memory_section(public_memory.output);
}
#[test]
fn test_output_mixing() {
    let mut channel: Channel = Default::default();
    let public_memory = mock_public_memory_with_outputs(1000);
    public_memory.mix_into(ref channel);
}
#[test]
fn test_sum_public_memory_entries() {
    const output_len: u32 = 1000;

    let mut public_memory_entries: Array<(u32, u32, [u32; 8])> = array![];
    for i in 0..output_len {
        public_memory_entries.append((i, i, [i; 8]));
    }

    let mut lookup_elements = dummy_interaction_lookup_elements();

    sum_public_memory_entries(@public_memory_entries, @lookup_elements);
}
