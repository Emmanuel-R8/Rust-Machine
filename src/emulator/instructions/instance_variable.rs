use super::common::{ Instruction, InstructionFamily };

pub fn make_instructions_instance_variable() -> Vec<Instruction<'static>> {
    return vec![
        Instruction::new()
            .set_family(InstructionFamily::InstanceVariableAccess)
            .set_opcode(0o110)
            .set_name("push-instance-variable"),
        Instruction::new()
            .set_family(InstructionFamily::InstanceVariableAccess)
            .set_opcode(0o320)
            .set_name("pop-instance-variable"),
        Instruction::new()
            .set_family(InstructionFamily::InstanceVariableAccess)
            .set_opcode(0o321)
            .set_name("movem-instance-variable"),

        Instruction::new()
            .set_family(InstructionFamily::InstanceVariableAccess)
            .set_opcode(0o111)
            .set_name("push-address-instance-variable"),

        // FIXME HOW CAN THE OPCODE BE THE SAME IN THE DOCS AT 0o322?
        // MODIFIED TO 0o112 provisionally
        Instruction::new()
            .set_family(InstructionFamily::InstanceVariableAccess)
            .set_opcode(0o112)
            .set_name("push-instance-variable-ordered"),

        // FIXME HOW CAN THE OPCODE BE THE SAME IN THE DOCS AT 0o322?
        Instruction::new()
            .set_family(InstructionFamily::InstanceVariableAccess)
            .set_opcode(0o322)
            .set_name("pop-instance-variable-ordered"),
        Instruction::new()
            .set_family(InstructionFamily::InstanceVariableAccess)
            .set_opcode(0o323)
            .set_name("movem-instance-variable-ordered"),

        Instruction::new()
            .set_family(InstructionFamily::InstanceVariableAccess)
            .set_opcode(0o113)
            .set_name("push-address-instance-variable-ordered"),

        Instruction::new()
            .set_family(InstructionFamily::InstanceVariableAccess)
            .set_opcode(0o324)
            .set_name("%instance-ref"),
        Instruction::new()
            .set_family(InstructionFamily::InstanceVariableAccess)
            .set_opcode(0o325)
            .set_name("%instance-set"),
        Instruction::new()
            .set_family(InstructionFamily::InstanceVariableAccess)
            .set_opcode(0o326)
            .set_name("%instance-loc")
    ];
}
