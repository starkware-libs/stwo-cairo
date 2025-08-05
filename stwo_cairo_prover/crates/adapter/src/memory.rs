use std::collections::hash_map::Entry;
use std::ops::{Deref, DerefMut};

use bytemuck::{Pod, Zeroable};
use cairo_vm::stdlib::collections::HashMap;
use dashmap::DashMap;
use serde::{Deserialize, Serialize};
use stwo_cairo_common::memory::{N_BITS_PER_FELT, N_M31_IN_SMALL_FELT252};
use tracing::{span, Level};

/// P is 2^251 + 17 * 2^192 - 1.
/// All constants below are in little endian.
pub const P_MIN_1: [u32; 8] = [
    0x0000_0000,
    0x0000_0000,
    0x0000_0000,
    0x0000_0000,
    0x0000_0000,
    0x0000_0000,
    0x0000_0011,
    0x0800_0000,
];

pub const P_MIN_2: [u32; 8] = [
    0xFFFF_FFFF,
    0xFFFF_FFFF,
    0xFFFF_FFFF,
    0xFFFF_FFFF,
    0xFFFF_FFFF,
    0xFFFF_FFFF,
    0x0000_0010,
    0x0800_0000,
];

pub(crate) type F252 = [u32; 8];

#[repr(C)]
#[derive(Copy, Clone, Default, Pod, Zeroable, Debug, PartialEq)]
pub struct MemoryEntry {
    // TODO(Stav): Change to `u32` after this struct is no longer used to read memory files.
    pub address: u64,
    pub value: [u32; 8],
}

/// Configuration for the memory.
///
/// # Attributes
///
/// - `small_max` the maximum value that can be stored in a small value.
/// - `log_small_value_capacity` maximal capacity for small values. Leftover values will be handled
///   as big values.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MemoryConfig {
    pub small_max: u128,
    pub log_small_value_capacity: u32,
}
impl MemoryConfig {
    pub fn new(small_max: u128, log_small_value_capacity: u32) -> MemoryConfig {
        assert!(small_max < 1 << (N_M31_IN_SMALL_FELT252 * N_BITS_PER_FELT));
        MemoryConfig {
            small_max,
            log_small_value_capacity,
        }
    }
}
impl Default for MemoryConfig {
    fn default() -> Self {
        MemoryConfig {
            small_max: (1 << 72) - 1,
            log_small_value_capacity: 24,
        }
    }
}

// TODO(spapini): Add U26 for addresses and U128 for range checks.
// TODO(spapini): Use some struct for Felt252 (that is still memory efficient).
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Memory {
    pub config: MemoryConfig,
    pub address_to_id: Vec<EncodedMemoryValueId>,
    pub f252_values: Vec<[u32; 8]>,
    pub small_values: Vec<u128>,
}
impl Memory {
    pub fn get(&self, addr: u32) -> MemoryValue {
        match self.address_to_id[addr as usize].decode() {
            MemoryValueId::Small(id) => MemoryValue::Small(self.small_values[id as usize]),
            MemoryValueId::F252(id) => MemoryValue::F252(self.f252_values[id as usize]),
            MemoryValueId::Empty => panic!("Accessing empty memory cell"),
        }
    }

    pub fn get_raw_id(&self, addr: u32) -> u32 {
        self.address_to_id[addr as usize].0
    }
}

// TODO(spapini): Optimize. This should be SIMD.
pub fn value_from_felt252(felt252: F252) -> MemoryValue {
    if felt252[3..8] == [0; 5] && felt252[2] < (1 << 8) {
        MemoryValue::Small(limbs_to_u128(felt252[0..4].try_into().unwrap()))
    } else {
        MemoryValue::F252(felt252)
    }
}

// TODO(Ohad): Remove `inst_cache`.
pub struct MemoryBuilder {
    memory: Memory,
    inst_cache: DashMap<u32, u128>,
    felt252_id_cache: HashMap<[u32; 8], usize>,
    small_values_cache: HashMap<u128, usize>,
}
impl MemoryBuilder {
    pub fn new(config: MemoryConfig) -> Self {
        Self {
            memory: Memory {
                config,
                address_to_id: Vec::new(),
                f252_values: Vec::new(),
                small_values: Vec::new(),
            },
            inst_cache: DashMap::new(),
            felt252_id_cache: HashMap::new(),
            small_values_cache: HashMap::new(),
        }
    }

    pub fn from_iter<I: IntoIterator<Item = MemoryEntry>>(
        config: MemoryConfig,
        iter: I,
    ) -> MemoryBuilder {
        let _span = span!(Level::INFO, "MemoryBuilder::from_iter").entered();
        let memory_entries = iter.into_iter();
        let mut builder = Self::new(config);
        for entry in memory_entries {
            let value = value_from_felt252(entry.value);
            builder.set(entry.address as u32, value);
        }

        builder
    }

    pub fn get_inst(&self, addr: u32) -> u128 {
        *self.inst_cache.entry(addr).or_insert_with(|| {
            let value = self.memory.get(addr).as_u256();
            assert_eq!(value[3..8], [0; 5]);
            value[0] as u128 | ((value[1] as u128) << 32) | ((value[2] as u128) << 64)
        })
    }

    pub fn set(&mut self, addr: u32, value: MemoryValue) {
        if addr as usize >= self.address_to_id.len() {
            self.address_to_id
                .resize(addr as usize + 1, EncodedMemoryValueId::default());
        }

        let res = EncodedMemoryValueId::encode(match value {
            MemoryValue::Small(val) => self.push_small_value(val),
            MemoryValue::F252(val) => self.push_f252_value(val),
        });
        self.address_to_id[addr as usize] = res;
    }

    // Assumes value is smaller than `config.small_max`.
    fn push_small_value(&mut self, val: u128) -> MemoryValueId {
        let len = self.small_values.len();
        let capacity = 1 << self.config.log_small_value_capacity;
        match self.small_values_cache.entry(val) {
            // If the value was seen before, return the ID.
            Entry::Occupied(occupied_entry) => MemoryValueId::Small(*occupied_entry.get() as u32),
            Entry::Vacant(vacant_entry) => {
                // Otherwise, check if we can fit it in the small values component.
                if len < capacity {
                    vacant_entry.insert(len);
                    self.small_values.push(val);
                    MemoryValueId::Small(len as u32)
                } else {
                    // If not, treat it as a large value.
                    let f252_value = MemoryValue::Small(val).as_u256();
                    self.push_f252_value(f252_value)
                }
            }
        }
    }

    fn push_f252_value(&mut self, val: [u32; 8]) -> MemoryValueId {
        let len = self.f252_values.len();
        let id = *self.felt252_id_cache.entry(val).or_insert(len);
        if id == len {
            self.f252_values.push(val);
        };
        MemoryValueId::F252(id as u32)
    }

    /// Copies a block of memory from one location to another.
    /// The values at addresses src_start_addr to src_start_addr + segment_length - 1 are copied to
    /// the addresses dst_start_addr to dst_start_addr + segment_length - 1.
    pub fn copy_block(&mut self, src_start_addr: u32, dst_start_addr: u32, segment_length: u32) {
        for i in 0..segment_length {
            self.set(dst_start_addr + i, self.memory.get(src_start_addr + i));
        }
    }

    pub fn assert_segment_is_empty(&self, start_addr: u32, segment_length: u32) {
        let len = self.address_to_id.len();
        let start = start_addr as usize;
        let end = std::cmp::min(len, (start_addr + segment_length) as usize);

        if let Some(non_empty) = self.address_to_id[start..end]
            .iter()
            .position(|&id| id != EncodedMemoryValueId::default())
        {
            panic!(
                "Memory expected empty at addresses {}, found ID: {:?}",
                start + non_empty,
                self.address_to_id[start + non_empty]
            );
        }
    }

    pub fn build(self) -> (Memory, Vec<(u32, u128)>) {
        (self.memory, self.inst_cache.into_iter().collect())
    }
}
impl Deref for MemoryBuilder {
    type Target = Memory;
    fn deref(&self) -> &Self::Target {
        &self.memory
    }
}
impl DerefMut for MemoryBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.memory
    }
}

pub const LARGE_MEMORY_VALUE_ID_BASE: u32 = 0x4000_0000;

/// Used to mark an unused address.
/// Cannot be assigned as a valid ID, as [`DEFAULT_ID`] > 2**[`LOG_MEMORY_ADDRESS_BOUND`].
pub const DEFAULT_ID: u32 = LARGE_MEMORY_VALUE_ID_BASE - 1;

#[derive(Copy, Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct EncodedMemoryValueId(pub u32);
impl EncodedMemoryValueId {
    pub fn encode(value: MemoryValueId) -> EncodedMemoryValueId {
        match value {
            MemoryValueId::Small(id) => EncodedMemoryValueId(id),
            MemoryValueId::F252(id) => EncodedMemoryValueId(id | LARGE_MEMORY_VALUE_ID_BASE),
            MemoryValueId::Empty => EncodedMemoryValueId(DEFAULT_ID),
        }
    }
    pub fn decode(&self) -> MemoryValueId {
        if self.0 == DEFAULT_ID {
            return MemoryValueId::Empty;
        }
        let tag = self.0 >> 30;
        let val = self.0 & 0x3FFF_FFFF;
        match tag {
            0 => MemoryValueId::Small(val),
            1 => MemoryValueId::F252(val),
            _ => panic!("Invalid tag"),
        }
    }
}

impl Default for EncodedMemoryValueId {
    fn default() -> Self {
        Self(DEFAULT_ID)
    }
}

pub enum MemoryValueId {
    Small(u32),
    F252(u32),
    // Used to mark an unused address, a 'hole' in the memory.
    Empty,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum MemoryValue {
    Small(u128),
    F252([u32; 8]),
}
impl MemoryValue {
    pub fn as_small(&self) -> u128 {
        match self {
            MemoryValue::Small(x) => *x,
            MemoryValue::F252(felt252) => {
                assert_eq!(felt252[4..8], [0; 4], "Cannot convert F252 to u128");
                limbs_to_u128(felt252[0..4].try_into().unwrap())
            }
        }
    }

    pub fn as_u256(&self) -> [u32; 8] {
        match *self {
            MemoryValue::Small(x) => {
                let x = u128_to_4_limbs(x);
                [x[0], x[1], x[2], x[3], 0, 0, 0, 0]
            }
            MemoryValue::F252(x) => x,
        }
    }

    pub fn is_zero(&self) -> bool {
        match *self {
            MemoryValue::Small(x) => x == 0,
            MemoryValue::F252(x) => x == [0; 8],
        }
    }
}

pub fn u128_to_4_limbs(x: u128) -> [u32; 4] {
    [
        x as u32,
        (x >> 32) as u32,
        (x >> 64) as u32,
        (x >> 96) as u32,
    ]
}

pub fn limbs_to_u128(limbs: [u32; 4]) -> u128 {
    limbs[0] as u128
        + ((limbs[1] as u128) << 32)
        + ((limbs[2] as u128) << 64)
        + ((limbs[3] as u128) << 96)
}

#[cfg(test)]
mod tests {

    use cairo_vm::relocatable;
    use cairo_vm::types::relocatable::{MaybeRelocatable, Relocatable};

    use super::*;
    use crate::relocator::Relocator;

    #[test]
    fn test_memory() {
        let entries = [
            MemoryEntry {
                address: 0,
                value: [1; 8],
            },
            MemoryEntry {
                address: 1,
                value: [6, 0, 0, 0, 0, 0, 0, 0],
            },
            MemoryEntry {
                address: 2,
                value: [1, 2, 0, 0, 0, 0, 0, 0],
            },
            MemoryEntry {
                address: 5,
                value: [1 << 24, 0, 0, 0, 0, 0, 0, 0],
            },
            MemoryEntry {
                address: 8,
                value: P_MIN_1,
            },
            MemoryEntry {
                address: 9,
                value: P_MIN_2,
            },
            // Duplicates.
            MemoryEntry {
                address: 100,
                value: [1; 8],
            },
            MemoryEntry {
                address: 105,
                value: [1 << 24, 0, 0, 0, 0, 0, 0, 0],
            },
            MemoryEntry {
                address: 200,
                value: [1, 1, 1, 0, 0, 0, 0, 0],
            },
            MemoryEntry {
                address: 201,
                value: [1, 1, 1 << 10, 0, 0, 0, 0, 0],
            },
        ];
        let memory = MemoryBuilder::from_iter(MemoryConfig::default(), entries.iter().cloned());
        assert_eq!(memory.get(0), MemoryValue::F252([1; 8]));
        assert_eq!(memory.get(1), MemoryValue::Small(6));
        assert_eq!(
            memory.get(200),
            MemoryValue::Small(1 + (1 << 32) + (1 << 64))
        );
        assert_eq!(
            memory.get(201),
            MemoryValue::F252([1, 1, 1 << 10, 0, 0, 0, 0, 0])
        );
        assert_eq!(memory.get(8), MemoryValue::F252(P_MIN_1));
        assert_eq!(memory.get(9), MemoryValue::F252(P_MIN_2));
        // Duplicates.
        assert_eq!(memory.get(100), MemoryValue::F252([1; 8]));
        assert_eq!(memory.address_to_id[0], memory.address_to_id[100]);
        assert_eq!(memory.address_to_id[5], memory.address_to_id[105]);
    }

    #[test]
    fn test_memory_value_casts() {
        let small = MemoryValue::Small(1);
        assert_eq!(small.as_small(), 1);
        assert_eq!(small.as_u256(), [1, 0, 0, 0, 0, 0, 0, 0]);

        let f252 = MemoryValue::F252([1; 8]);
        assert_eq!(f252.as_u256(), [1; 8]);
    }

    #[test]
    fn test_memory_holes_have_default_id() {
        let entries = [
            MemoryEntry {
                address: 0,
                value: [1; 8],
            },
            MemoryEntry {
                address: 2,
                value: [1, 2, 0, 0, 0, 0, 0, 0],
            },
        ];
        let expxcted_id_addr_0 = EncodedMemoryValueId::encode(MemoryValueId::F252(0));
        let expxcted_id_addr_1 = EncodedMemoryValueId::default();
        let expxcted_id_addr_2 = EncodedMemoryValueId::encode(MemoryValueId::Small(0));

        let (memory, ..) = MemoryBuilder::from_iter(MemoryConfig::default(), entries).build();
        let addr_0_id = memory.address_to_id[0];
        let addr_1_id = memory.address_to_id[1];
        let addr_2_id = memory.address_to_id[2];

        assert_eq!(addr_0_id, expxcted_id_addr_0);
        assert_eq!(addr_1_id, expxcted_id_addr_1);
        assert_eq!(addr_2_id, expxcted_id_addr_2);
    }

    #[test]
    fn test_memory_from_relocator() {
        let segment0 = vec![
            Some(MaybeRelocatable::Int(1.into())),
            Some(MaybeRelocatable::Int(9.into())),
            Some(MaybeRelocatable::RelocatableValue(relocatable!(2, 1))),
        ];
        let builtin_segment1 =
            vec![Some(MaybeRelocatable::RelocatableValue(relocatable!(0, 1))); 80];
        let segment2 = vec![
            Some(MaybeRelocatable::Int(1.into())),
            Some(MaybeRelocatable::Int(2.into())),
            Some(MaybeRelocatable::Int(3.into())),
        ];
        let memory = vec![segment0, builtin_segment1, segment2];
        let relocator = Relocator::new(&memory);

        let memory: MemoryBuilder =
            MemoryBuilder::from_iter(MemoryConfig::default(), relocator.relocate_memory(&memory));
        assert_eq!(memory.get(1), MemoryValue::Small(1));
        assert_eq!(memory.get(85), MemoryValue::Small(2));
    }

    #[should_panic = "Accessing empty memory cell"]
    #[test]
    fn test_access_invalid_address() {
        let entries = [
            MemoryEntry {
                address: 0,
                value: [1; 8],
            },
            MemoryEntry {
                address: 2,
                value: [1, 2, 0, 0, 0, 0, 0, 0],
            },
        ];
        let (memory, ..) = MemoryBuilder::from_iter(MemoryConfig::default(), entries).build();

        memory.get(1);
    }

    #[should_panic = "Memory expected empty at addresses 2, found ID: EncodedMemoryValueId(0)"]
    #[test]
    fn test_assert_segment_is_empty() {
        let memory_config = MemoryConfig::default();
        let mut memory_builder = MemoryBuilder::new(memory_config);

        memory_builder.set(2, MemoryValue::Small(123));

        memory_builder.assert_segment_is_empty(0, 4);
    }
}
