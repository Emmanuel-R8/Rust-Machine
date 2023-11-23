use super::common::{ Instruction, InstructionFamily, InstructionFormat };

pub fn make_instructions_function_calling() -> Vec<Instruction<'static>> {
    return vec![
        // DATATYPES
        // Instruction::new().set_family(InstructionFamily::FUNCTION_CALLING).set_opcode(0o000).set_name("dtp-call-compile-even")
        // Instruction::new().set_family(InstructionFamily::FUNCTION_CALLING).set_opcode(0o000).set_name("dtp-call-compile-even-prefetch")
        // Instruction::new().set_family(InstructionFamily::FUNCTION_CALLING).set_opcode(0o000).set_name("dtp-call-compile-odd")
        // Instruction::new().set_family(InstructionFamily::FUNCTION_CALLING).set_opcode(0o000).set_name("dtp-call-compile-odd-prefetch")
        // Instruction::new().set_family(InstructionFamily::FUNCTION_CALLING).set_opcode(0o000).set_name("dtp-call-indirect")
        // Instruction::new().set_family(InstructionFamily::FUNCTION_CALLING).set_opcode(0o000).set_name("dtp-call-indirect-prefetch")
        // Instruction::new().set_family(InstructionFamily::FUNCTION_CALLING).set_opcode(0o000).set_name("dtp-call-generic")
        // Instruction::new().set_family(InstructionFamily::FUNCTION_CALLING).set_opcode(0o000).set_name("dtp-call-generic-prefetch")

        Instruction::new()
            .set_name("start-call")
            .set_family(InstructionFamily::FunctionCalling)
            .set_format(InstructionFormat::OperandFromStack)
            .set_opcode(0o010),
        Instruction::new()
            .set_name("finish-call-n")
            .set_family(InstructionFamily::FunctionCalling)
            .set_format(InstructionFormat::Immediate10Bits)
            .set_opcode(0o134),
        Instruction::new()
            .set_name("finish-call-n-apply")
            .set_family(InstructionFamily::FunctionCalling)
            .set_format(InstructionFormat::Immediate10Bits)
            .set_opcode(0o135),
        Instruction::new()
            .set_name("finish-call-tos")
            .set_family(InstructionFamily::FunctionCalling)
            .set_format(InstructionFormat::Immediate10Bits)
            .set_opcode(0o136),
        Instruction::new()
            .set_name("finish-call-tos-apply")
            .set_family(InstructionFamily::FunctionCalling)
            .set_format(InstructionFormat::Immediate10Bits)
            .set_opcode(0o137),
        Instruction::new()
            .set_name("entry-rest-accepted")
            .set_family(InstructionFamily::FunctionCalling)
            .set_format(InstructionFormat::Immediate10Bits)
            .set_opcode(0o176),
        Instruction::new()
            .set_name("entry-rest-not-accepted")
            .set_family(InstructionFamily::FunctionCalling)
            .set_format(InstructionFormat::EntryInstruction)
            .set_opcode(0o177),
        Instruction::new()
            .set_name("locate-locals")
            .set_family(InstructionFamily::FunctionCalling)
            .set_format(InstructionFormat::OperandFromStack)
            .set_opcode(0o050),
        Instruction::new()
            .set_name("return-single")
            .set_family(InstructionFamily::FunctionCalling)
            .set_format(InstructionFormat::Immediate10Bits)
            .set_opcode(0o115),
        Instruction::new()
            .set_name("return-multiple")
            .set_family(InstructionFamily::FunctionCalling)
            .set_format(InstructionFormat::OperandFromStack)
            .set_opcode(0o104),
        Instruction::new()
            .set_name("return-kludge")
            .set_family(InstructionFamily::FunctionCalling)
            .set_format(InstructionFormat::OperandFromStack)
            .set_opcode(0o105),
        Instruction::new()
            .set_name("take-values")
            .set_family(InstructionFamily::FunctionCalling)
            .set_format(InstructionFormat::OperandFromStack)
            .set_opcode(0o106)
    ]; // Instruction::new().set_family(InstructionFamily::FUNCTION_CALLING).set_opcode(0o000).set_name("dtp-call-compile-even-prefetch") // Instruction::new().set_family(InstructionFamily::FUNCTION_CALLING).set_opcode(0o000).set_name("dtp-call-compile-odd")
    // Instruction::new().set_family(InstructionFamily::FUNCTION_CALLING).set_opcode(0o000).set_name("dtp-call-compile-odd-prefetch")
    // Instruction::new().set_family(InstructionFamily::FUNCTION_CALLING).set_opcode(0o000).set_name("dtp-call-indirect")
    // Instruction::new().set_family(InstructionFamily::FUNCTION_CALLING).set_opcode(0o000).set_name("dtp-call-indirect-prefetch")
    // Instruction::new().set_family(InstructionFamily::FUNCTION_CALLING).set_opcode(0o000).set_name("dtp-call-generic")
    // Instruction::new().set_family(InstructionFamily::FUNCTION_CALLING).set_opcode(0o000).set_name("dtp-call-generic-prefetch")
}
