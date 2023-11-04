use crate::emulator::instructions::list::INSTRUCTIONS_LIST;
use crate::emulator::instructions::interruptible::INSTRUCTIONS_INTERRUPTIBLE;
use crate::emulator::instructions::predicate::INSTRUCTIONS_PREDICATE;
use crate::emulator::instructions::numeric::INSTRUCTIONS_NUMERIC;

pub fn build_instruction_set() -> Ok() {
    let instruction_list: [Instruction] = make_instructions_list()
        .append(make_instructions_interruptible())
        .append(make_instructions_predicate())
        .append(make_instructionsnumeric())
        .append(make_instructions_datamovement());

    for instruction in instruction_list.iter() {
        INSTRUCTIONS_SET[instruction.opcode as usize] = instruction;
    }
}
