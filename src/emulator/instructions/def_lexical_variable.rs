use crate::hardware::machine::VirtualMachine;

use super::common::{ Instruction, InstructionFamily, InstructionFormat };

pub fn make_instructions_lexical_variable() -> Vec<Instruction> {
    return vec![
        Instruction::new()
            .set_name("push-lexical-var-0".to_string())
            .set_family(InstructionFamily::LexicalVariableAccess)
            .set_format(InstructionFormat::OperandFromStack)
            .set_opcode(0o020)
            .set_exec(Some(VirtualMachine::cpu_push_lexical_var_0)),
        Instruction::new()
            .set_name("push-lexical-var-1".to_string())
            .set_family(InstructionFamily::LexicalVariableAccess)
            .set_format(InstructionFormat::OperandFromStack)
            .set_opcode(0o021)
            .set_exec(Some(VirtualMachine::cpu_push_lexical_var_1)),
        Instruction::new()
            .set_name("push-lexical-var-2".to_string())
            .set_family(InstructionFamily::LexicalVariableAccess)
            .set_format(InstructionFormat::OperandFromStack)
            .set_opcode(0o022)
            .set_exec(Some(VirtualMachine::cpu_push_lexical_var_2)),
        Instruction::new()
            .set_name("push-lexical-var-3".to_string())
            .set_family(InstructionFamily::LexicalVariableAccess)
            .set_format(InstructionFormat::OperandFromStack)
            .set_opcode(0o023)
            .set_exec(Some(VirtualMachine::cpu_push_lexical_var_3)),
        Instruction::new()
            .set_name("push-lexical-var-4".to_string())
            .set_family(InstructionFamily::LexicalVariableAccess)
            .set_format(InstructionFormat::OperandFromStack)
            .set_opcode(0o024)
            .set_exec(Some(VirtualMachine::cpu_push_lexical_var_4)),
        Instruction::new()
            .set_name("push-lexical-var-5".to_string())
            .set_family(InstructionFamily::LexicalVariableAccess)
            .set_format(InstructionFormat::OperandFromStack)
            .set_opcode(0o025)
            .set_exec(Some(VirtualMachine::cpu_push_lexical_var_5)),
        Instruction::new()
            .set_name("push-lexical-var-6".to_string())
            .set_family(InstructionFamily::LexicalVariableAccess)
            .set_format(InstructionFormat::OperandFromStack)
            .set_opcode(0o026)
            .set_exec(Some(VirtualMachine::cpu_push_lexical_var_6)),
        Instruction::new()
            .set_name("push-lexical-var-7".to_string())
            .set_family(InstructionFamily::LexicalVariableAccess)
            .set_format(InstructionFormat::OperandFromStack)
            .set_opcode(0o027)
            .set_exec(Some(VirtualMachine::cpu_push_lexical_var_7)),
        Instruction::new()
            .set_name("pop-lexical-var-0".to_string())
            .set_family(InstructionFamily::LexicalVariableAccess)
            .set_format(InstructionFormat::OperandFromStack)
            .set_opcode(0o240)
            .set_exec(Some(VirtualMachine::cpu_pop_lexical_var_0)),
        Instruction::new()
            .set_name("pop-lexical-var-1".to_string())
            .set_family(InstructionFamily::LexicalVariableAccess)
            .set_format(InstructionFormat::OperandFromStack)
            .set_opcode(0o241)
            .set_exec(Some(VirtualMachine::cpu_pop_lexical_var_1)),
        Instruction::new()
            .set_name("pop-lexical-var-2".to_string())
            .set_family(InstructionFamily::LexicalVariableAccess)
            .set_format(InstructionFormat::OperandFromStack)
            .set_opcode(0o242)
            .set_exec(Some(VirtualMachine::cpu_pop_lexical_var_2)),
        Instruction::new()
            .set_name("pop-lexical-var-3".to_string())
            .set_family(InstructionFamily::LexicalVariableAccess)
            .set_format(InstructionFormat::OperandFromStack)
            .set_opcode(0o243)
            .set_exec(Some(VirtualMachine::cpu_pop_lexical_var_3)),
        Instruction::new()
            .set_name("pop-lexical-var-4".to_string())
            .set_family(InstructionFamily::LexicalVariableAccess)
            .set_format(InstructionFormat::OperandFromStack)
            .set_opcode(0o244)
            .set_exec(Some(VirtualMachine::cpu_pop_lexical_var_4)),
        Instruction::new()
            .set_name("pop-lexical-var-5".to_string())
            .set_family(InstructionFamily::LexicalVariableAccess)
            .set_format(InstructionFormat::OperandFromStack)
            .set_opcode(0o245)
            .set_exec(Some(VirtualMachine::cpu_pop_lexical_var_5)),
        Instruction::new()
            .set_name("pop-lexical-var-6".to_string())
            .set_family(InstructionFamily::LexicalVariableAccess)
            .set_format(InstructionFormat::OperandFromStack)
            .set_opcode(0o246)
            .set_exec(Some(VirtualMachine::cpu_pop_lexical_var_6)),
        Instruction::new()
            .set_name("pop-lexical-var-7".to_string())
            .set_family(InstructionFamily::LexicalVariableAccess)
            .set_format(InstructionFormat::OperandFromStack)
            .set_opcode(0o247)
            .set_exec(Some(VirtualMachine::cpu_pop_lexical_var_7)),
        Instruction::new()
            .set_name("movem-lexical-var-0".to_string())
            .set_family(InstructionFamily::LexicalVariableAccess)
            .set_format(InstructionFormat::OperandFromStack)
            .set_opcode(0o250)
            .set_exec(Some(VirtualMachine::cpu_movem_lexical_var_0)),
        Instruction::new()
            .set_name("movem-lexical-var-1".to_string())
            .set_family(InstructionFamily::LexicalVariableAccess)
            .set_format(InstructionFormat::OperandFromStack)
            .set_opcode(0o251)
            .set_exec(Some(VirtualMachine::cpu_movem_lexical_var_1)),
        Instruction::new()
            .set_name("movem-lexical-var-2".to_string())
            .set_family(InstructionFamily::LexicalVariableAccess)
            .set_format(InstructionFormat::OperandFromStack)
            .set_opcode(0o252)
            .set_exec(Some(VirtualMachine::cpu_movem_lexical_var_2)),
        Instruction::new()
            .set_name("movem-lexical-var-3".to_string())
            .set_family(InstructionFamily::LexicalVariableAccess)
            .set_format(InstructionFormat::OperandFromStack)
            .set_opcode(0o253)
            .set_exec(Some(VirtualMachine::cpu_movem_lexical_var_3)),
        Instruction::new()
            .set_name("movem-lexical-var-4".to_string())
            .set_family(InstructionFamily::LexicalVariableAccess)
            .set_format(InstructionFormat::OperandFromStack)
            .set_opcode(0o254)
            .set_exec(Some(VirtualMachine::cpu_movem_lexical_var_4)),
        Instruction::new()
            .set_name("movem-lexical-var-5".to_string())
            .set_family(InstructionFamily::LexicalVariableAccess)
            .set_format(InstructionFormat::OperandFromStack)
            .set_opcode(0o255)
            .set_exec(Some(VirtualMachine::cpu_movem_lexical_var_5)),
        Instruction::new()
            .set_name("movem-lexical-var-6".to_string())
            .set_family(InstructionFamily::LexicalVariableAccess)
            .set_format(InstructionFormat::OperandFromStack)
            .set_opcode(0o256)
            .set_exec(Some(VirtualMachine::cpu_movem_lexical_var_6)),
        Instruction::new()
            .set_name("movem-lexical-var-7".to_string())
            .set_family(InstructionFamily::LexicalVariableAccess)
            .set_format(InstructionFormat::OperandFromStack)
            .set_opcode(0o257)
            .set_exec(Some(VirtualMachine::cpu_movem_lexical_var_7))
    ];
}
