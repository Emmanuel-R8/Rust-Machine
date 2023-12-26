use crate::common::constants::OpCode;
use crate::hardware::machine::VirtualMachine;

use super::common::{Instruction, InstructionFamily, InstructionFormat};

pub fn make_instructions_instance_variable() -> Vec<Instruction> {
    return vec![
        Instruction::new()
            .set_name("push-instance-variable".to_string())
            .set_family(InstructionFamily::InstanceVariableAccess)
            .set_format(InstructionFormat::OperandFromStackImmediate)
            .set_opcode(OpCode::PushInstanceVariable)
            .set_exec(Some(VirtualMachine::cpu_push_instance_variable)),
        Instruction::new()
            .set_name("pop-instance-variable".to_string())
            .set_family(InstructionFamily::InstanceVariableAccess)
            .set_format(InstructionFormat::OperandFromStackImmediate)
            .set_opcode(OpCode::PopInstanceVariable)
            .set_exec(Some(VirtualMachine::cpu_pop_instance_variable)),
        Instruction::new()
            .set_name("movem-instance-variable".to_string())
            .set_family(InstructionFamily::InstanceVariableAccess)
            .set_format(InstructionFormat::OperandFromStackImmediate)
            .set_opcode(OpCode::MovemInstanceVariable)
            .set_exec(Some(VirtualMachine::cpu_movem_instance_variable)),
        Instruction::new()
            .set_name("push-address-instance-variable".to_string())
            .set_family(InstructionFamily::InstanceVariableAccess)
            .set_format(InstructionFormat::OperandFromStackImmediate)
            .set_opcode(OpCode::PushAddressInstanceVariable)
            .set_exec(Some(VirtualMachine::cpu_push_address_instance_variable)),
        Instruction::new()
            .set_name("push-instance-variable-ordered".to_string())
            .set_family(InstructionFamily::InstanceVariableAccess)
            .set_format(InstructionFormat::OperandFromStackImmediate)
            .set_opcode(OpCode::PushAddressInstanceVariableOrdered)
            .set_exec(Some(VirtualMachine::cpu_push_instance_variable_ordered)),
        Instruction::new()
            .set_name("pop-instance-variable-ordered".to_string())
            .set_family(InstructionFamily::InstanceVariableAccess)
            .set_format(InstructionFormat::OperandFromStackImmediate)
            .set_opcode(OpCode::PopInstanceVariableOrdered)
            .set_exec(Some(VirtualMachine::cpu_pop_instance_variable_ordered)),
        Instruction::new()
            .set_name("movem-instance-variable-ordered".to_string())
            .set_family(InstructionFamily::InstanceVariableAccess)
            .set_format(InstructionFormat::OperandFromStackImmediate)
            .set_opcode(OpCode::MovemInstanceVariableOrdered)
            .set_exec(Some(VirtualMachine::cpu_movem_instance_variable_ordered)),
        Instruction::new()
            .set_name("push-address-instance-variable-ordered".to_string())
            .set_family(InstructionFamily::InstanceVariableAccess)
            .set_format(InstructionFormat::OperandFromStackImmediate)
            .set_opcode(OpCode::PushAddressInstanceVariableOrdered)
            .set_exec(Some(
                VirtualMachine::cpu_push_address_instance_variable_ordered,
            )),
        Instruction::new()
            .set_name("%instance-ref".to_string())
            .set_family(InstructionFamily::InstanceVariableAccess)
            .set_format(InstructionFormat::OperandFromStack)
            .set_opcode(OpCode::InstanceRef)
            .set_exec(Some(VirtualMachine::cpu_instance_ref)),
        Instruction::new()
            .set_name("%instance-set".to_string())
            .set_family(InstructionFamily::InstanceVariableAccess)
            .set_format(InstructionFormat::OperandFromStack)
            .set_opcode(OpCode::InstanceSet)
            .set_exec(Some(VirtualMachine::cpu_instance_set)),
        Instruction::new()
            .set_name("%instance-loc".to_string())
            .set_family(InstructionFamily::InstanceVariableAccess)
            .set_format(InstructionFormat::OperandFromStack)
            .set_opcode(OpCode::InstanceLoc)
            .set_exec(Some(VirtualMachine::cpu_instance_loc)),
    ];
}
