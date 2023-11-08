use super::common::{ Instruction, InstructionFamily, InstructionFormat };

pub fn make_instructions_list() -> Vec<Instruction<'static>> {
    return vec![
        Instruction::new()
            .set_family(InstructionFamily::List)
            .set_format(InstructionFormat::OperandFromStack)
            .set_opcode(0o000)
            .set_name("car"),
        Instruction::new()
            .set_family(InstructionFamily::List)
            .set_format(InstructionFormat::OperandFromStack)
            .set_opcode(0o001)
            .set_name("cdr"),
        Instruction::new()
            .set_family(InstructionFamily::List)
            .set_format(InstructionFormat::OperandFromStack)
            .set_opcode(0o140)
            .set_name("set-to-car"),
        Instruction::new()
            .set_family(InstructionFamily::List)
            .set_format(InstructionFormat::OperandFromStack)
            .set_opcode(0o141)
            .set_name("set-to-cdr"),
        Instruction::new()
            .set_family(InstructionFamily::List)
            .set_format(InstructionFormat::OperandFromStack)
            .set_opcode(0o142)
            .set_name("set-to-cdr-push-car"),
        Instruction::new()
            .set_family(InstructionFamily::List)
            .set_format(InstructionFormat::OperandFromStack)
            .set_opcode(0o200)
            .set_name("rplaca"),
        Instruction::new()
            .set_family(InstructionFamily::List)
            .set_format(InstructionFormat::OperandFromStack)
            .set_opcode(0o201)
            .set_name("rplacd")
    ];
}
