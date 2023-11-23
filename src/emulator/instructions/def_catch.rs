use super::common::{ Instruction, InstructionFamily, InstructionFormat };

pub fn make_instructions_catch() -> Vec<Instruction<'static>> {
    return vec![
        Instruction::new()
            .set_name("catch-open")
            .set_family(InstructionFamily::Catch)
            .set_format(InstructionFormat::Immediate10Bits)
            .set_opcode(0o376),
        Instruction::new()
            .set_name("catch-close")
            .set_family(InstructionFamily::Catch)
            .set_format(InstructionFormat::OperandFromStack)
            .set_opcode(0o051)
    ];
}
