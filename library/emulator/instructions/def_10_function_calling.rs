use crate::common::constants::OpCode;
use crate::hardware::machine::VirtualMachine;

use super::common::{Instruction, InstructionFamily, InstructionFormat};

pub fn make_instructions_function_calling() -> Vec<Instruction> {
    return vec![
        // DATATYPES
        // Instruction::new().set_family(InstructionFamily::FUNCTION_CALLING).set_opcode(0o000).set_name("dtp-call-compile-even".to_string())
        // Instruction::new().set_family(InstructionFamily::FUNCTION_CALLING).set_opcode(0o000).set_name("dtp-call-compile-even-prefetch".to_string())
        // Instruction::new().set_family(InstructionFamily::FUNCTION_CALLING).set_opcode(0o000).set_name("dtp-call-compile-odd".to_string())
        // Instruction::new().set_family(InstructionFamily::FUNCTION_CALLING).set_opcode(0o000).set_name("dtp-call-compile-odd-prefetch".to_string())
        // Instruction::new().set_family(InstructionFamily::FUNCTION_CALLING).set_opcode(0o000).set_name("dtp-call-indirect".to_string())
        // Instruction::new().set_family(InstructionFamily::FUNCTION_CALLING).set_opcode(0o000).set_name("dtp-call-indirect-prefetch".to_string())
        // Instruction::new().set_family(InstructionFamily::FUNCTION_CALLING).set_opcode(0o000).set_name("dtp-call-generic".to_string())
        // Instruction::new().set_family(InstructionFamily::FUNCTION_CALLING).set_opcode(0o000).set_name("dtp-call-generic-prefetch".to_string())
        Instruction::new()
            .set_name("start-call".to_string())
            .set_family(InstructionFamily::FunctionCalling)
            .set_format(InstructionFormat::OperandFromStack)
            .set_opcode(OpCode::StartCall)
            .set_exec(Some(VirtualMachine::cpu_start_call)),
        Instruction::new()
            .set_name("finish-call-n".to_string())
            .set_family(InstructionFamily::FunctionCalling)
            .set_format(InstructionFormat::Immediate10Bits)
            .set_opcode(OpCode::FinishCallN)
            .set_exec(Some(VirtualMachine::cpu_finish_call_n)),
        Instruction::new()
            .set_name("finish-call-n-apply".to_string())
            .set_family(InstructionFamily::FunctionCalling)
            .set_format(InstructionFormat::Immediate10Bits)
            .set_opcode(OpCode::FinishCallNApply)
            .set_exec(Some(VirtualMachine::cpu_finish_call_n_apply)),
        Instruction::new()
            .set_name("finish-call-tos".to_string())
            .set_family(InstructionFamily::FunctionCalling)
            .set_format(InstructionFormat::Immediate10Bits)
            .set_opcode(OpCode::FinishCallTos)
            .set_exec(Some(VirtualMachine::cpu_finish_call_tos)),
        Instruction::new()
            .set_name("finish-call-tos-apply".to_string())
            .set_family(InstructionFamily::FunctionCalling)
            .set_format(InstructionFormat::Immediate10Bits)
            .set_opcode(OpCode::FinishCallTosApply)
            .set_exec(Some(VirtualMachine::cpu_finish_call_tos_apply)),
        Instruction::new()
            .set_name("entry-rest-accepted".to_string())
            .set_family(InstructionFamily::FunctionCalling)
            .set_format(InstructionFormat::Immediate10Bits)
            .set_opcode(OpCode::EntryRestAccepted)
            .set_exec(Some(VirtualMachine::cpu_entry_rest_accepted)),
        Instruction::new()
            .set_name("entry-rest-not-accepted".to_string())
            .set_family(InstructionFamily::FunctionCalling)
            .set_format(InstructionFormat::EntryInstruction)
            .set_opcode(OpCode::EntryRestNotAccepted)
            .set_exec(Some(VirtualMachine::cpu_entry_rest_not_accepted)),
        Instruction::new()
            .set_name("locate-locals".to_string())
            .set_family(InstructionFamily::FunctionCalling)
            .set_format(InstructionFormat::OperandFromStack)
            .set_opcode(OpCode::LocateLocals)
            .set_exec(Some(VirtualMachine::cpu_locate_locals)),
        Instruction::new()
            .set_name("return-single".to_string())
            .set_family(InstructionFamily::FunctionCalling)
            .set_format(InstructionFormat::Immediate10Bits)
            .set_opcode(OpCode::ReturnSingle)
            .set_exec(Some(VirtualMachine::cpu_return_single)),
        Instruction::new()
            .set_name("return-multiple".to_string())
            .set_family(InstructionFamily::FunctionCalling)
            .set_format(InstructionFormat::OperandFromStack)
            .set_opcode(OpCode::ReturnMultiple)
            .set_exec(Some(VirtualMachine::cpu_return_multiple)),
        Instruction::new()
            .set_name("return-kludge".to_string())
            .set_family(InstructionFamily::FunctionCalling)
            .set_format(InstructionFormat::OperandFromStack)
            .set_opcode(OpCode::ReturnKludge)
            .set_exec(Some(VirtualMachine::cpu_return_kludge)),
        Instruction::new()
            .set_name("take-values".to_string())
            .set_family(InstructionFamily::FunctionCalling)
            .set_format(InstructionFormat::OperandFromStack)
            .set_opcode(OpCode::TakeValues)
            .set_exec(Some(VirtualMachine::cpu_take_values)),
    ];
    //
    // Instruction::new().set_family(InstructionFamily::FUNCTION_CALLING).set_opcode(0o000).set_name("dtp-call-compile-even-prefetch".to_string()) // Instruction::new().set_family(InstructionFamily::FUNCTION_CALLING).set_opcode(0o000).set_name("dtp-call-compile-odd".to_string())
    // Instruction::new().set_family(InstructionFamily::FUNCTION_CALLING).set_opcode(0o000).set_name("dtp-call-compile-odd-prefetch".to_string())
    // Instruction::new().set_family(InstructionFamily::FUNCTION_CALLING).set_opcode(0o000).set_name("dtp-call-indirect".to_string())
    // Instruction::new().set_family(InstructionFamily::FUNCTION_CALLING).set_opcode(0o000).set_name("dtp-call-indirect-prefetch".to_string())
    // Instruction::new().set_family(InstructionFamily::FUNCTION_CALLING).set_opcode(0o000).set_name("dtp-call-generic".to_string())
    // Instruction::new().set_family(InstructionFamily::FUNCTION_CALLING).set_opcode(0o000).set_name("dtp-call-generic-prefetch".to_string())
}
