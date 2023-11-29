use crate::hardware::machine::VirtualMachine;

use super::common::{ Instruction, InstructionFamily, InstructionFormat };

pub fn make_instructions_block() -> Vec<Instruction> {
    return vec![
        Instruction::new()
            .set_name("%block-1-read".to_string())
            .set_family(InstructionFamily::Block)
            .set_format(InstructionFormat::Immediate10Bits)
            .set_opcode(0o120)
            .set_exec(Some(VirtualMachine::cpu_block_1_read)),
        Instruction::new()
            .set_name("%block-2-read".to_string())
            .set_family(InstructionFamily::Block)
            .set_format(InstructionFormat::Immediate10Bits)
            .set_opcode(0o121)
            .set_exec(Some(VirtualMachine::cpu_block_2_read)),
        Instruction::new()
            .set_name("%block-3-read".to_string())
            .set_family(InstructionFamily::Block)
            .set_format(InstructionFormat::Immediate10Bits)
            .set_opcode(0o122)
            .set_exec(Some(VirtualMachine::cpu_block_3_read)),
        Instruction::new()
            .set_name("%block-4-read".to_string())
            .set_family(InstructionFamily::Block)
            .set_format(InstructionFormat::Immediate10Bits)
            .set_opcode(0o123)
            .set_exec(Some(VirtualMachine::cpu_block_4_read)),
        Instruction::new()
            .set_name("%block-1-read-shift".to_string())
            .set_family(InstructionFamily::Block)
            .set_format(InstructionFormat::Immediate10Bits)
            .set_opcode(0o124)
            .set_exec(Some(VirtualMachine::cpu_block_1_read_shift)),
        Instruction::new()
            .set_name("%block-2-read-shift".to_string())
            .set_family(InstructionFamily::Block)
            .set_format(InstructionFormat::Immediate10Bits)
            .set_opcode(0o125)
            .set_exec(Some(VirtualMachine::cpu_block_2_read_shift)),
        Instruction::new()
            .set_name("%block-3-read-shift".to_string())
            .set_family(InstructionFamily::Block)
            .set_format(InstructionFormat::Immediate10Bits)
            .set_opcode(0o126)
            .set_exec(Some(VirtualMachine::cpu_block_3_read_shift)),
        Instruction::new()
            .set_name("%block-4-read-shift".to_string())
            .set_family(InstructionFamily::Block)
            .set_format(InstructionFormat::Immediate10Bits)
            .set_opcode(0o127)
            .set_exec(Some(VirtualMachine::cpu_block_4_read_shift)),
        Instruction::new()
            .set_name("%block-1-read-alu".to_string())
            .set_family(InstructionFamily::Block)
            .set_format(InstructionFormat::OperandFromStack)
            .set_opcode(0o160)
            .set_exec(Some(VirtualMachine::cpu_block_1_read_alu)),
        Instruction::new()
            .set_name("%block-2-read-alu".to_string())
            .set_family(InstructionFamily::Block)
            .set_format(InstructionFormat::OperandFromStack)
            .set_opcode(0o161)
            .set_exec(Some(VirtualMachine::cpu_block_2_read_alu)),
        Instruction::new()
            .set_name("%block-3-read-alu".to_string())
            .set_family(InstructionFamily::Block)
            .set_format(InstructionFormat::OperandFromStack)
            .set_opcode(0o162)
            .set_exec(Some(VirtualMachine::cpu_block_3_read_alu)),
        Instruction::new()
            .set_name("%block-4-read-alu".to_string())
            .set_family(InstructionFamily::Block)
            .set_format(InstructionFormat::OperandFromStack)
            .set_opcode(0o163)
            .set_exec(Some(VirtualMachine::cpu_block_4_read_alu)),
        Instruction::new()
            .set_name("%block-1-test".to_string())
            .set_family(InstructionFamily::Block)
            .set_format(InstructionFormat::Immediate10Bits)
            .set_opcode(0o130)
            .set_exec(Some(VirtualMachine::cpu_block_1_test)),
        Instruction::new()
            .set_name("%block-2-test".to_string())
            .set_family(InstructionFamily::Block)
            .set_format(InstructionFormat::Immediate10Bits)
            .set_opcode(0o131)
            .set_exec(Some(VirtualMachine::cpu_block_2_test)),
        Instruction::new()
            .set_name("%block-3-test".to_string())
            .set_family(InstructionFamily::Block)
            .set_format(InstructionFormat::Immediate10Bits)
            .set_opcode(0o132)
            .set_exec(Some(VirtualMachine::cpu_block_3_test)),
        Instruction::new()
            .set_name("%block-4-test".to_string())
            .set_family(InstructionFamily::Block)
            .set_format(InstructionFormat::Immediate10Bits)
            .set_opcode(0o133)
            .set_exec(Some(VirtualMachine::cpu_block_4_test)),
        Instruction::new()
            .set_name("%block-1-write".to_string())
            .set_family(InstructionFamily::Block)
            .set_format(InstructionFormat::OperandFromStack)
            .set_opcode(0o030)
            .set_exec(Some(VirtualMachine::cpu_block_1_write)),
        Instruction::new()
            .set_name("%block-2-write".to_string())
            .set_family(InstructionFamily::Block)
            .set_format(InstructionFormat::OperandFromStack)
            .set_opcode(0o031)
            .set_exec(Some(VirtualMachine::cpu_block_2_write)),
        Instruction::new()
            .set_name("%block-3-write".to_string())
            .set_family(InstructionFamily::Block)
            .set_format(InstructionFormat::OperandFromStack)
            .set_opcode(0o032)
            .set_exec(Some(VirtualMachine::cpu_block_3_write)),
        Instruction::new()
            .set_name("%block-4-write".to_string())
            .set_family(InstructionFamily::Block)
            .set_format(InstructionFormat::OperandFromStack)
            .set_opcode(0o033)
            .set_exec(Some(VirtualMachine::cpu_block_4_write))
    ];
}
