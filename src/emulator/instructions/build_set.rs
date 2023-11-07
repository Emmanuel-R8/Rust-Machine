// use crate::emulator::instructions::list::INSTRUCTIONS_LIST;
// use crate::emulator::instructions::interruptible::INSTRUCTIONS_INTERRUPTIBLE;
// use crate::emulator::instructions::predicate::INSTRUCTIONS_PREDICATE;
// use crate::emulator::instructions::numeric::INSTRUCTIONS_NUMERIC;

use std::borrow::BorrowMut;

use super::common::{INSTRUCTIONS_SET, Instruction, InstructionFamily, ImmediateArgumentType};
use super::datamovement::make_instructions_datamovement;
use super::field_extraction::make_instructions_field_extraction;
use super::interruptible::make_instructions_interruptible;
use super::list::make_instructions_list;
use super::numeric::make_instructions_numeric;
use super::predicate::make_instructions_predicate;

pub fn build_instruction_set() {
    let instructions_list = make_instructions_list();
    let instructions_interruptible = make_instructions_interruptible();
    let instructions_predicate = make_instructions_predicate();
    let instructions_numeric = make_instructions_numeric();
    let instructions_datamovement = make_instructions_datamovement();
    let instructions_field_extraction = make_instructions_field_extraction();

    let mut instructions = [
        instructions_list,
        instructions_interruptible,
        instructions_predicate,
        instructions_numeric,
        instructions_datamovement,
        instructions_field_extraction,
    ].concat();

    let instruction_set: [Box<Instruction<'static>>; 0o777] ;
    for (i, _) in instruction_set.iter_mut().enumerate() {
        instruction_set[i] = Box::new(Instruction {
            family: InstructionFamily::UNDEFINED,
            arg_count: 0,
            ret_count: 0,
            immediate_arg_type: ImmediateArgumentType::UNDEFINED,
            name: "unknown",
            opcode: 0,
            is_implemented: false,
        });
    }

    for instruction in &instructions {
        let opcode = instruction.opcode as usize;
        let inst = instruction.clone();

        instruction_set[opcode] = inst;
    }
}
