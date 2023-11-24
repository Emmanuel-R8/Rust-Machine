use std::collections::HashMap;

use super::def_array::make_instructions_array;
use super::def_binding::make_instructions_binding;
use super::def_block::make_instructions_block;
use super::def_branch_loop::make_instructions_branch_loop;
use super::def_catch::make_instructions_catch;
use super::common::Instruction;
use super::def_data_movement::make_instructions_data_movement;
use super::def_field_extraction::make_instructions_field_extraction;
use super::def_function_calling::make_instructions_function_calling;
use super::def_instance_variable::make_instructions_instance_variable;
use super::def_interruptible::make_instructions_interruptible;
use super::def_lexical_variable::make_instructions_lexical_variable;
use super::def_list::make_instructions_list;
use super::def_numeric::make_instructions_numeric;
use super::def_predicate::make_instructions_predicate;
use super::def_subprimitive::make_instructions_subprimitive;

pub fn build_instruction_vec_map() -> (
    Vec<Option<Box<Instruction<'static>>>>,
    HashMap<&'static str, u32>,
) {
    let instructions_list = make_instructions_list();
    let instructions_interruptible = make_instructions_interruptible();
    let instructions_predicate = make_instructions_predicate();
    let instructions_numeric = make_instructions_numeric();
    let instructions_datamovement = make_instructions_data_movement();
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

    // Create an array where instructions accessed by opcode
    let mut instruction_set: Vec<Option<Box<Instruction<'static>>>> = vec![None; 0o777];

    // Create a hasmap were opcode are mapped from opcode - used for populating execution code
    let mut instruction_map: HashMap<&str, u32> = HashMap::new();

    for instruction in instructions.iter_mut() {
        let opcode = instruction.opcode as usize;
        instruction_set[opcode] = Some(Box::new(instruction.clone()));

        instruction_map.insert(instruction.name, instruction.opcode);
    }

    return (instruction_set, instruction_map);
}
