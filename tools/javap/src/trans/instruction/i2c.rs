use super::{Instruction, InstructionInfo};
use classfile::OpCode;

pub struct I2C;

impl Instruction for I2C {
    fn run(&self, _codes: &[u8], pc: usize) -> (InstructionInfo, usize) {
        let info = InstructionInfo {
            pc,
            op_code: OpCode::i2c,
            icp: 0,
        };

        (info, pc + 1)
    }
}