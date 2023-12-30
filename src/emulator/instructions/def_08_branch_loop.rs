use crate::common::constants::OpCode;
use crate::hardware::machine::VirtualMachine;

use super::common::{ Instruction, InstructionFamily, InstructionFormat };

pub fn make_instructions_branch_loop() -> Vec<Instruction> {
    return vec![
        Instruction::new()
            .set_name("branch".to_string())
            .set_family(InstructionFamily::BranchLoop)
            .set_format(InstructionFormat::Immediate10Bits)
            .set_opcode(OpCode::Branch)
            .set_exec(Some(VirtualMachine::cpu_branch)),
        Instruction::new()
            .set_name("branch-true".to_string())
            .set_family(InstructionFamily::BranchLoop)
            .set_format(InstructionFormat::Immediate10Bits)
            .set_opcode(OpCode::BranchTrue)
            .set_exec(Some(VirtualMachine::cpu_branch_true)),
        Instruction::new()
            .set_name("branch-false".to_string())
            .set_family(InstructionFamily::BranchLoop)
            .set_format(InstructionFormat::Immediate10Bits)
            .set_opcode(OpCode::BranchTrue)
            .set_exec(Some(VirtualMachine::cpu_branch_false)),
        Instruction::new()
            .set_name("branch-true-no-pop".to_string())
            .set_family(InstructionFamily::BranchLoop)
            .set_format(InstructionFormat::Immediate10Bits)
            .set_opcode(OpCode::BranchTrueNoPop)
            .set_exec(Some(VirtualMachine::cpu_branch_true_no_pop)),
        Instruction::new()
            .set_name("branch-false-no-pop".to_string())
            .set_family(InstructionFamily::BranchLoop)
            .set_format(InstructionFormat::Immediate10Bits)
            .set_opcode(OpCode::BranchFalseNoPop)
            .set_exec(Some(VirtualMachine::cpu_branch_false_no_pop)),
        Instruction::new()
            .set_name("branch-true-else-no-pop".to_string())
            .set_family(InstructionFamily::BranchLoop)
            .set_format(InstructionFormat::Immediate10Bits)
            .set_opcode(OpCode::BranchTrueElseNoPop)
            .set_exec(Some(VirtualMachine::cpu_branch_true_else_no_pop)),
        Instruction::new()
            .set_name("branch-false-else-no-pop".to_string())
            .set_family(InstructionFamily::BranchLoop)
            .set_format(InstructionFormat::Immediate10Bits)
            .set_opcode(OpCode::BranchFalseElseNoPop)
            .set_exec(Some(VirtualMachine::cpu_branch_false_else_no_pop)),
        Instruction::new()
            .set_name("branch-true-and-no-pop".to_string())
            .set_family(InstructionFamily::BranchLoop)
            .set_format(InstructionFormat::Immediate10Bits)
            .set_opcode(OpCode::BranchTrueAndNoPop)
            .set_exec(Some(VirtualMachine::cpu_branch_true_and_no_pop)),
        Instruction::new()
            .set_name("branch-false-and-no-pop".to_string())
            .set_family(InstructionFamily::BranchLoop)
            .set_format(InstructionFormat::Immediate10Bits)
            .set_opcode(OpCode::BranchFalseAndNoPop)
            .set_exec(Some(VirtualMachine::cpu_branch_false_and_no_pop)),
        Instruction::new()
            .set_name("branch-true-and-extra-pop".to_string())
            .set_family(InstructionFamily::BranchLoop)
            .set_format(InstructionFormat::Immediate10Bits)
            .set_opcode(OpCode::BranchTrueAndExtraPop)
            .set_exec(Some(VirtualMachine::cpu_branch_true_and_extra_pop)),
        Instruction::new()
            .set_name("branch-false-and-extra-pop".to_string())
            .set_family(InstructionFamily::BranchLoop)
            .set_format(InstructionFormat::Immediate10Bits)
            .set_opcode(OpCode::BranchFalseAndExtraPop)
            .set_exec(Some(VirtualMachine::cpu_branch_false_and_extra_pop)),
        Instruction::new()
            .set_name("branch-true-else-extra-pop".to_string())
            .set_family(InstructionFamily::BranchLoop)
            .set_format(InstructionFormat::Immediate10Bits)
            .set_opcode(OpCode::BranchTrueElseExtraPop)
            .set_exec(Some(VirtualMachine::cpu_branch_true_else_extra_pop)),
        Instruction::new()
            .set_name("branch-false-else-extra-pop".to_string())
            .set_family(InstructionFamily::BranchLoop)
            .set_format(InstructionFormat::Immediate10Bits)
            .set_opcode(OpCode::BranchFalseElseExtraPop)
            .set_exec(Some(VirtualMachine::cpu_branch_false_else_extra_pop)),
        Instruction::new()
            .set_name("branch-true-extra-pop".to_string())
            .set_family(InstructionFamily::BranchLoop)
            .set_format(InstructionFormat::Immediate10Bits)
            .set_opcode(OpCode::BranchTrueExtraPop)
            .set_exec(Some(VirtualMachine::cpu_branch_true_extra_pop)),
        Instruction::new()
            .set_name("branch-false-extra-pop".to_string())
            .set_family(InstructionFamily::BranchLoop)
            .set_format(InstructionFormat::Immediate10Bits)
            .set_opcode(OpCode::BranchFalseExtraPop)
            .set_exec(Some(VirtualMachine::cpu_branch_false_extra_pop)),
        Instruction::new()
            .set_name("branch-true-and-no-pop-else-no-pop-extra-pop".to_string())
            .set_family(InstructionFamily::BranchLoop)
            .set_format(InstructionFormat::Immediate10Bits)
            .set_opcode(OpCode::BranchTrueAndNoPopElseNoPopExtraPop)
            .set_exec(Some(VirtualMachine::cpu_branch_true_and_no_pop_else_no_pop_extra_pop)),
        Instruction::new()
            .set_name("branch-false-and-no-pop-else-no-pop-extra-pop".to_string())
            .set_family(InstructionFamily::BranchLoop)
            .set_format(InstructionFormat::Immediate10Bits)
            .set_opcode(OpCode::BranchFalseAndNoPopElseNoPopExtraPop)
            .set_exec(Some(VirtualMachine::cpu_branch_false_and_no_pop_else_no_pop_extra_pop)),
        Instruction::new()
            .set_name("loop-decrement-tos".to_string())
            .set_family(InstructionFamily::BranchLoop)
            .set_format(InstructionFormat::Immediate10Bits)
            .set_opcode(OpCode::LoopDecrementTos)
            .set_exec(Some(VirtualMachine::cpu_loop_decrement_tos)),
        Instruction::new()
            .set_name("loop-decrement-tos-less-than".to_string())
            .set_family(InstructionFamily::BranchLoop)
            .set_format(InstructionFormat::Immediate10Bits)
            .set_opcode(OpCode::LoopIncrementTosLessThan)
            .set_exec(Some(VirtualMachine::cpu_loop_decrement_tos_less_than))
    ];
}
