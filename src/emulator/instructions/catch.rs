use super::common::{ Instruction, InstructionFamily };

pub fn make_instructions_catch() -> Vec<Instruction<'static>> {
    return vec![
        Instruction::new()
            .set_family(InstructionFamily::Catch)
            .set_opcode(0o376)
            .set_name("catch-open"),
        Instruction::new()
            .set_family(InstructionFamily::Catch)
            .set_opcode(0o051)
            .set_name("catch-close")
    ];
}
