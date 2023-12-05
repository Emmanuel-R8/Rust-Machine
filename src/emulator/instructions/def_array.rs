use crate::hardware::machine::VirtualMachine;

use super::common::{ Instruction, InstructionFamily, InstructionFormat };

pub fn make_instructions_array() -> Vec<Instruction> {
    return vec![
        Instruction::new()
            .set_name("aref-1".to_string())
            .set_family(InstructionFamily::Array)
            .set_format(InstructionFormat::OperandFromStack)
            .set_opcode(0o312)
            .set_exec(Some(VirtualMachine::cpu_aref_1)),
        Instruction::new()
            .set_name("aset-1".to_string())
            .set_family(InstructionFamily::Array)
            .set_format(InstructionFormat::OperandFromStack)
            .set_opcode(0o310)
            .set_exec(Some(VirtualMachine::cpu_aset_1)),
        Instruction::new()
            .set_name("aloc-1".to_string())
            .set_family(InstructionFamily::Array)
            .set_format(InstructionFormat::OperandFromStack)
            .set_opcode(0o313)
            .set_exec(Some(VirtualMachine::cpu_aloc_1)),
        Instruction::new()
            .set_name("setup-1d-array".to_string())
            .set_family(InstructionFamily::Array)
            .set_format(InstructionFormat::OperandFromStack)
            .set_opcode(0o003)
            .set_exec(Some(VirtualMachine::cpu_setup_1d_array)),
        Instruction::new()
            .set_name("setup-force-1d-array".to_string())
            .set_family(InstructionFamily::Array)
            .set_format(InstructionFormat::OperandFromStack)
            .set_opcode(0o004)
            .set_exec(Some(VirtualMachine::cpu_setup_force_1d_array)),
        Instruction::new()
            .set_name("fast-aref-1".to_string())
            .set_family(InstructionFamily::Array)
            .set_format(InstructionFormat::OperandFromStack)
            .set_opcode(0o350)
            .set_exec(Some(VirtualMachine::cpu_fast_aref_1)),
        Instruction::new()
            .set_name("fast-aset-1".to_string())
            .set_family(InstructionFamily::Array)
            .set_format(InstructionFormat::OperandFromStack)
            .set_opcode(0o351)
            .set_exec(Some(VirtualMachine::cpu_fast_aset_1)),
        Instruction::new()
            .set_name("array-leader".to_string())
            .set_family(InstructionFamily::Array)
            .set_format(InstructionFormat::OperandFromStack)
            .set_opcode(0o316)
            .set_exec(Some(VirtualMachine::cpu_array_leader)),
        Instruction::new()
            .set_name("store-array-leader".to_string())
            .set_family(InstructionFamily::Array)
            .set_format(InstructionFormat::OperandFromStack)
            .set_opcode(0o314)
            .set_exec(Some(VirtualMachine::cpu_store_array_leader)),
        Instruction::new()
            .set_name("aloc-leader".to_string())
            .set_family(InstructionFamily::Array)
            .set_format(InstructionFormat::OperandFromStack)
            .set_opcode(0o317)
            .set_exec(Some(VirtualMachine::cpu_aloc_leader))
    ];
}
