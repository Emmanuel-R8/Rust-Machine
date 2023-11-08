use super::common::{ Instruction, InstructionFamily, InstructionFormat };

pub fn make_instructions_catch() -> Vec<Instruction<'static>> {
    return vec![
        Instruction::new()
            .set_family(InstructionFamily::Catch)
            .set_format(InstructionFormat::Immediate10Bits)
            .set_opcode(0o376)
            .set_name("catch-open"),
        Instruction::new()
            .set_family(InstructionFamily::Catch)
            .set_format(InstructionFormat::OperandFromStack)
            .set_opcode(0o051)
            .set_name("catch-close")
    ];
}
