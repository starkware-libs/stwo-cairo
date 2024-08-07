use crate::execution_trace::MemEntry;

pub const SMALL_VALUE_SHIFT: i32 = 1 << 30;

#[derive(Default)]
pub struct Memory {
    address_to_id: Vec<PackedMemoryValueId>,
    large_values: Vec<[u32; 8]>,
}
impl Memory {
    pub fn from_iter(mem_entries: impl Iterator<Item = MemEntry>) -> Memory {
        let mut memory = Memory::default();
        for entry in mem_entries {
            memory.set(entry.addr, MemoryValue::from(entry.val));
        }
        memory
    }

    pub fn get(&self, addr: u64) -> MemoryValue {
        match self.address_to_id[addr as usize].unpack() {
            UnpackedMemoryValueId::Small(id) => MemoryValue::Small(id),
            UnpackedMemoryValueId::Large(id) => MemoryValue::Large(self.large_values[id]),
        }
    }
    // TODO(spapini): Get as bigint.
    pub fn set(&mut self, addr: u64, value: MemoryValue) {
        if addr as usize >= self.address_to_id.len() {
            self.address_to_id
                .resize(addr as usize + 1, PackedMemoryValueId(0));
        }
        match value {
            MemoryValue::Small(id) => {
                self.address_to_id[addr as usize] =
                    PackedMemoryValueId((id + SMALL_VALUE_SHIFT) as u32)
            }
            MemoryValue::Large(value) => {
                let id = self.large_values.len();
                self.address_to_id[addr as usize] = PackedMemoryValueId(id as u32 | 0x8000_0000);
                self.large_values.push(value);
            }
        }
    }
}

#[derive(Copy, Clone, Default)]
pub struct PackedMemoryValueId(u32);
impl PackedMemoryValueId {
    pub fn unpack(&self) -> UnpackedMemoryValueId {
        if self.0 & 0x8000_0000 != 0 {
            UnpackedMemoryValueId::Large((self.0 & 0x7FFF_FFFF) as usize)
        } else {
            UnpackedMemoryValueId::Small(self.0 as i32 - SMALL_VALUE_SHIFT)
        }
    }
}

pub enum UnpackedMemoryValueId {
    Small(i32),
    Large(usize),
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum MemoryValue {
    Small(i32),
    Large([u32; 8]),
}
impl From<[u32; 8]> for MemoryValue {
    fn from(value: [u32; 8]) -> MemoryValue {
        if value[1..].iter().all(|&x| x == 0) && value[0] < SMALL_VALUE_SHIFT as u32 {
            // TODO: Handle small negatives.
            MemoryValue::Small(value[0] as i32)
        } else {
            MemoryValue::Large(value)
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
                val: [3; 8],
            },
        ];
        let memory = Memory::from_iter(entries.iter().cloned());
        assert_eq!(memory.get(0), MemoryValue::Large([1; 8]));
        assert_eq!(memory.get(1), MemoryValue::Small(6));
        assert_eq!(memory.get(2), MemoryValue::Large([3; 8]));
    }
}
