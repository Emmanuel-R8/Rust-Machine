use crate::hardware::machine::VirtualMachine;

use super::common::{ Instruction, InstructionFamily, InstructionFormat };

pub fn make_instructions_numeric() -> Vec<Instruction<'static>> {
    return vec![
        Instruction::new()
            .set_name("add")
            .set_family(InstructionFamily::Numeric)
            .set_format(InstructionFormat::OperandFromStack)
            .set_opcode(0o300)
            .set_arg_count(2)
            .set_ret_count(1),
        Instruction::new()
            .set_name("sub")
            .set_family(InstructionFamily::Numeric)
            .set_format(InstructionFormat::OperandFromStack)
            .set_opcode(0o301), //
        Instruction::new()
            .set_name("unary-minus")
            .set_family(InstructionFamily::Numeric)
            .set_format(InstructionFormat::OperandFromStack)
            .set_opcode(0o114), //
        Instruction::new()
            .set_name("increment")
            .set_family(InstructionFamily::Numeric)
            .set_format(InstructionFormat::OperandFromStack)
            .set_opcode(0o143),
        Instruction::new()
            .set_name("decrement")
            .set_family(InstructionFamily::Numeric)
            .set_format(InstructionFormat::OperandFromStack)
            .set_opcode(0o144), //
        Instruction::new()
            .set_name("multiply")
            .set_family(InstructionFamily::Numeric)
            .set_format(InstructionFormat::OperandFromStack)
            .set_opcode(0o202),
        Instruction::new()
            .set_name("quotient")
            .set_family(InstructionFamily::Numeric)
            .set_format(InstructionFormat::OperandFromStack)
            .set_opcode(0o203),
        Instruction::new()
            .set_name("ceiling")
            .set_family(InstructionFamily::Numeric)
            .set_format(InstructionFormat::OperandFromStack)
            .set_opcode(0o204),
        Instruction::new()
            .set_name("floor")
            .set_family(InstructionFamily::Numeric)
            .set_format(InstructionFormat::OperandFromStack)
            .set_opcode(0o205),
        Instruction::new()
            .set_name("truncate")
            .set_family(InstructionFamily::Numeric)
            .set_format(InstructionFormat::OperandFromStack)
            .set_opcode(0o206),
        Instruction::new()
            .set_name("round")
            .set_family(InstructionFamily::Numeric)
            .set_format(InstructionFormat::OperandFromStack)
            .set_opcode(0o207),
        Instruction::new()
            .set_name("remainder")
            .set_family(InstructionFamily::Numeric)
            .set_format(InstructionFormat::OperandFromStack)
            .set_opcode(0o210),
        Instruction::new()
            .set_name("rational-quotient")
            .set_family(InstructionFamily::Numeric)
            .set_format(InstructionFormat::OperandFromStack)
            .set_opcode(0o211),
        Instruction::new()
            .set_name("min")
            .set_family(InstructionFamily::Numeric)
            .set_format(InstructionFormat::OperandFromStack)
            .set_opcode(0o212),
        Instruction::new()
            .set_name("max")
            .set_family(InstructionFamily::Numeric)
            .set_format(InstructionFormat::OperandFromStack)
            .set_opcode(0o213), //
        Instruction::new()
            .set_name("logand")
            .set_family(InstructionFamily::Numeric)
            .set_format(InstructionFormat::OperandFromStack)
            .set_opcode(0o215),
        Instruction::new()
            .set_name("logxor")
            .set_family(InstructionFamily::Numeric)
            .set_format(InstructionFormat::OperandFromStack)
            .set_opcode(0o216),
        Instruction::new()
            .set_name("logior")
            .set_family(InstructionFamily::Numeric)
            .set_format(InstructionFormat::OperandFromStack)
            .set_opcode(0o217), //
        Instruction::new()
            .set_name("rot")
            .set_family(InstructionFamily::Numeric)
            .set_format(InstructionFormat::OperandFromStack)
            .set_opcode(0o220),
        Instruction::new()
            .set_name("lsh")
            .set_family(InstructionFamily::Numeric)
            .set_format(InstructionFormat::OperandFromStack)
            .set_opcode(0o221),
        Instruction::new()
            .set_name("multiply-divide")
            .set_family(InstructionFamily::Numeric)
            .set_format(InstructionFormat::OperandFromStack)
            .set_opcode(0o222), //
        Instruction::new()
            .set_name("ash")
            .set_family(InstructionFamily::Numeric)
            .set_format(InstructionFormat::OperandFromStack)
            .set_opcode(0o232), //
        Instruction::new()
            .set_name("%32-bit-plus")
            .set_family(InstructionFamily::Numeric)
            .set_format(InstructionFormat::OperandFromStack)
            .set_opcode(0o302),
        Instruction::new()
            .set_name("%32-bit-difference")
            .set_family(InstructionFamily::Numeric)
            .set_format(InstructionFormat::OperandFromStack)
            .set_opcode(0o303),
        Instruction::new()
            .set_name("%add-bignum-step")
            .set_family(InstructionFamily::Numeric)
            .set_format(InstructionFormat::OperandFromStack)
            .set_opcode(0o304),
        Instruction::new()
            .set_name("%sub-bignum-step")
            .set_family(InstructionFamily::Numeric)
            .set_format(InstructionFormat::OperandFromStack)
            .set_opcode(0o305),
        Instruction::new()
            .set_name("%multiply-bignum-step")
            .set_family(InstructionFamily::Numeric)
            .set_format(InstructionFormat::OperandFromStack)
            .set_opcode(0o306),
        Instruction::new()
            .set_name("%divide-bignum-step")
            .set_family(InstructionFamily::Numeric)
            .set_format(InstructionFormat::OperandFromStack)
            .set_opcode(0o307)
    ];
}

// Instruction add
impl VirtualMachine {
    pub fn add(&mut self) -> &Self {
        let operand = self.pop_stack().unwrap();
        let operand2 = self.pop_stack().unwrap();
        let result = operand + operand2;

        self.push_stack(result.unwrap());
        return self;
    }
}
