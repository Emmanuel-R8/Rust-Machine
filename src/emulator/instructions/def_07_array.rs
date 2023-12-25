use crate::hardware::machine::VirtualMachine;
use crate::common::constants::OpCode;

use super::common::{ Instruction, InstructionFamily, InstructionFormat };

pub fn make_instructions_array() -> Vec<Instruction> {
    return vec![
        Instruction::new()
            .set_name("aref-1".to_string())
            .set_family(InstructionFamily::Array)
            .set_format(InstructionFormat::OperandFromStack)
            .set_opcode(OpCode::Aref1)
            .set_exec(Some(VirtualMachine::cpu_aref_1)),
        Instruction::new()
            .set_name("aset-1".to_string())
            .set_family(InstructionFamily::Array)
            .set_format(InstructionFormat::OperandFromStack)
            .set_opcode(OpCode::Aset1)
            .set_exec(Some(VirtualMachine::cpu_aset_1)),
        Instruction::new()
            .set_name("aloc-1".to_string())
            .set_family(InstructionFamily::Array)
            .set_format(InstructionFormat::OperandFromStack)
            .set_opcode(OpCode::Aloc1)
            .set_exec(Some(VirtualMachine::cpu_aloc_1)),
        Instruction::new()
            .set_name("setup-1d-array".to_string())
            .set_family(InstructionFamily::Array)
            .set_format(InstructionFormat::OperandFromStack)
            .set_opcode(OpCode::Setup1DArray)
            .set_exec(Some(VirtualMachine::cpu_setup_1d_array)),
        Instruction::new()
            .set_name("setup-force-1d-array".to_string())
            .set_family(InstructionFamily::Array)
            .set_format(InstructionFormat::OperandFromStack)
            .set_opcode(OpCode::SetupForce1DArray)
            .set_exec(Some(VirtualMachine::cpu_setup_force_1d_array)),
        Instruction::new()
            .set_name("fast-aref-1".to_string())
            .set_family(InstructionFamily::Array)
            .set_format(InstructionFormat::OperandFromStack)
            .set_opcode(OpCode::FastAref1)
            .set_exec(Some(VirtualMachine::cpu_fast_aref_1)),
        Instruction::new()
            .set_name("fast-aset-1".to_string())
            .set_family(InstructionFamily::Array)
            .set_format(InstructionFormat::OperandFromStack)
            .set_opcode(OpCode::FastAset1)
            .set_exec(Some(VirtualMachine::cpu_fast_aset_1)),
        Instruction::new()
            .set_name("array-leader".to_string())
            .set_family(InstructionFamily::Array)
            .set_format(InstructionFormat::OperandFromStack)
            .set_opcode(OpCode::ArrayLeader)
            .set_exec(Some(VirtualMachine::cpu_array_leader)),
        Instruction::new()
            .set_name("store-array-leader".to_string())
            .set_family(InstructionFamily::Array)
            .set_format(InstructionFormat::OperandFromStack)
            .set_opcode(OpCode::StoreArrayLeader)
            .set_exec(Some(VirtualMachine::cpu_store_array_leader)),
        Instruction::new()
            .set_name("aloc-leader".to_string())
            .set_family(InstructionFamily::Array)
            .set_format(InstructionFormat::OperandFromStack)
            .set_opcode(OpCode::AlocLeader)
            .set_exec(Some(VirtualMachine::cpu_aloc_leader))
    ];
}
