use super::common::{ Instruction, InstructionFamily };

pub fn make_instructions_interruptible() -> Vec<Instruction<'static>> {
    return vec![
        Instruction::new()
            .set_family(InstructionFamily::INTERRUPTIBLE)
            .set_opcode(0o225)
            .set_name("rgetf"),
        Instruction::new()
            .set_family(InstructionFamily::INTERRUPTIBLE)
            .set_opcode(0o226)
            .set_name("member"),
        Instruction::new()
            .set_family(InstructionFamily::INTERRUPTIBLE)
            .set_opcode(0o227)
            .set_name("assoc")
    ];
}
