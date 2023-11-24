use super::common::{ Instruction, InstructionFamily, InstructionFormat };

pub fn make_instructions_instance_variable() -> Vec<Instruction> {
    return vec![
        Instruction::new()
            .set_name("push-instance-variable".to_string())
            .set_family(InstructionFamily::InstanceVariableAccess)
            .set_format(InstructionFormat::OperandFromStackImmediate)
            .set_opcode(0o110),
        Instruction::new()
            .set_name("pop-instance-variable".to_string())
            .set_family(InstructionFamily::InstanceVariableAccess)
            .set_format(InstructionFormat::OperandFromStackImmediate)
            .set_opcode(0o320),
        Instruction::new()
            .set_name("movem-instance-variable".to_string())
            .set_family(InstructionFamily::InstanceVariableAccess)
            .set_format(InstructionFormat::OperandFromStackImmediate)
            .set_opcode(0o321),
        Instruction::new()
            .set_name("push-address-instance-variable".to_string())
            .set_family(InstructionFamily::InstanceVariableAccess)
            .set_format(InstructionFormat::OperandFromStackImmediate)
            .set_opcode(0o111),
        // FIXME HOW CAN THE OPCODE BE THE SAME IN THE DOCS AT 0o322?
        // MODIFIED TO 0o112 provisionally
        Instruction::new()
            .set_name("push-instance-variable-ordered".to_string())
            .set_family(InstructionFamily::InstanceVariableAccess)
            .set_format(InstructionFormat::OperandFromStackImmediate)
            .set_opcode(0o112),
        // FIXME HOW CAN THE OPCODE BE THE SAME IN THE DOCS AT 0o322?
        Instruction::new()
            .set_name("pop-instance-variable-ordered".to_string())
            .set_family(InstructionFamily::InstanceVariableAccess)
            .set_format(InstructionFormat::OperandFromStackImmediate)
            .set_opcode(0o322),
        Instruction::new()
            .set_name("movem-instance-variable-ordered".to_string())
            .set_family(InstructionFamily::InstanceVariableAccess)
            .set_format(InstructionFormat::OperandFromStackImmediate)
            .set_opcode(0o323),
        Instruction::new()
            .set_name("push-address-instance-variable-ordered".to_string())
            .set_family(InstructionFamily::InstanceVariableAccess)
            .set_format(InstructionFormat::OperandFromStackImmediate)
            .set_opcode(0o113),
        Instruction::new()
            .set_name("%instance-ref".to_string())
            .set_family(InstructionFamily::InstanceVariableAccess)
            .set_format(InstructionFormat::OperandFromStack)
            .set_opcode(0o324),
        Instruction::new()
            .set_name("%instance-set".to_string())
            .set_family(InstructionFamily::InstanceVariableAccess)
            .set_format(InstructionFormat::OperandFromStack)
            .set_opcode(0o325),
        Instruction::new()
            .set_name("%instance-loc".to_string())
            .set_family(InstructionFamily::InstanceVariableAccess)
            .set_format(InstructionFormat::OperandFromStack)
            .set_opcode(0o326)
    ];
}
