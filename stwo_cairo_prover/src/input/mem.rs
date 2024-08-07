use super::vm_import::MemEntry;

pub const SMALL_VALUE_SHIFT: i32 = 1 << 16;

// TODO(spapini): Add U26 for addresses and U128 for range checks.
// TODO(spapini): Use some struct for Felt252 (that is still memory efficient).
#[derive(Default)]
pub struct Memory {
    pub address_to_id: Vec<EncodedMemoryValueId>,
    pub u64_values: Vec<u64>,
    pub f252_values: Vec<[u32; 8]>,
}
impl Memory {
    pub fn get(&self, addr: u64) -> MemoryValue {
        match self.address_to_id[addr as usize].decode() {
            MemoryValueId::I16(id) => MemoryValue::I16(id),
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
            MemoryValue::I16(id) => MemoryValueId::I16(id),
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
}

impl FromIterator<MemEntry> for Memory {
    fn from_iter<I: IntoIterator<Item = MemEntry>>(iter: I) -> Memory {
        let mem_entries = iter.into_iter();
        let mut memory = Memory::default();
        for entry in mem_entries {
            memory.set(entry.addr, MemoryValue::from(entry.val));
        }
        memory
    }
}

#[derive(Copy, Clone, Default)]
pub struct EncodedMemoryValueId(u32);
impl EncodedMemoryValueId {
    pub fn encode(value: MemoryValueId) -> EncodedMemoryValueId {
        match value {
            MemoryValueId::I16(id) => EncodedMemoryValueId((id + SMALL_VALUE_SHIFT) as u32),
            MemoryValueId::U64(id) => EncodedMemoryValueId(id as u32 | 0x4000_0000),
            MemoryValueId::F252(id) => EncodedMemoryValueId(id as u32 | 0x8000_0000),
        }
    }
    pub fn decode(&self) -> MemoryValueId {
        let tag = self.0 >> 30;
        let val = self.0 & 0x3FFF_FFFF;
        match tag {
            0 => MemoryValueId::I16(val as i32 - SMALL_VALUE_SHIFT),
            1 => MemoryValueId::U64(val as usize),
            2 => MemoryValueId::F252(val as usize),
            _ => panic!("Invalid tag"),
        }
    }
}

pub enum MemoryValueId {
    I16(i32),
    U64(usize),
    F252(usize),
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum MemoryValue {
    I16(i32),
    U64(u64),
    F252([u32; 8]),
}
impl MemoryValue {
    pub fn as_64(&self) -> u64 {
        match self {
            MemoryValue::I16(x) => (*x).try_into().unwrap(),
            MemoryValue::U64(x) => *x,
            MemoryValue::F252(_) => panic!("Cannot convert F252 to u64"),
        }
    }
}
impl From<[u32; 8]> for MemoryValue {
    fn from(value: [u32; 8]) -> MemoryValue {
        if value[2..].iter().all(|&x| x == 0) {
            if value[1] == 0 && value[0] < SMALL_VALUE_SHIFT as u32 {
                // TODO: Handle small negatives.
                MemoryValue::I16(value[0] as i32)
            } else {
                MemoryValue::U64((value[0] as u64) | ((value[1] as u64) << 32))
            }
        } else {
            MemoryValue::F252(value)
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
        ];
        let memory = Memory::from_iter(entries.iter().cloned());
        assert_eq!(memory.get(0), MemoryValue::F252([1; 8]));
        assert_eq!(memory.get(1), MemoryValue::I16(6));
        assert_eq!(memory.get(2), MemoryValue::U64((2 << 32) | 1));
    }
}
