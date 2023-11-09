use super::common::{ Instruction, InstructionFamily, InstructionFormat };

pub fn make_instructions_binding() -> Vec<Instruction<'static>> {
    return vec![
        Instruction::new()
            .set_name("bind-locative-to-value")
            .set_family(InstructionFamily::Binding)
            .set_format(InstructionFormat::OperandFromStack)
            .set_opcode(0o236),
        Instruction::new()
            .set_name("bind-locative")
            .set_family(InstructionFamily::Binding)
            .set_format(InstructionFormat::OperandFromStack)
            .set_opcode(0o005),
        Instruction::new()
            .set_name("unbind-n")
            .set_family(InstructionFamily::Binding)
            .set_format(InstructionFormat::OperandFromStack)
            .set_opcode(0o107),
        Instruction::new()
            .set_name("%restore-binding-stack")
            .set_family(InstructionFamily::Binding)
            .set_format(InstructionFormat::OperandFromStack)
            .set_opcode(0o006)
    ];
}
