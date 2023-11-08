use super::common::{ Instruction, InstructionFamily, InstructionFormat };

pub fn make_instructions_array() -> Vec<Instruction<'static>> {
    return vec![
        Instruction::new()
            .set_family(InstructionFamily::Array)
            .set_format(InstructionFormat::OperandFromStack)
            .set_opcode(0o312)
            .set_name("aref-1"),
        Instruction::new()
            .set_family(InstructionFamily::Array)
            .set_format(InstructionFormat::OperandFromStack)
            .set_opcode(0o310)
            .set_name("aset-1"),
        Instruction::new()
            .set_family(InstructionFamily::Array)
            .set_format(InstructionFormat::OperandFromStack)
            .set_opcode(0o313)
            .set_name("aloc-1"),
        Instruction::new()
            .set_family(InstructionFamily::Array)
            .set_format(InstructionFormat::OperandFromStack)
            .set_opcode(0o003)
            .set_name("setup-1d-array"),
        Instruction::new()
            .set_family(InstructionFamily::Array)
            .set_format(InstructionFormat::OperandFromStack)
            .set_opcode(0o004)
            .set_name("setup-force-1d-array"),

        Instruction::new()
            .set_family(InstructionFamily::Array)
            .set_format(InstructionFormat::OperandFromStack)
            .set_opcode(0o350)
            .set_name("fast-aref-1"),
        Instruction::new()
            .set_family(InstructionFamily::Array)
            .set_format(InstructionFormat::OperandFromStack)
            .set_opcode(0o351)
            .set_name("fast-aset-1"),

        Instruction::new()
            .set_family(InstructionFamily::Array)
            .set_format(InstructionFormat::OperandFromStack)
            .set_opcode(0o316)
            .set_name("array-leader"),
        Instruction::new()
            .set_family(InstructionFamily::Array)
            .set_format(InstructionFormat::OperandFromStack)
            .set_opcode(0o314)
            .set_name("store-array-leader"),
        Instruction::new()
            .set_family(InstructionFamily::Array)
            .set_format(InstructionFormat::OperandFromStack)
            .set_opcode(0o317)
            .set_name("aloc-leader")
    ];
}
