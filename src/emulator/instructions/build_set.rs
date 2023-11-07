use super::array::make_instructions_array;
use super::binding::make_instructions_binding;
use super::block::make_instructions_block;
use super::branch_loop::make_instructions_branch_loop;
use super::catch::make_instructions_catch;
use super::common::{ INSTRUCTIONS_SET, Instruction, InstructionFamily, ImmediateArgumentType };
use super::datamovement::make_instructions_datamovement;
use super::field_extraction::make_instructions_field_extraction;
use super::function_calling::make_instructions_function_calling;
use super::instance_variable::make_instructions_instance_variable;
use super::interruptible::make_instructions_interruptible;
use super::lexical_variable::make_instructions_lexical_variable;
use super::list::make_instructions_list;
use super::numeric::make_instructions_numeric;
use super::predicate::make_instructions_predicate;
use super::subprimitive::make_instructions_subprimitive;

pub fn build_instruction_set() {
    let instructions_list = make_instructions_list();
    let instructions_interruptible = make_instructions_interruptible();
    let instructions_predicate = make_instructions_predicate();
    let instructions_numeric = make_instructions_numeric();
    let instructions_datamovement = make_instructions_datamovement();
    let instructions_field_extraction = make_instructions_field_extraction();
    let instructions_array = make_instructions_array();
    let instructions_branch_loop = make_instructions_branch_loop();
    let instructions_block = make_instructions_block();
    let instructions_function_calling = make_instructions_function_calling();
    let instructions_binding = make_instructions_binding();
    let instructions_catch = make_instructions_catch();
    let instructions_lexical_variable = make_instructions_lexical_variable();
    let instructions_instance_variable = make_instructions_instance_variable();
    let instructions_subprimitive = make_instructions_subprimitive();

    let mut instructions = [
        instructions_list,
        instructions_interruptible,
        instructions_predicate,
        instructions_numeric,
        instructions_datamovement,
        instructions_field_extraction,
        instructions_array,
        instructions_branch_loop,
        instructions_block,
        instructions_function_calling,
        instructions_binding,
        instructions_catch,
        instructions_lexical_variable,
        instructions_instance_variable,
        instructions_subprimitive,
    ].concat();

    let instruction_set: [Box<Instruction<'static>>; 0o777];
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
