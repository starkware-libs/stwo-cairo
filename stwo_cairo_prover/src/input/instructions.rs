use cairo_vm::types::instruction::Opcode;
use cairo_vm::vm::decoding::decoder::decode_instruction;

use super::mem::Memory;
use super::vm_import::TraceEntry;

// TODO(spapini): Move this:
pub struct RetInput {
    pub pc: u32,
    pub ap: u32,
    pub fp: u32,
    pub new_pc: u32,
    pub new_fp: u32,
}

#[derive(Default)]
pub struct Instructions {
    pub ret: Vec<RetInput>,
    // ...
}
impl Instructions {
    pub fn from_iter(iter: impl Iterator<Item = TraceEntry>, mem: &Memory) -> Self {
        let mut res = Self::default();
        for TraceEntry { ap, fp, pc } in iter {
            let instruction = mem.get(pc).as_64();
            let instruction = decode_instruction(instruction).unwrap();
            match instruction.opcode {
                Opcode::Ret => {
                    assert_eq!(instruction.off0, -2);
                    assert_eq!(instruction.off1, -1);
                    assert_eq!(instruction.off2, -1);
                    let new_fp = mem.get(fp - 1).as_64() as u32;
                    let new_pc = mem.get(fp - 2).as_64() as u32;
                    res.ret.push(RetInput {
                        pc: pc as u32,
                        ap: ap as u32,
                        fp: fp as u32,
                        new_pc,
                        new_fp,
                    });
                }
                _ => {
                    continue;
                }
            }
        }
        res
    }
}
