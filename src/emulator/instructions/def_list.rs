use super::common::{ Instruction, InstructionFamily, InstructionFormat };

pub fn make_instructions_list() -> Vec<Instruction<'static>> {
    return vec![
        Instruction::new()
            .set_name("car")
            .set_family(InstructionFamily::List)
            .set_format(InstructionFormat::OperandFromStack)
            .set_opcode(0o000)
            .set_arg_count(1)
            .set_ret_count(1),
        Instruction::new()
            .set_name("cdr")
            .set_family(InstructionFamily::List)
            .set_format(InstructionFormat::OperandFromStack)
            .set_opcode(0o001)
            .set_arg_count(1)
            .set_ret_count(1),
        Instruction::new()
            .set_name("set-to-car")
            .set_family(InstructionFamily::List)
            .set_format(InstructionFormat::OperandFromStack)
            .set_opcode(0o140)
            .set_arg_count(1)
            .set_ret_count(0),
        Instruction::new()
            .set_name("set-to-cdr")
            .set_family(InstructionFamily::List)
            .set_format(InstructionFormat::OperandFromStack)
            .set_opcode(0o141)
            .set_arg_count(1)
            .set_ret_count(0),
        Instruction::new()
            .set_name("set-to-cdr-push-car")
            .set_family(InstructionFamily::List)
            .set_format(InstructionFormat::OperandFromStack)
            .set_opcode(0o142)
            .set_arg_count(1)
            .set_ret_count(1),

        Instruction::new()
            .set_name("rplaca")
            .set_family(InstructionFamily::List)
            .set_format(InstructionFormat::OperandFromStack)
            .set_opcode(0o200)
            .set_arg_count(2)
            .set_ret_count(0),
        Instruction::new()
            .set_name("rplacd")
            .set_family(InstructionFamily::List)
            .set_format(InstructionFormat::OperandFromStack)
            .set_opcode(0o201)
            .set_arg_count(2)
            .set_ret_count(0)
    ];
}
