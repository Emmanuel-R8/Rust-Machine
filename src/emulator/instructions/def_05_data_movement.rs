use crate::common::constants::OpCode;
use crate::hardware::machine::VirtualMachine;

use super::common::{Instruction, InstructionFamily, InstructionFormat};

pub fn make_instructions_data_movement() -> Vec<Instruction> {
    return vec![
        Instruction::new()
            .set_name("push".to_string())
            .set_family(InstructionFamily::DataMovement)
            .set_format(InstructionFormat::OperandFromStack)
            .set_opcode(OpCode::Push)
            .set_exec(Some(VirtualMachine::cpu_push)),
        Instruction::new()
            .set_name("pop".to_string())
            .set_family(InstructionFamily::DataMovement)
            .set_format(InstructionFormat::OperandFromStack)
            .set_opcode(OpCode::Pop)
            .set_exec(Some(VirtualMachine::cpu_pop)),
        Instruction::new()
            .set_name("movem".to_string())
            .set_family(InstructionFamily::DataMovement)
            .set_format(InstructionFormat::OperandFromStack)
            .set_opcode(OpCode::Movem)
            .set_exec(Some(VirtualMachine::cpu_movem)),
        Instruction::new()
            .set_name("push-n-nils".to_string())
            .set_family(InstructionFamily::DataMovement)
            .set_format(InstructionFormat::OperandFromStack)
            .set_opcode(OpCode::PushNNils)
            .set_exec(Some(VirtualMachine::cpu_push_n_nils)),
        Instruction::new()
            .set_name("push-address".to_string())
            .set_family(InstructionFamily::DataMovement)
            .set_format(InstructionFormat::OperandFromStack)
            .set_opcode(OpCode::PushAddress)
            .set_exec(Some(VirtualMachine::cpu_push_address)),
        Instruction::new()
            .set_name("set-sp-to-address".to_string())
            .set_family(InstructionFamily::DataMovement)
            .set_format(InstructionFormat::OperandFromStack)
            .set_opcode(OpCode::SetSpToAddress)
            .set_exec(Some(VirtualMachine::cpu_set_sp_to_address)),
        Instruction::new()
            .set_name("set-sp-to-address-save-tos".to_string())
            .set_family(InstructionFamily::DataMovement)
            .set_format(InstructionFormat::OperandFromStack)
            .set_opcode(OpCode::SetSpToAddressSaveTos)
            .set_exec(Some(VirtualMachine::cpu_set_sp_to_address_save_tos)),
        Instruction::new()
            .set_name("push-address-sp-relative".to_string())
            .set_family(InstructionFamily::DataMovement)
            .set_format(InstructionFormat::OperandFromStack)
            .set_opcode(OpCode::PushAddressSpRelative)
            .set_exec(Some(VirtualMachine::cpu_push_address_sp_relative)),
        Instruction::new()
            .set_name("stack-blt".to_string())
            .set_family(InstructionFamily::DataMovement)
            .set_format(InstructionFormat::OperandFromStack)
            .set_opcode(OpCode::StackBlt)
            .set_exec(Some(VirtualMachine::cpu_stack_blt)),
        Instruction::new()
            .set_name("stack-blt-address".to_string())
            .set_family(InstructionFamily::DataMovement)
            .set_format(InstructionFormat::OperandFromStack)
            .set_opcode(OpCode::StackBltAddress)
            .set_exec(Some(VirtualMachine::cpu_stack_blt_address)),
    ];
}
