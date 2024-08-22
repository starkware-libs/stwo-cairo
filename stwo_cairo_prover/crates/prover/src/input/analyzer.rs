use std::path::Path;

use super::decode::Instruction;
use super::instructions::VmState;
use super::mem::{MemConfig, MemoryBuilder};
use super::vm_import::{MemEntryIter, TraceIter};
use crate::input::mem::MemoryValue;

#[allow(clippy::single_match)]
pub fn analyze(mem_path: &Path, trace_path: &Path) {
    let mut trace_file = std::io::BufReader::new(std::fs::File::open(trace_path).unwrap());
    let mut mem_file = std::io::BufReader::new(std::fs::File::open(mem_path).unwrap());
    let mem_config = MemConfig::new((1 << 20) - 1, (1 << 20) - 1);
    let mem = MemoryBuilder::from_iter(mem_config, MemEntryIter(&mut mem_file));
    let mut n_muls = 0;
    let mut n_muls_15 = 0;
    for entry in TraceIter(&mut trace_file) {
        let VmState { ap, fp, pc } = entry.into();
        let instruction = mem.get(pc).as_u64();
        let instruction = Instruction::decode(instruction);
        match instruction {
            Instruction {
                offset0: _,
                offset1,
                offset2,
                op0_base_fp,
                op1_imm,
                op1_base_fp,
                op1_base_ap,
                res_mul: true,
                ..
            } => {
                n_muls += 1;
                let op0 = if op0_base_fp { fp } else { ap };
                let op1 = if op1_imm {
                    pc + 1
                } else if op1_base_fp {
                    fp
                } else if op1_base_ap {
                    ap
                } else {
                    0
                };
                let MemoryValue::Small(op0) = mem.get((op0 as i32 + offset1 as i32) as u32) else {
                    continue;
                };
                let MemoryValue::Small(op1) = mem.get((op1 as i32 + offset2 as i32) as u32) else {
                    continue;
                };
                if ((op0 < (1 << 15)) || (op0 > -(1 << 15)))
                    && ((op1 < (1 << 15)) || (op1 > -(1 << 15)))
                {
                    n_muls_15 += 1;
                }
            }
            _ => {}
        }
    }

    println!("n_muls = {}", n_muls);
    println!("n_muls_15 = {}", n_muls_15);
}

#[test]
fn run_analyzer() {
    analyze(
        Path::new("/home/spapini/Downloads/mem_file_sn"),
        Path::new("/home/spapini/Downloads/trace_file_sn"),
    );
}
