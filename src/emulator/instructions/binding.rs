use super::common::{ Instruction, InstructionFamily, InstructionFormat };

pub fn make_instructions_binding() -> Vec<Instruction<'static>> {
    return vec![
        Instruction::new()
            .set_family(InstructionFamily::Binding)
            .set_format(InstructionFormat::OperandFromStack)
            .set_opcode(0o236)
            .set_name("bind-locative-to-value"),
        Instruction::new()
            .set_family(InstructionFamily::Binding)
            .set_format(InstructionFormat::OperandFromStack)
            .set_opcode(0o005)
            .set_name("bind-locative"),
        Instruction::new()
            .set_family(InstructionFamily::Binding)
            .set_format(InstructionFormat::OperandFromStack)
            .set_opcode(0o107)
            .set_name("unbind-n"),
        Instruction::new()
            .set_family(InstructionFamily::Binding)
            .set_format(InstructionFormat::OperandFromStack)
            .set_opcode(0o006)
            .set_name("%restore-binding-stack")
    ];
}
