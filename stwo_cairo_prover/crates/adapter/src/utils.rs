use std::sync::atomic::{AtomicU32, AtomicU8, Ordering};

use itertools::Itertools;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct AtomicBitmap {
    bits: Vec<AtomicU8>,
    highest_set_bit: AtomicU32,
}

impl AtomicBitmap {
    pub fn new(size: usize) -> Self {
        let byte_size = size.div_ceil(8);
        let bits = std::iter::repeat_with(|| AtomicU8::new(0))
            .take(byte_size)
            .collect();
        Self {
            bits,
            highest_set_bit: AtomicU32::new(0),
        }
    }

    pub fn set_1(&self, index: u32) {
        let byte_index = index as usize / 8;
        assert!(byte_index < self.bits.len());

        let mask = 1u8 << (index % 8);
        self.bits[byte_index].fetch_or(mask, Ordering::Relaxed);
        self.highest_set_bit.fetch_max(index, Ordering::Relaxed);
    }

    pub fn u32s(self) -> IndexIterator {
        let highest_set_bit = self.highest_set_bit.into_inner();
        let byte_size = (highest_set_bit + 1).div_ceil(8);
        let bits = self
            .bits
            .into_iter()
            .take(byte_size as usize)
            .map(|b| b.into_inner())
            .collect_vec();
        let first_byte = *bits.first().unwrap_or(&0);
        IndexIterator {
            bits,
            byte_index: 0,
            current_byte: first_byte,
        }
    }
}

impl Clone for AtomicBitmap {
    fn clone(&self) -> Self {
        Self {
            bits: self
                .bits
                .iter()
                .map(|b| AtomicU8::new(b.load(Ordering::Relaxed)))
                .collect(),
            highest_set_bit: AtomicU32::new(self.highest_set_bit.load(Ordering::Relaxed)),
        }
    }
}

pub struct IndexIterator {
    bits: Vec<u8>,
    byte_index: u32,
    current_byte: u8,
}

impl Iterator for IndexIterator {
    type Item = u32;
    fn next(&mut self) -> Option<u32> {
        while self.current_byte == 0 {
            self.byte_index += 1;
            if self.byte_index >= self.bits.len() as u32 {
                return None;
            }
            self.current_byte = self.bits[self.byte_index as usize];
        }

        let bit_index = self.current_byte.trailing_zeros();
        self.current_byte &= !(1 << bit_index);

        Some(self.byte_index * 8 + bit_index)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_index_iterator() {
        let bitmap = AtomicBitmap::new(100);
        bitmap.set_1(0);
        bitmap.set_1(1);
        bitmap.set_1(2);
        bitmap.set_1(4);
        bitmap.set_1(5);
        bitmap.set_1(80);

        let mut iterator = bitmap.u32s();

        assert_eq!(iterator.next(), Some(0));
        assert_eq!(iterator.next(), Some(1));
        assert_eq!(iterator.next(), Some(2));
        assert_eq!(iterator.next(), Some(4));
        assert_eq!(iterator.next(), Some(5));
        assert_eq!(iterator.next(), Some(80));
        assert_eq!(iterator.next(), None);
    }
}
