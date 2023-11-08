use super::common::{ Instruction, InstructionFamily, InstructionFormat };

pub fn make_instructions_field_extraction() -> Vec<Instruction<'static>> {
    return vec![
        Instruction::new()
            .set_family(InstructionFamily::FieldExtraction)
            .set_format(InstructionFormat::FieldExtraction)
            .set_opcode(0o170)
            .set_name("ldb"),
        Instruction::new()
            .set_family(InstructionFamily::FieldExtraction)
               .set_format(InstructionFormat::FieldExtraction)
         .set_opcode(0o370)
            .set_name("dpb"),
        Instruction::new()
            .set_family(InstructionFamily::FieldExtraction)
            .set_format(InstructionFormat::FieldExtraction)
            .set_opcode(0o171)
            .set_name("char-ldb"),
        Instruction::new()
            .set_family(InstructionFamily::FieldExtraction)
            .set_format(InstructionFormat::FieldExtraction)
            .set_opcode(0o371)
            .set_name("char-dpb"),
        Instruction::new()
            .set_family(InstructionFamily::FieldExtraction)
            .set_format(InstructionFormat::FieldExtraction)
            .set_opcode(0o172)
            .set_name("%p-ldb"),
        Instruction::new()
            .set_family(InstructionFamily::FieldExtraction)
            .set_format(InstructionFormat::FieldExtraction)
            .set_opcode(0o372)
            .set_name("%p-dpb"),
        Instruction::new()
            .set_family(InstructionFamily::FieldExtraction)
            .set_format(InstructionFormat::FieldExtraction)
            .set_opcode(0o173)
            .set_name("%p-tag-ldb"),
        Instruction::new()
            .set_family(InstructionFamily::FieldExtraction)
            .set_format(InstructionFormat::FieldExtraction)
            .set_opcode(0o373)
            .set_name("%p-tag-dpb")
    ];
}
