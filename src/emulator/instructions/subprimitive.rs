use super::common::{ Instruction, InstructionFamily };

pub fn make_instructions_subprimitive() -> Vec<Instruction<'static>> {
    return vec![
        Instruction::new()
            .set_family(InstructionFamily::LIST)
            .set_opcode(0o007)
            .set_name("%ephemeralp"),

        Instruction::new()
            .set_family(InstructionFamily::LIST)
            .set_opcode(0o331)
            .set_name("%unsigned-lessp"),
        Instruction::new()
            .set_family(InstructionFamily::LIST)
            .set_opcode(0o335)
            .set_name("%unsigned-lessp-no-pop"),

        Instruction::new()
            .set_family(InstructionFamily::LIST)
            .set_opcode(0o311)
            .set_name("%allocate-list-block"),
        Instruction::new()
            .set_family(InstructionFamily::LIST)
            .set_opcode(0o315)
            .set_name("%allocate-structure-block"),

        Instruction::new()
            .set_family(InstructionFamily::LIST)
            .set_opcode(0o230)
            .set_name("%pointer-plus"),
        Instruction::new()
            .set_family(InstructionFamily::LIST)
            .set_opcode(0o231)
            .set_name("%pointer-difference"),
        Instruction::new()
            .set_family(InstructionFamily::LIST)
            .set_opcode(0o145)
            .set_name("%pointer-increment"),

        Instruction::new()
            .set_family(InstructionFamily::LIST)
            .set_opcode(0o154)
            .set_name("%read-internal-register"),
        Instruction::new()
            .set_family(InstructionFamily::LIST)
            .set_opcode(0o155)
            .set_name("%write-internal-register"),

        Instruction::new().set_family(InstructionFamily::LIST).set_opcode(0o056).set_name("no-op"),

        Instruction::new()
            .set_family(InstructionFamily::LIST)
            .set_opcode(0o156)
            .set_name("%coprocessor-read"),
        Instruction::new()
            .set_family(InstructionFamily::LIST)
            .set_opcode(0o157)
            .set_name("%coprocessor-write"),

        Instruction::new()
            .set_family(InstructionFamily::LIST)
            .set_opcode(0o116)
            .set_name("%memory-read"),
        Instruction::new()
            .set_family(InstructionFamily::LIST)
            .set_opcode(0o117)
            .set_name("%memory-read-address"),

        Instruction::new().set_family(InstructionFamily::LIST).set_opcode(0o012).set_name("%tag"),
        Instruction::new()
            .set_family(InstructionFamily::LIST)
            .set_opcode(0o327)
            .set_name("%set-tag"),

        Instruction::new()
            .set_family(InstructionFamily::LIST)
            .set_opcode(0o233)
            .set_name("store-conditional"),
        Instruction::new()
            .set_family(InstructionFamily::LIST)
            .set_opcode(0o235)
            .set_name("%p-store-contents"),

        Instruction::new()
            .set_family(InstructionFamily::LIST)
            .set_opcode(0o234)
            .set_name("%memory-write"),

        Instruction::new()
            .set_family(InstructionFamily::LIST)
            .set_opcode(0o146)
            .set_name("%set-cdr-code-1"),
        Instruction::new()
            .set_family(InstructionFamily::LIST)
            .set_opcode(0o147)
            .set_name("%set-cdr-code-2"),

        Instruction::new()
            .set_family(InstructionFamily::LIST)
            .set_opcode(0o342)
            .set_name("%merge-cdr-no-pop"),

        Instruction::new()
            .set_family(InstructionFamily::LIST)
            .set_opcode(0o052)
            .set_name("%generic-dispatch"),
        Instruction::new()
            .set_family(InstructionFamily::LIST)
            .set_opcode(0o053)
            .set_name("%message-dispatch"),

        Instruction::new().set_family(InstructionFamily::LIST).set_opcode(0o011).set_name("%jump"),

        Instruction::new()
            .set_family(InstructionFamily::LIST)
            .set_opcode(0o054)
            .set_name("%check-preempt-request"),
        Instruction::new().set_family(InstructionFamily::LIST).set_opcode(0o377).set_name("%halt")
    ];
}
