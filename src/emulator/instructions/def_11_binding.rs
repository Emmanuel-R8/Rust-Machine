use crate::common::constants::OpCode;
use crate::hardware::machine::VirtualMachine;

use super::common::{ Instruction, InstructionFamily, InstructionFormat };

pub fn make_instructions_binding() -> Vec<Instruction> {
    return vec![
        Instruction::new()
            .set_name("bind-locative-to-value".to_string())
            .set_family(InstructionFamily::Binding)
            .set_format(InstructionFormat::OperandFromStack)
            .set_opcode(OpCode::BindLocativeToValue)
            .set_exec(Some(VirtualMachine::cpu_bind_locative_to_value)),
        Instruction::new()
            .set_name("bind-locative".to_string())
            .set_family(InstructionFamily::Binding)
            .set_format(InstructionFormat::OperandFromStack)
            .set_opcode(OpCode::BindLocative)
            .set_exec(Some(VirtualMachine::cpu_bind_locative)),
        Instruction::new()
            .set_name("unbind-n".to_string())
            .set_family(InstructionFamily::Binding)
            .set_format(InstructionFormat::OperandFromStack)
            .set_opcode(OpCode::UnbindN)
            .set_exec(Some(VirtualMachine::cpu_unbind_n)),
        Instruction::new()
            .set_name("%restore-binding-stack".to_string())
            .set_family(InstructionFamily::Binding)
            .set_format(InstructionFormat::OperandFromStack)
            .set_opcode(OpCode::RestoreBindingStack)
            .set_exec(Some(VirtualMachine::cpu_restore_binding_stack))
    ];
}
