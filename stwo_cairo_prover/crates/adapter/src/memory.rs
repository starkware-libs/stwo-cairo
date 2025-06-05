use std::ops::{Deref, DerefMut};

use bytemuck::{Pod, Zeroable};
use cairo_vm::stdlib::collections::HashMap;
use cairo_vm::types::relocatable::MaybeRelocatable as MaybeRelocatableVM;
use dashmap::DashMap;
use serde::{Deserialize, Serialize};
use stwo_cairo_common::memory::{N_BITS_PER_FELT, N_M31_IN_SMALL_FELT252};
use stwo_cairo_common::prover_types::cpu::Relocatable;
use tracing::{span, Level};

/// Prime 2^251 + 17 * 2^192 + 1 in little endian.
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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MemoryConfig {
    pub small_max: u128,
}
impl MemoryConfig {
    pub fn new(small_max: u128) -> MemoryConfig {
        assert!(small_max < 1 << (N_M31_IN_SMALL_FELT252 * N_BITS_PER_FELT));
        MemoryConfig { small_max }
    }
}
impl Default for MemoryConfig {
    fn default() -> Self {
        MemoryConfig {
            small_max: (1 << 72) - 1,
        }
    }
}

// TODO(spapini): Add U26 for addresses and U128 for range checks.
// TODO(spapini): Use some struct for Felt252 (that is still memory efficient).
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Memory {
    pub config: MemoryConfig,
    pub relocatable_to_id: Vec<Vec<EncodedMemoryValueId>>,
    pub f252_values: Vec<[u32; 8]>,
    pub small_values: Vec<u128>,
    pub relocatable_values: Vec<[u32; 2]>,
}
impl Memory {
    pub fn get(&self, relocatable: Relocatable) -> MemoryValue {
        let segment = self
            .relocatable_to_id
            .get(relocatable.segment_index as usize)
            .unwrap_or_else(|| {
                panic!(
                    "Memory access error: segment_index {} out of bounds",
                    relocatable.segment_index
                )
            });

        let value_id = segment.get(relocatable.offset as usize).unwrap_or_else(|| {
            panic!(
                "Memory access error: offset {} out of bounds for segment {}",
                relocatable.offset, relocatable.segment_index
            )
        });

        match value_id.decode() {
            MemoryValueId::Small(id) => MemoryValue::Small(self.small_values[id as usize]),
            MemoryValueId::F252(id) => MemoryValue::F252(self.f252_values[id as usize]),
            MemoryValueId::MemoryRelocatable(id) => {
                MemoryValue::MemoryRelocatable(self.relocatable_values[id as usize])
            }
            // TODO(Ohad): This case should be a panic, but at the moment there is padding on memory
            // holes, fill the holes before padding, then uncomment.
            MemoryValueId::Empty => panic!("Accessing empty memory cell"),
            // MemoryValueId::Empty => MemoryValue::Small(0),
        }
    }

    pub fn get_raw_id(&self, relocatable: Relocatable) -> u32 {
        self.relocatable_to_id[relocatable.segment_index as usize][relocatable.offset as usize].0
    }
}

// TODO(spapini): Optimize. This should be SIMD.
pub fn value_from_felt252(felt252: F252) -> MemoryValue {
    if felt252[3..8] == [0; 5] && felt252[2] < (1 << 8) {
        MemoryValue::Small(
            felt252[0] as u128
                + ((felt252[1] as u128) << 32)
                + ((felt252[2] as u128) << 64)
                + ((felt252[3] as u128) << 96),
        )
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
    relocatable_id_cache: HashMap<[u32; 2], usize>,
}
impl MemoryBuilder {
    pub fn new(config: MemoryConfig) -> Self {
        Self {
            memory: Memory {
                config,
                relocatable_to_id: Vec::new(),
                f252_values: Vec::new(),
                small_values: Vec::new(),
                relocatable_values: Vec::new(),
            },
            inst_cache: DashMap::new(),
            felt252_id_cache: HashMap::new(),
            small_values_cache: HashMap::new(),
            relocatable_id_cache: HashMap::new(),
        }
    }

    pub fn from_relocatable_memory(
        config: MemoryConfig,
        relocatable_memory: &Vec<Vec<Option<MaybeRelocatableVM>>>,
    ) -> MemoryBuilder {
        let _span = span!(Level::INFO, "MemoryBuilder::from_relocatable_memory").entered();
        let mut builder = Self::new(config);
        for (segment_index, segment) in relocatable_memory.iter().enumerate() {
            builder.relocatable_to_id.push(Vec::new());
            for (offset, value) in segment.iter().enumerate() {
                let relocatable = Relocatable {
                    segment_index: segment_index as usize,
                    offset: offset as u32,
                };
                match value {
                    Some(MaybeRelocatableVM::RelocatableValue(relocatable_value)) => {
                        let relocatable_value_u32 = [
                            relocatable_value.segment_index as u32,
                            relocatable_value.offset as u32,
                        ];
                        builder.set(
                            relocatable,
                            MemoryValue::MemoryRelocatable(relocatable_value_u32),
                        );
                    }
                    Some(MaybeRelocatableVM::Int(felt252)) => {
                        let value = value_from_felt252(bytemuck::cast(felt252.to_bytes_le()));
                        builder.set(relocatable, value);
                    }
                    _ => {}
                }
            }
        }
        for segment in builder.relocatable_to_id.iter_mut() {
            if !segment.is_empty() {
                segment.pop();
            }
        }
        builder
    }

    pub fn get_inst(&self, address: u32) -> u128 {
        *self.inst_cache.entry(address).or_insert_with(|| {
            let value = self.memory.get(Relocatable::program(address)).as_u256();
            assert_eq!(value[3..8], [0; 5]);
            value[0] as u128 | ((value[1] as u128) << 32) | ((value[2] as u128) << 64)
        })
    }

    // TODO(ohadn): settle on an address integer type, and use it consistently.
    // TODO(Ohad): add debug sanity checks.
    pub fn set(&mut self, relocatable: Relocatable, value: MemoryValue) {
        if relocatable.segment_index as usize >= self.relocatable_to_id.len() {
            self.relocatable_to_id
                .resize(relocatable.segment_index as usize + 1, Vec::new());
        }
        if relocatable.offset as usize
            >= self.relocatable_to_id[relocatable.segment_index as usize].len()
        {
            self.relocatable_to_id[relocatable.segment_index as usize].resize(
                relocatable.offset as usize + 1,
                EncodedMemoryValueId::default(),
            );
        }
        let res = EncodedMemoryValueId::encode(match value {
            MemoryValue::Small(val) => {
                let len = self.small_values.len();
                let id = *self.small_values_cache.entry(val).or_insert(len);
                if id == len {
                    self.small_values.push(val);
                };
                MemoryValueId::Small(id as u32)
            }
            MemoryValue::F252(val) => {
                let len = self.f252_values.len();
                let id = *self.felt252_id_cache.entry(val).or_insert(len);
                if id == len {
                    self.f252_values.push(val);
                };
                MemoryValueId::F252(id as u32)
            }
            MemoryValue::MemoryRelocatable(val) => {
                let len = self.relocatable_values.len();
                let id = *self.relocatable_id_cache.entry(val).or_insert(len);
                if id == len {
                    self.relocatable_values.push(val);
                };
                MemoryValueId::MemoryRelocatable(id as u32)
            }
        });
        self.relocatable_to_id[relocatable.segment_index as usize]
            .insert(relocatable.offset as usize, res);
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

pub const LARGE_MEMORY_VALUE_ID_BASE: u32 = 0x2000_0000;
pub const RELOCATABLE_ID_BASE: u32 = 0x4000_0000;

/// Used to mark an unused address.
/// Cannot be assigned as a valid ID, as [`DEFAULT_ID`] > 2**[`LOG_MEMORY_ADDRESS_BOUND`].
pub const DEFAULT_ID: u32 = LARGE_MEMORY_VALUE_ID_BASE - 1;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct EncodedMemoryValueId(pub u32);

impl EncodedMemoryValueId {
    pub fn encode(value: MemoryValueId) -> EncodedMemoryValueId {
        match value {
            MemoryValueId::Small(id) => EncodedMemoryValueId(id),
            MemoryValueId::F252(id) => EncodedMemoryValueId(id | LARGE_MEMORY_VALUE_ID_BASE),
            MemoryValueId::MemoryRelocatable(id) => EncodedMemoryValueId(id | RELOCATABLE_ID_BASE),
            MemoryValueId::Empty => EncodedMemoryValueId(DEFAULT_ID),
        }
    }
    pub fn decode(&self) -> MemoryValueId {
        if self.0 == DEFAULT_ID {
            return MemoryValueId::Empty;
        }
        let tag = self.0 >> 29;
        let val = self.0 & 0x1FFF_FFFF;
        match tag {
            0 => MemoryValueId::Small(val),
            1 => MemoryValueId::F252(val),
            2 => MemoryValueId::MemoryRelocatable(val),
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
    MemoryRelocatable(u32),
    // Used to mark an unused address, a 'hole' in the memory.
    Empty,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum MemoryValue {
    Small(u128),
    F252([u32; 8]),
    MemoryRelocatable([u32; 2]),
}
impl MemoryValue {
    pub fn as_small(&self) -> u128 {
        match self {
            MemoryValue::Small(x) => *x,
            MemoryValue::F252(_) => panic!("Cannot convert F252 to u128"),
            MemoryValue::MemoryRelocatable(_) => panic!("Cannot convert MemoryRelocatable to u128"),
        }
    }

    pub fn as_u256(&self) -> [u32; 8] {
        match *self {
            MemoryValue::Small(x) => {
                let x = u128_to_4_limbs(x);
                [x[0], x[1], x[2], x[3], 0, 0, 0, 0]
            }
            MemoryValue::F252(x) => x,
            MemoryValue::MemoryRelocatable(x) => [x[0], x[1], 0, 0, 0, 0, 0, 0],
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
