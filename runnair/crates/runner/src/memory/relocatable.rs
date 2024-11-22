use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Relocatable {
    segment: isize,
    offset: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MaybeRelocatable<T: From<u32>> {
    Relocatable(Relocatable),
    Absolute(T),
}

pub type RelocationTable = HashMap<isize, u32>;

impl Relocatable {
    pub fn relocate(&self, table: &RelocationTable) -> u32 {
        table[&self.segment] + self.offset
    }
}

impl<T: From<u32> + Copy> MaybeRelocatable<T> {
    pub fn relocate(&self, table: &RelocationTable) -> T {
        match self {
            MaybeRelocatable::Relocatable(relocatable) => relocatable.relocate(table).into(),
            MaybeRelocatable::Absolute(absolute) => *absolute,
        }
    }
}

impl From<(isize, u32)> for Relocatable {
    fn from((segment, offset): (isize, u32)) -> Self {
        Relocatable { segment, offset }
    }
}

impl<T: From<u32>> From<Relocatable> for MaybeRelocatable<T> {
    fn from(relocatable: Relocatable) -> Self {
        MaybeRelocatable::Relocatable(relocatable)
    }
}

impl<T: From<u32>> From<T> for MaybeRelocatable<T> {
    fn from(value: T) -> Self {
        MaybeRelocatable::Absolute(value)
    }
}
