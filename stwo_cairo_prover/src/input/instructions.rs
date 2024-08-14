use cairo_vm::types::instruction::{ApUpdate, Instruction, Opcode};
use cairo_vm::vm::decoding::decoder::decode_instruction;

use super::mem::Memory;
use super::vm_import::TraceEntry;

// TODO(spapini): Move this:
pub struct VmState {
    pub pc: u32,
    pub ap: u32,
    pub fp: u32,
}

#[derive(Default)]
pub struct Instructions {
    pub ret: Vec<VmState>,
    pub add_ap: Vec<VmState>,
    // ...
}
impl Instructions {
    pub fn from_iter(iter: impl Iterator<Item = TraceEntry>, mem: &Memory) -> Self {
        let mut res = Self::default();
        for TraceEntry { ap, fp, pc } in iter {
            let ap = ap as u32;
            let fp = fp as u32;
            let pc = pc as u32;
            let state = VmState { pc, ap, fp };
            let instruction = mem.get(pc).as_u64();
            let instruction = decode_instruction(instruction).unwrap();
            match instruction {
                Instruction {
                    opcode: Opcode::Ret,
                    ..
                } => {
                    assert_eq!(instruction.off0, -2);
                    assert_eq!(instruction.off1, -1);
                    assert_eq!(instruction.off2, -1);
                    res.ret.push(state)
                }
                Instruction {
                    ap_update: ApUpdate::Add,
                    ..
                } => {
                    // TODO: asserts.
                    res.add_ap.push(state)
                }
                _ => {
                    continue;
                }
            }
        }
        res
    }
}
