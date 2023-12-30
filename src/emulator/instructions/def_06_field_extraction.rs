use crate::common::constants::OpCode;
use crate::hardware::machine::VirtualMachine;

use super::common::{ Instruction, InstructionFamily, InstructionFormat };

pub fn make_instructions_field_extraction() -> Vec<Instruction> {
    return vec![
        Instruction::new()
            .set_name("ldb".to_string())
            .set_family(InstructionFamily::FieldExtraction)
            .set_format(InstructionFormat::FieldExtraction)
            .set_opcode(OpCode::Ldb)
            .set_exec(Some(VirtualMachine::cpu_ldb)),
        Instruction::new()
            .set_name("dpb".to_string())
            .set_family(InstructionFamily::FieldExtraction)
            .set_format(InstructionFormat::FieldExtraction)
            .set_opcode(OpCode::Dpb)
            .set_exec(Some(VirtualMachine::cpu_dpb)),
        Instruction::new()
            .set_name("char-ldb".to_string())
            .set_family(InstructionFamily::FieldExtraction)
            .set_format(InstructionFormat::FieldExtraction)
            .set_opcode(OpCode::CharLdb)
            .set_exec(Some(VirtualMachine::cpu_char_ldb)),
        Instruction::new()
            .set_name("char-dpb".to_string())
            .set_family(InstructionFamily::FieldExtraction)
            .set_format(InstructionFormat::FieldExtraction)
            .set_opcode(OpCode::CharDpb)
            .set_exec(Some(VirtualMachine::cpu_char_dpb)),
        Instruction::new()
            .set_name("%p-ldb".to_string())
            .set_family(InstructionFamily::FieldExtraction)
            .set_format(InstructionFormat::FieldExtraction)
            .set_opcode(OpCode::PLdb)
            .set_exec(Some(VirtualMachine::cpu_p_ldb)),
        Instruction::new()
            .set_name("%p-dpb".to_string())
            .set_family(InstructionFamily::FieldExtraction)
            .set_format(InstructionFormat::FieldExtraction)
            .set_opcode(OpCode::PDpb)
            .set_exec(Some(VirtualMachine::cpu_p_dpb)),
        Instruction::new()
            .set_name("%p-tag-ldb".to_string())
            .set_family(InstructionFamily::FieldExtraction)
            .set_format(InstructionFormat::FieldExtraction)
            .set_opcode(OpCode::PTagLdb)
            .set_exec(Some(VirtualMachine::cpu_p_tag_ldb)),
        Instruction::new()
            .set_name("%p-tag-dpb".to_string())
            .set_family(InstructionFamily::FieldExtraction)
            .set_format(InstructionFormat::FieldExtraction)
            .set_opcode(OpCode::PTagDpb)
            .set_exec(Some(VirtualMachine::cpu_p_tag_dpb))
    ];
}
