use super::{Instruction, InstructionInfo};
use classfile::OpCode;

pub struct Ifnull;

impl Instruction for Ifnull {
    fn run(&self, _codes: &[u8], pc: usize) -> (InstructionInfo, usize) {
        let info = InstructionInfo {
            pc,
            op_code: OpCode::ifnull,
            icp: 0,
            wide: false,
        };

        (info, pc + 3)
    }
}
