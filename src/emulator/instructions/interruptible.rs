use super::common::{ Instruction, InstructionFamily, InstructionFormat };

pub fn make_instructions_interruptible() -> Vec<Instruction<'static>> {
    return vec![
        Instruction::new()
            .set_family(InstructionFamily::Interruptible)
            .set_format(InstructionFormat::OperandFromStack)
            .set_opcode(0o225)
            .set_name("rgetf"),
        Instruction::new()
            .set_family(InstructionFamily::Interruptible)
            .set_format(InstructionFormat::OperandFromStack)
            .set_opcode(0o226)
            .set_name("member"),
        Instruction::new()
            .set_family(InstructionFamily::Interruptible)
            .set_format(InstructionFormat::OperandFromStack)
            .set_opcode(0o227)
            .set_name("assoc")
    ];
}
