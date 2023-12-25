use crate::common::constants::OpCode;
use crate::hardware::machine::VirtualMachine;

use super::common::{ Instruction, InstructionFamily, InstructionFormat };

pub fn make_instructions_catch() -> Vec<Instruction> {
    return vec![
        Instruction::new()
            .set_name("catch-open".to_string())
            .set_family(InstructionFamily::Catch)
            .set_format(InstructionFormat::Immediate10Bits)
            .set_opcode(OpCode::CatchOpen)
            .set_exec(Some(VirtualMachine::cpu_catch_open)),
        Instruction::new()
            .set_name("catch-close".to_string())
            .set_family(InstructionFamily::Catch)
            .set_format(InstructionFormat::OperandFromStack)
            .set_opcode(OpCode::CatchClose)
            .set_exec(Some(VirtualMachine::cpu_catch_close))
    ];
}
