use super::common::{ Instruction, InstructionFamily };

pub fn make_instructions_branch_loop() -> Vec<Instruction<'static>> {
    return vec![
        Instruction::new().set_family(InstructionFamily::LIST).set_opcode(0o174).set_name("branch"),

        Instruction::new()
            .set_family(InstructionFamily::LIST)
            .set_opcode(0o060)
            .set_name("branch-true"),
        Instruction::new()
            .set_family(InstructionFamily::LIST)
            .set_opcode(0o070)
            .set_name("branch-false"),

        Instruction::new()
            .set_family(InstructionFamily::LIST)
            .set_opcode(0o064)
            .set_name("branch-true-no-pop"),
        Instruction::new()
            .set_family(InstructionFamily::LIST)
            .set_opcode(0o074)
            .set_name("branch-false-no-pop"),

        Instruction::new()
            .set_family(InstructionFamily::LIST)
            .set_opcode(0o066)
            .set_name("branch-true-else-no-pop"),
        Instruction::new()
            .set_family(InstructionFamily::LIST)
            .set_opcode(0o076)
            .set_name("branch-false-else-no-pop"),

        Instruction::new()
            .set_family(InstructionFamily::LIST)
            .set_opcode(0o065)
            .set_name("branch-true-and-no-pop"),
        Instruction::new()
            .set_family(InstructionFamily::LIST)
            .set_opcode(0o075)
            .set_name("branch-false-and-no-pop"),

        Instruction::new()
            .set_family(InstructionFamily::LIST)
            .set_opcode(0o062)
            .set_name("branch-true-and-extra-pop"),
        Instruction::new()
            .set_family(InstructionFamily::LIST)
            .set_opcode(0o072)
            .set_name("branch-false-and-extra-pop"),

        Instruction::new()
            .set_family(InstructionFamily::LIST)
            .set_opcode(0o061)
            .set_name("branch-true-else-extra-pop"),
        Instruction::new()
            .set_family(InstructionFamily::LIST)
            .set_opcode(0o071)
            .set_name("branch-false-else-extra-pop"),

        Instruction::new()
            .set_family(InstructionFamily::LIST)
            .set_opcode(0o063)
            .set_name("branch-true-extra-pop"),
        Instruction::new()
            .set_family(InstructionFamily::LIST)
            .set_opcode(0o073)
            .set_name("branch-false-extra-pop"),

        Instruction::new()
            .set_family(InstructionFamily::LIST)
            .set_opcode(0o067)
            .set_name("branch-true-and-no-pop-else-no-pop-extra-pop"),
        Instruction::new()
            .set_family(InstructionFamily::LIST)
            .set_opcode(0o077)
            .set_name("branch-false-and-no-pop-else-no-pop-extra-pop"),

        Instruction::new()
            .set_family(InstructionFamily::LIST)
            .set_opcode(0o175)
            .set_name("loop-decrement-tos"),
        Instruction::new()
            .set_family(InstructionFamily::LIST)
            .set_opcode(0o375)
            .set_name("loop-decrement-tos-less-than")
    ];
}
