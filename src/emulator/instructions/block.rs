use super::common::{ Instruction, InstructionFamily };

pub fn make_instructions_block() -> Vec<Instruction<'static>> {
    return vec![
        Instruction::new().set_family(InstructionFamily::LIST).set_opcode(0o120).set_name("%block-1-read"),
        Instruction::new().set_family(InstructionFamily::LIST).set_opcode(0o121).set_name("%block-2-read"),
        Instruction::new().set_family(InstructionFamily::LIST).set_opcode(0o122).set_name("%block-3-read"),
        Instruction::new().set_family(InstructionFamily::LIST).set_opcode(0o123).set_name("%block-4-read"),

        Instruction::new().set_family(InstructionFamily::LIST).set_opcode(0o124).set_name("%block-1-read-shift"),
        Instruction::new().set_family(InstructionFamily::LIST).set_opcode(0o125).set_name("%block-2-read-shift"),
        Instruction::new().set_family(InstructionFamily::LIST).set_opcode(0o126).set_name("%block-3-read-shift"),
        Instruction::new().set_family(InstructionFamily::LIST).set_opcode(0o127).set_name("%block-4-read-shift"),

        Instruction::new().set_family(InstructionFamily::LIST).set_opcode(0o160).set_name("%block-1-read-alu"),
        Instruction::new().set_family(InstructionFamily::LIST).set_opcode(0o161).set_name("%block-2-read-alu"),
        Instruction::new().set_family(InstructionFamily::LIST).set_opcode(0o162).set_name("%block-3-read-alu"),
        Instruction::new().set_family(InstructionFamily::LIST).set_opcode(0o163).set_name("%block-4-read-alu"),

        Instruction::new().set_family(InstructionFamily::LIST).set_opcode(0o130).set_name("%block-1-test"),
        Instruction::new().set_family(InstructionFamily::LIST).set_opcode(0o131).set_name("%block-2-test"),
        Instruction::new().set_family(InstructionFamily::LIST).set_opcode(0o132).set_name("%block-3-test"),
        Instruction::new().set_family(InstructionFamily::LIST).set_opcode(0o133).set_name("%block-4-test"),

        Instruction::new().set_family(InstructionFamily::LIST).set_opcode(0o030).set_name("%block-1-write"),
        Instruction::new().set_family(InstructionFamily::LIST).set_opcode(0o031).set_name("%block-2-write"),
        Instruction::new().set_family(InstructionFamily::LIST).set_opcode(0o032).set_name("%block-3-write"),
        Instruction::new().set_family(InstructionFamily::LIST).set_opcode(0o033).set_name("%block-4-write"),
        ];
}