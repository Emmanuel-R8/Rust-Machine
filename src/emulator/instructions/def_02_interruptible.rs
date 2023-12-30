use crate::common::constants::OpCode;
use crate::hardware::machine::VirtualMachine;

use super::common::{ Instruction, InstructionFamily, InstructionFormat };

pub fn make_instructions_interruptible() -> Vec<Instruction> {
    return vec![
        Instruction::new()
            .set_name("rgetf".to_string())
            .set_family(InstructionFamily::Interruptible)
            .set_format(InstructionFormat::OperandFromStack)
            .set_opcode(OpCode::Rgetf)
            .set_exec(Some(VirtualMachine::cpu_rgetf)),
        Instruction::new()
            .set_name("member".to_string())
            .set_family(InstructionFamily::Interruptible)
            .set_format(InstructionFormat::OperandFromStack)
            .set_opcode(OpCode::Member)
            .set_exec(Some(VirtualMachine::cpu_member)),
        Instruction::new()
            .set_name("assoc".to_string())
            .set_family(InstructionFamily::Interruptible)
            .set_format(InstructionFormat::OperandFromStack)
            .set_opcode(OpCode::Assoc)
            .set_exec(Some(VirtualMachine::cpu_assoc))
    ];
}
