use super::{Instruction, InstructionInfo};
use classfile::OpCode;

pub struct Dneg;

impl Instruction for Dneg {
    fn run(&self, _codes: &[u8], pc: usize) -> (InstructionInfo, usize) {
        let info = InstructionInfo {
            pc,
            op_code: OpCode::dneg,
            icp: 0,
            wide: false,
        };

        (info, pc + 1)
    }
}
