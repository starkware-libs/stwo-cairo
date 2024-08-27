use std::collections::HashMap;
use std::ops::{Deref, DerefMut};

use super::vm_import::MemEntry;

/// Prime 2^251 + 17 * 2^192 + 1 in little endian.
const P_MIN_1: [u32; 8] = [
    0x0000_0000,
    0x0000_0000,
    0x0000_0000,
    0x0000_0000,
    0x0000_0000,
    0x0000_0000,
    0x0000_0011,
    0x0800_0000,
];
const P_MIN_2: [u32; 8] = [
    0xFFFF_FFFF,
    0xFFFF_FFFF,
    0xFFFF_FFFF,
    0xFFFF_FFFF,
    0xFFFF_FFFF,
    0xFFFF_FFFF,
    0x0000_0010,
    0x0800_0000,
];

// Note: this should be smaller than 2^29.
const SMALL_VALUE_SHIFT: u32 = 1 << 26;

#[derive(Debug)]
pub struct MemConfig {
    /// The absolute value of the smallest negative value that can be stored as a small value.
    pub small_min_neg: u32,
    /// The largest value that can be stored as a small value.
    pub small_max: u32,
}
impl MemConfig {
    pub fn new(small_min_neg: u32, small_max: u32) -> MemConfig {
        assert!(small_min_neg <= SMALL_VALUE_SHIFT);
        assert!(small_max <= SMALL_VALUE_SHIFT);
        MemConfig {
            small_min_neg,
            small_max,
        }
    }
}
impl Default for MemConfig {
    fn default() -> Self {
        MemConfig {
            small_min_neg: (1 << 10) - 1,
            small_max: (1 << 10) - 1,
        }
    }
}

// TODO(spapini): Add U26 for addresses and U128 for range checks.
// TODO(spapini): Use some struct for Felt252 (that is still memory efficient).
#[derive(Debug)]
pub struct Memory {
    pub config: MemConfig,
    pub address_to_id: Vec<EncodedMemoryValueId>,
    pub inst_cache: HashMap<u32, u64>,
    pub f252_values: Vec<[u32; 8]>,
}
impl Memory {
    pub fn get(&self, addr: u32) -> MemoryValue {
        match self.address_to_id[addr as usize].decode() {
            MemoryValueId::Small(id) => MemoryValue::Small(id),
            MemoryValueId::F252(id) => MemoryValue::F252(self.f252_values[id]),
        }
    }
    pub fn get_inst(&self, addr: u32) -> Option<u64> {
        self.inst_cache.get(&addr).copied()
    }
    // TODO(spapini): Optimize. This should be SIMD.
    pub fn value_from_felt252(&self, value: [u32; 8]) -> MemoryValue {
        if value[7] == 0 {
            // Positive case.
            if value[1..7] != [0; 6] || value[0] > self.config.small_max {
                // Not small.
                return MemoryValue::F252(value);
            }
            MemoryValue::Small(value[0] as i32)
        } else {
            // Negative case.
            if value == P_MIN_1 {
                return MemoryValue::Small(-1);
            }
            if value[1..7] != P_MIN_2[1..7] {
                return MemoryValue::F252(value);
            }
            let num = 0xFFFF_FFFF - value[0];
            if num < self.config.small_min_neg - 2 {
                return MemoryValue::Small(-(num as i32 + 2));
            }
            MemoryValue::F252(value)
        }
    }
}

pub struct MemoryBuilder {
    pub mem: Memory,
    felt252_id_cache: HashMap<[u32; 8], usize>,
}
impl MemoryBuilder {
    pub fn new(config: MemConfig) -> Self {
        Self {
            mem: Memory {
                config,
                address_to_id: Vec::new(),
                inst_cache: HashMap::new(),
                f252_values: Vec::new(),
            },
            felt252_id_cache: HashMap::new(),
        }
    }
    pub fn from_iter<I: IntoIterator<Item = MemEntry>>(
        config: MemConfig,
        iter: I,
    ) -> MemoryBuilder {
        let mem_entries = iter.into_iter();
        let mut builder = Self::new(config);
        for entry in mem_entries {
            let value = builder.value_from_felt252(entry.val);
            builder.set(entry.addr, value);
        }
        builder
    }
    pub fn set(&mut self, addr: u64, value: MemoryValue) {
        if addr as usize >= self.address_to_id.len() {
            self.address_to_id
                .resize(addr as usize + 1, EncodedMemoryValueId(0));
        }
        let res = EncodedMemoryValueId::encode(match value {
            MemoryValue::Small(val) => MemoryValueId::Small(val),
            MemoryValue::F252(val) => {
                let len = self.f252_values.len();
                let id = *self.felt252_id_cache.entry(val).or_insert(len);
                if id == len {
                    self.f252_values.push(val);
                };
                MemoryValueId::F252(id)
            }
        });
        self.address_to_id[addr as usize] = res;
    }
    pub fn get_inst(&mut self, addr: u32) -> u64 {
        let mut inst_cache = std::mem::take(&mut self.inst_cache);
        let res = *inst_cache.entry(addr).or_insert_with(|| {
            let value = self.mem.get(addr).as_u256();
            assert_eq!(value[2..8], [0; 6]);
            value[0] as u64 | ((value[1] as u64) << 32)
        });
        self.inst_cache = inst_cache;
        res
    }
    pub fn build(self) -> Memory {
        self.mem
    }
}
impl Deref for MemoryBuilder {
    type Target = Memory;
    fn deref(&self) -> &Self::Target {
        &self.mem
    }
}
impl DerefMut for MemoryBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.mem
    }
}

#[derive(Copy, Clone, Default, PartialEq, Eq, Debug)]
pub struct EncodedMemoryValueId(u32);
impl EncodedMemoryValueId {
    pub fn encode(value: MemoryValueId) -> EncodedMemoryValueId {
        match value {
            MemoryValueId::Small(id) => {
                EncodedMemoryValueId((id + SMALL_VALUE_SHIFT as i32) as u32)
            }
            MemoryValueId::F252(id) => EncodedMemoryValueId(id as u32 | 0x4000_0000),
        }
    }
    pub fn decode(&self) -> MemoryValueId {
        let tag = self.0 >> 30;
        let val = self.0 & 0x3FFF_FFFF;
        match tag {
            0 => MemoryValueId::Small(val as i32 - SMALL_VALUE_SHIFT as i32),
            1 => MemoryValueId::F252(val as usize),
            _ => panic!("Invalid tag"),
        }
    }
}

pub enum MemoryValueId {
    Small(i32),
    F252(usize),
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum MemoryValue {
    Small(i32),
    F252([u32; 8]),
}
impl MemoryValue {
    pub fn as_small(&self) -> i32 {
        match self {
            MemoryValue::Small(x) => *x,
            MemoryValue::F252(_) => panic!("Cannot convert F252 to i32"),
        }
    }
    pub fn as_u256(&self) -> [u32; 8] {
        match *self {
            MemoryValue::Small(x) => {
                if x >= 0 {
                    [x as u32, 0, 0, 0, 0, 0, 0, 0]
                } else if x == -1 {
                    P_MIN_1
                } else {
                    let mut res = P_MIN_2;
                    res[0] = 0xFFFF_FFFF - (-x - 2) as u32;
                    res
                }
            }
            MemoryValue::F252(x) => x,
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_memory() {
        let entries = [
            MemEntry {
                addr: 0,
                val: [1; 8],
            },
            MemEntry {
                addr: 1,
                val: [6, 0, 0, 0, 0, 0, 0, 0],
            },
            MemEntry {
                addr: 2,
                val: [1, 2, 0, 0, 0, 0, 0, 0],
            },
            MemEntry {
                addr: 5,
                val: [1 << 24, 0, 0, 0, 0, 0, 0, 0],
            },
            MemEntry {
                addr: 8,
                val: P_MIN_1,
            },
            MemEntry {
                addr: 9,
                val: P_MIN_2,
            },
            // Duplicates.
            MemEntry {
                addr: 100,
                val: [1; 8],
            },
            MemEntry {
                addr: 105,
                val: [1 << 24, 0, 0, 0, 0, 0, 0, 0],
            },
        ];
        let memory = MemoryBuilder::from_iter(MemConfig::default(), entries.iter().cloned());
        assert_eq!(memory.get(0), MemoryValue::F252([1; 8]));
        assert_eq!(memory.get(1), MemoryValue::Small(6));
        assert_eq!(memory.get(8), MemoryValue::Small(-1));
        assert_eq!(memory.get(9), MemoryValue::Small(-2));
        // Duplicates.
        assert_eq!(memory.get(100), MemoryValue::F252([1; 8]));
        assert_eq!(memory.address_to_id[0], memory.address_to_id[100]);
        assert_eq!(memory.address_to_id[5], memory.address_to_id[105]);
    }

    #[test]
    fn test_mem_value_casts() {
        let small = MemoryValue::Small(1);
        assert_eq!(small.as_small(), 1);
        assert_eq!(small.as_u256(), [1, 0, 0, 0, 0, 0, 0, 0]);

        let small_negative = MemoryValue::Small(-5);
        assert_eq!(small_negative.as_small(), -5);
        assert_eq!(
            small_negative.as_u256().as_slice(),
            [&[0xFFFFFFFC], &P_MIN_2[1..]].concat().as_slice()
        );

        let f252 = MemoryValue::F252([1; 8]);
        assert_eq!(f252.as_u256(), [1; 8]);
    }
}
