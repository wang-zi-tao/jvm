#![allow(non_camel_case_types)]
use super::{Instruction, InstructionInfo};
use classfile::OpCode;

pub struct Fload_0;

impl Instruction for Fload_0 {
    fn run(&self, _codes: &[u8], pc: usize) -> (InstructionInfo, usize) {
        let info = InstructionInfo {
            pc,
            op_code: OpCode::fload_0,
            icp: 0,
            wide: false,
        };

        (info, pc + 1)
    }
}
