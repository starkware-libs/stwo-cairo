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
pub struct MemConfig {
    /// The absolute value of the smallest negative value that can be stored as a small value.
    small_min_neg: u32,
    /// The largest value that can be stored as a small value.
    small_max: u32,
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
            small_min_neg: (1 << 20) - 1,
            small_max: (1 << 20) - 1,
        }
    }
}

// TODO(spapini): Add U26 for addresses and U128 for range checks.
// TODO(spapini): Use some struct for Felt252 (that is still memory efficient).
pub struct Memory {
    pub config: MemConfig,
    pub address_to_id: Vec<EncodedMemoryValueId>,
    pub u64_values: Vec<u64>,
    pub f252_values: Vec<[u32; 8]>,
}
impl Memory {
    fn new(config: MemConfig) -> Memory {
        Memory {
            config,
            address_to_id: Vec::new(),
            u64_values: Vec::new(),
            f252_values: Vec::new(),
        }
    }
    pub fn get(&self, addr: u32) -> MemoryValue {
        match self.address_to_id[addr as usize].decode() {
            MemoryValueId::Small(id) => MemoryValue::Small(id),
            MemoryValueId::U64(id) => MemoryValue::U64(self.u64_values[id]),
            MemoryValueId::F252(id) => MemoryValue::F252(self.f252_values[id]),
        }
    }
    pub fn set(&mut self, addr: u64, value: MemoryValue) {
        if addr as usize >= self.address_to_id.len() {
            self.address_to_id
                .resize(addr as usize + 1, EncodedMemoryValueId(0));
        }
        self.address_to_id[addr as usize] = EncodedMemoryValueId::encode(match value {
            MemoryValue::Small(id) => MemoryValueId::Small(id),
            MemoryValue::U64(id) => {
                let res = self.u64_values.len();
                self.u64_values.push(id);
                MemoryValueId::U64(res)
            }
            MemoryValue::F252(id) => {
                let res = self.f252_values.len();
                self.f252_values.push(id);
                MemoryValueId::F252(res)
            }
        })
    }
    pub fn from_iter<I: IntoIterator<Item = MemEntry>>(config: MemConfig, iter: I) -> Memory {
        let mem_entries = iter.into_iter();
        let mut memory = Memory::new(config);
        for entry in mem_entries {
            let value = memory.value_from_felt252(entry.val);
            memory.set(entry.addr, value);
        }
        memory
    }
    // TODO(spapini): Optimize. This should be SIMD.
    fn value_from_felt252(&self, value: [u32; 8]) -> MemoryValue {
        if value[7] == 0 {
            // Positive case.
            if value[2..7] != [0; 5] {
                // Over 64bit.
                return MemoryValue::F252(value);
            }
            if value[1] != 0 || value[0] > self.config.small_max {
                // Not small.
                return MemoryValue::U64((value[0] as u64) | ((value[1] as u64) << 32));
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

#[derive(Copy, Clone, Default)]
pub struct EncodedMemoryValueId(u32);
impl EncodedMemoryValueId {
    pub fn encode(value: MemoryValueId) -> EncodedMemoryValueId {
        match value {
            MemoryValueId::Small(id) => {
                EncodedMemoryValueId((id + SMALL_VALUE_SHIFT as i32) as u32)
            }
            MemoryValueId::U64(id) => EncodedMemoryValueId(id as u32 | 0x4000_0000),
            MemoryValueId::F252(id) => EncodedMemoryValueId(id as u32 | 0x8000_0000),
        }
    }
    pub fn decode(&self) -> MemoryValueId {
        let tag = self.0 >> 30;
        let val = self.0 & 0x3FFF_FFFF;
        match tag {
            0 => MemoryValueId::Small(val as i32 - SMALL_VALUE_SHIFT as i32),
            1 => MemoryValueId::U64(val as usize),
            2 => MemoryValueId::F252(val as usize),
            _ => panic!("Invalid tag"),
        }
    }
}

pub enum MemoryValueId {
    Small(i32),
    U64(usize),
    F252(usize),
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum MemoryValue {
    Small(i32),
    U64(u64),
    F252([u32; 8]),
}
impl MemoryValue {
    pub fn as_small(&self) -> i32 {
        match self {
            MemoryValue::Small(x) => *x,
            MemoryValue::U64(x) => (*x).try_into().unwrap(),
            MemoryValue::F252(_) => panic!("Cannot convert F252 to i32"),
        }
    }
    pub fn as_u64(&self) -> u64 {
        match self {
            MemoryValue::Small(x) => (*x).try_into().unwrap(),
            MemoryValue::U64(x) => *x,
            MemoryValue::F252(_) => panic!("Cannot convert F252 to u64"),
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
        ];
        let memory = Memory::from_iter(MemConfig::default(), entries.iter().cloned());
        assert_eq!(memory.get(0), MemoryValue::F252([1; 8]));
        assert_eq!(memory.get(1), MemoryValue::Small(6));
        assert_eq!(memory.get(2), MemoryValue::U64((2 << 32) | 1));
        assert_eq!(memory.get(5), MemoryValue::U64(1 << 24));
        assert_eq!(memory.get(8), MemoryValue::Small(-1));
        assert_eq!(memory.get(9), MemoryValue::Small(-2));
    }
}
