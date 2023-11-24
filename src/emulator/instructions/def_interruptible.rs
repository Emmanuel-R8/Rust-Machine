use super::common::{ Instruction, InstructionFamily, InstructionFormat };

pub fn make_instructions_interruptible() -> Vec<Instruction<'static>> {
    return vec![
        Instruction::new()
            .set_name("rgetf")
            .set_family(InstructionFamily::Interruptible)
            .set_format(InstructionFormat::OperandFromStack)
            .set_opcode(0o225),
        Instruction::new()
            .set_name("member")
            .set_family(InstructionFamily::Interruptible)
            .set_format(InstructionFormat::OperandFromStack)
            .set_opcode(0o226),
        Instruction::new()
            .set_name("assoc")
            .set_family(InstructionFamily::Interruptible)
            .set_format(InstructionFormat::OperandFromStack)
            .set_opcode(0o227)
    ];
}
