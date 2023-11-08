use super::common::{ Instruction, InstructionFamily, InstructionFormat };

pub fn make_instructions_function_calling() -> Vec<Instruction<'static>> {
    return vec![
        // DATATYPES
        // Instruction::new().set_family(InstructionFamily::FUNCTION_CALLING).set_opcode(0o000).set_name("dtp-call-compile-even"),
        // Instruction::new().set_family(InstructionFamily::FUNCTION_CALLING).set_opcode(0o000).set_name("dtp-call-compile-even-prefetch"),
        // Instruction::new().set_family(InstructionFamily::FUNCTION_CALLING).set_opcode(0o000).set_name("dtp-call-compile-odd"),
        // Instruction::new().set_family(InstructionFamily::FUNCTION_CALLING).set_opcode(0o000).set_name("dtp-call-compile-odd-prefetch"),
        // Instruction::new().set_family(InstructionFamily::FUNCTION_CALLING).set_opcode(0o000).set_name("dtp-call-indirect"),
        // Instruction::new().set_family(InstructionFamily::FUNCTION_CALLING).set_opcode(0o000).set_name("dtp-call-indirect-prefetch"),
        // Instruction::new().set_family(InstructionFamily::FUNCTION_CALLING).set_opcode(0o000).set_name("dtp-call-generic"),
        // Instruction::new().set_family(InstructionFamily::FUNCTION_CALLING).set_opcode(0o000).set_name("dtp-call-generic-prefetch"),

        Instruction::new()
            .set_family(InstructionFamily::FunctionCalling)
            .set_format(InstructionFormat::OperandFromStack)
            .set_opcode(0o010)
            .set_name("start-call"),
        Instruction::new()
            .set_family(InstructionFamily::FunctionCalling)
            .set_format(InstructionFormat::Immediate10Bits)
            .set_opcode(0o134)
            .set_name("finish-call-n"),
        Instruction::new()
            .set_family(InstructionFamily::FunctionCalling)
            .set_format(InstructionFormat::Immediate10Bits)
            .set_opcode(0o135)
            .set_name("finish-call-n-apply"),
        Instruction::new()
            .set_family(InstructionFamily::FunctionCalling)
            .set_format(InstructionFormat::Immediate10Bits)
            .set_opcode(0o136)
            .set_name("finish-call-tos"),
        Instruction::new()
            .set_family(InstructionFamily::FunctionCalling)
            .set_format(InstructionFormat::Immediate10Bits)
            .set_opcode(0o137)
            .set_name("finish-call-tos-apply"),

        Instruction::new()
            .set_family(InstructionFamily::FunctionCalling)
            .set_format(InstructionFormat::Immediate10Bits)
            .set_opcode(0o176)
            .set_name("entry-rest-accepted"),
        Instruction::new()
            .set_family(InstructionFamily::FunctionCalling)
            .set_format(InstructionFormat::EntryInstruction)
            .set_opcode(0o177)
            .set_name("entry-rest-not-accepted"),

        Instruction::new()
            .set_family(InstructionFamily::FunctionCalling)
            .set_format(InstructionFormat::OperandFromStack)
            .set_opcode(0o050)
            .set_name("locate-locals"),

        Instruction::new()
            .set_family(InstructionFamily::FunctionCalling)
            .set_format(InstructionFormat::Immediate10Bits)
            .set_opcode(0o115)
            .set_name("return-single"),
        Instruction::new()
            .set_family(InstructionFamily::FunctionCalling)
            .set_format(InstructionFormat::OperandFromStack)
            .set_opcode(0o104)
            .set_name("return-multiple"),
        Instruction::new()
            .set_family(InstructionFamily::FunctionCalling)
            .set_format(InstructionFormat::OperandFromStack)
            .set_opcode(0o105)
            .set_name("return-kludge"),

        Instruction::new()
            .set_family(InstructionFamily::FunctionCalling)
            .set_format(InstructionFormat::OperandFromStack)
            .set_opcode(0o106)
            .set_name("take-values")
    ];
}
