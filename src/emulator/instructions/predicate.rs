use super::common::{ Instruction, InstructionFamily };

pub fn make_instructions_predicate() -> Vec<Instruction<'static>> {
    return vec![
        Instruction::new()
            .set_family(InstructionFamily::PREDICATE)
            .set_opcode(0o270)
            .set_name("eq"),
        Instruction::new()
            .set_family(InstructionFamily::PREDICATE)
            .set_opcode(0o274)
            .set_name("eq-no-pop"),
        Instruction::new()
            .set_family(InstructionFamily::PREDICATE)
            .set_opcode(0o263)
            .set_name("eql"),
        Instruction::new()
            .set_family(InstructionFamily::PREDICATE)
            .set_opcode(0o267)
            .set_name("eql-no-pop"),
        Instruction::new()
            .set_family(InstructionFamily::PREDICATE)
            .set_opcode(0o260)
            .set_name("equal-number"),
        Instruction::new()
            .set_family(InstructionFamily::PREDICATE)
            .set_opcode(0o264)
            .set_name("equal-number-no-pop"),
        Instruction::new()
            .set_family(InstructionFamily::PREDICATE)
            .set_opcode(0o262)
            .set_name("greaterp"),
        Instruction::new()
            .set_family(InstructionFamily::PREDICATE)
            .set_opcode(0o266)
            .set_name("greaterp-no-pop"),
        Instruction::new()
            .set_family(InstructionFamily::PREDICATE)
            .set_opcode(0o261)
            .set_name("lesserp"),
        Instruction::new()
            .set_family(InstructionFamily::PREDICATE)
            .set_opcode(0o265)
            .set_name("lesserp-no-pop"),
        Instruction::new()
            .set_family(InstructionFamily::PREDICATE)
            .set_opcode(0o273)
            .set_name("logtest"),
        Instruction::new()
            .set_family(InstructionFamily::PREDICATE)
            .set_opcode(0o277)
            .set_name("logtest-no-pop"),
        Instruction::new()
            .set_family(InstructionFamily::PREDICATE)
            .set_opcode(0o040)
            .set_name("type-member-1"),
        Instruction::new()
            .set_family(InstructionFamily::PREDICATE)
            .set_opcode(0o041)
            .set_name("type-member-2"),
        Instruction::new()
            .set_family(InstructionFamily::PREDICATE)
            .set_opcode(0o042)
            .set_name("type-member-3"),
        Instruction::new()
            .set_family(InstructionFamily::PREDICATE)
            .set_opcode(0o043)
            .set_name("type-member-4"),
        Instruction::new()
            .set_family(InstructionFamily::PREDICATE)
            .set_opcode(0o044)
            .set_name("type-member-1-no-pop"),
        Instruction::new()
            .set_family(InstructionFamily::PREDICATE)
            .set_opcode(0o045)
            .set_name("type-member-2-no-pop"),
        Instruction::new()
            .set_family(InstructionFamily::PREDICATE)
            .set_opcode(0o046)
            .set_name("type-member-3-no-pop"),
        Instruction::new()
            .set_family(InstructionFamily::PREDICATE)
            .set_opcode(0o047)
            .set_name("type-member-4-no-pop"),
        Instruction::new()
            .set_family(InstructionFamily::PREDICATE)
            .set_opcode(0o002)
            .set_name("endp"),
        Instruction::new()
            .set_family(InstructionFamily::PREDICATE)
            .set_opcode(0o036)
            .set_name("plusp"),
        Instruction::new()
            .set_family(InstructionFamily::PREDICATE)
            .set_opcode(0o035)
            .set_name("minusp"),
        Instruction::new()
            .set_family(InstructionFamily::PREDICATE)
            .set_opcode(0o034)
            .set_name("zerop")
    ];
}
