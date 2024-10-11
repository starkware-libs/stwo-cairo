use instructions::Instructions;
use mem::Memory;
use serde::{Deserialize, Serialize};

use self::range_check_unit::RangeCheckUnitInput;

mod decode;
pub mod instructions;
pub mod mem;
pub mod plain;
pub mod range_check_unit;
pub mod vm_import;

pub const N_REGISTERS: usize = 3;

// Externally provided inputs.
#[derive(Debug)]
pub struct CairoInput {
    pub instructions: Instructions,
    pub mem: Memory,
    pub public_mem_addresses: Vec<u32>,
    pub range_check9: RangeCheckUnitInput,

    // Builtins.
    pub range_check_builtin: SegmentAddrs,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SegmentAddrs {
    pub begin_addr: u32,
    pub end_addr: u32,
}

impl SegmentAddrs {
    pub fn addresses(&self) -> Vec<u32> {
        (self.begin_addr..self.end_addr).collect()
    }
}
