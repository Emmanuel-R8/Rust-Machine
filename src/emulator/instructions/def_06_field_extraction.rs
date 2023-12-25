use crate::hardware::machine::VirtualMachine;

use super::common::{ Instruction, InstructionFamily, InstructionFormat };

pub fn make_instructions_field_extraction() -> Vec<Instruction> {
    return vec![
        Instruction::new()
            .set_name("ldb".to_string())
            .set_family(InstructionFamily::FieldExtraction)
            .set_format(InstructionFormat::FieldExtraction)
            .set_opcode(0o170)
            .set_exec(Some(VirtualMachine::cpu_ldb)),
        Instruction::new()
            .set_name("dpb".to_string())
            .set_family(InstructionFamily::FieldExtraction)
            .set_format(InstructionFormat::FieldExtraction)
            .set_opcode(0o370)
            .set_exec(Some(VirtualMachine::cpu_dpb)),
        Instruction::new()
            .set_name("char-ldb".to_string())
            .set_family(InstructionFamily::FieldExtraction)
            .set_format(InstructionFormat::FieldExtraction)
            .set_opcode(0o171)
            .set_exec(Some(VirtualMachine::cpu_char_ldb)),
        Instruction::new()
            .set_name("char-dpb".to_string())
            .set_family(InstructionFamily::FieldExtraction)
            .set_format(InstructionFormat::FieldExtraction)
            .set_opcode(0o371)
            .set_exec(Some(VirtualMachine::cpu_char_dpb)),
        Instruction::new()
            .set_name("%p-ldb".to_string())
            .set_family(InstructionFamily::FieldExtraction)
            .set_format(InstructionFormat::FieldExtraction)
            .set_opcode(0o172)
            .set_exec(Some(VirtualMachine::cpu_p_ldb)),
        Instruction::new()
            .set_name("%p-dpb".to_string())
            .set_family(InstructionFamily::FieldExtraction)
            .set_format(InstructionFormat::FieldExtraction)
            .set_opcode(0o372)
            .set_exec(Some(VirtualMachine::cpu_p_dpb)),
        Instruction::new()
            .set_name("%p-tag-ldb".to_string())
            .set_family(InstructionFamily::FieldExtraction)
            .set_format(InstructionFormat::FieldExtraction)
            .set_opcode(0o173)
            .set_exec(Some(VirtualMachine::cpu_p_tag_ldb)),
        Instruction::new()
            .set_name("%p-tag-dpb".to_string())
            .set_family(InstructionFamily::FieldExtraction)
            .set_format(InstructionFormat::FieldExtraction)
            .set_opcode(0o373)
            .set_exec(Some(VirtualMachine::cpu_p_tag_dpb))
    ];
}
