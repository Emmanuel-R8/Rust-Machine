use super::common::{ Instruction, InstructionFamily, InstructionFormat };

pub fn make_instructions_field_extraction() -> Vec<Instruction<'static>> {
    return vec![
        Instruction::new()
            .set_name("ldb")
            .set_family(InstructionFamily::FieldExtraction)
            .set_format(InstructionFormat::FieldExtraction)
            .set_opcode(0o170),
        Instruction::new()
            .set_name("dpb")
            .set_family(InstructionFamily::FieldExtraction)
            .set_format(InstructionFormat::FieldExtraction)
            .set_opcode(0o370),
        Instruction::new()
            .set_name("char-ldb")
            .set_family(InstructionFamily::FieldExtraction)
            .set_format(InstructionFormat::FieldExtraction)
            .set_opcode(0o171),
        Instruction::new()
            .set_name("char-dpb")
            .set_family(InstructionFamily::FieldExtraction)
            .set_format(InstructionFormat::FieldExtraction)
            .set_opcode(0o371),
        Instruction::new()
            .set_name("%p-ldb")
            .set_family(InstructionFamily::FieldExtraction)
            .set_format(InstructionFormat::FieldExtraction)
            .set_opcode(0o172),
        Instruction::new()
            .set_name("%p-dpb")
            .set_family(InstructionFamily::FieldExtraction)
            .set_format(InstructionFormat::FieldExtraction)
            .set_opcode(0o372),
        Instruction::new()
            .set_name("%p-tag-ldb")
            .set_family(InstructionFamily::FieldExtraction)
            .set_format(InstructionFormat::FieldExtraction)
            .set_opcode(0o173),
        Instruction::new()
            .set_name("%p-tag-dpb")
            .set_family(InstructionFamily::FieldExtraction)
            .set_format(InstructionFormat::FieldExtraction)
            .set_opcode(0o373)
    ];
}
