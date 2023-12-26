use crate::common::constants::OpCode;
use crate::hardware::machine::VirtualMachine;

use super::common::{Instruction, InstructionFamily, InstructionFormat};

pub fn make_instructions_list() -> Vec<Instruction> {
    return vec![
        Instruction::new()
            .set_name("car".to_string())
            .set_family(InstructionFamily::List)
            .set_format(InstructionFormat::OperandFromStack)
            .set_opcode(OpCode::Car)
            .set_arg_count(1)
            .set_ret_count(1)
            .set_exec(Some(VirtualMachine::cpu_car)),
        Instruction::new()
            .set_name("cdr".to_string())
            .set_family(InstructionFamily::List)
            .set_format(InstructionFormat::OperandFromStack)
            .set_opcode(OpCode::Cdr)
            .set_arg_count(1)
            .set_ret_count(1)
            .set_exec(Some(VirtualMachine::cpu_cdr)),
        Instruction::new()
            .set_name("set-to-car".to_string())
            .set_family(InstructionFamily::List)
            .set_format(InstructionFormat::OperandFromStack)
            .set_opcode(OpCode::SetToCar)
            .set_arg_count(1)
            .set_ret_count(0)
            .set_exec(Some(VirtualMachine::cpu_set_to_car)),
        Instruction::new()
            .set_name("set-to-cdr".to_string())
            .set_family(InstructionFamily::List)
            .set_format(InstructionFormat::OperandFromStack)
            .set_opcode(OpCode::SetToCdr)
            .set_arg_count(1)
            .set_ret_count(0)
            .set_exec(Some(VirtualMachine::cpu_set_to_cdr)),
        Instruction::new()
            .set_name("set-to-cdr-push-car".to_string())
            .set_family(InstructionFamily::List)
            .set_format(InstructionFormat::OperandFromStack)
            .set_opcode(OpCode::SetToCdrPushCar)
            .set_arg_count(1)
            .set_ret_count(1)
            .set_exec(Some(VirtualMachine::cpu_set_to_cdr_push_car)),
        Instruction::new()
            .set_name("rplaca".to_string())
            .set_family(InstructionFamily::List)
            .set_format(InstructionFormat::OperandFromStack)
            .set_opcode(OpCode::Rplaca)
            .set_arg_count(2)
            .set_ret_count(0)
            .set_exec(Some(VirtualMachine::cpu_rplaca)),
        Instruction::new()
            .set_name("rplacd".to_string())
            .set_family(InstructionFamily::List)
            .set_format(InstructionFormat::OperandFromStack)
            .set_opcode(OpCode::Rplacd)
            .set_arg_count(2)
            .set_ret_count(0)
            .set_exec(Some(VirtualMachine::cpu_rplacd)),
    ];
}
