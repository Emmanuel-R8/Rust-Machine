use super::common::{ Instruction, InstructionFamily };

pub fn make_instructions_lexical_variable() -> Vec<Instruction<'static>> {
    return vec![
        Instruction::new().set_family(InstructionFamily::LIST).set_opcode(0o000).set_name("unknown"),
    ];
}
