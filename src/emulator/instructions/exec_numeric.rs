use crate::hardware::machine::VirtualMachine;

// Instruction add
// Instruction add
pub fn cpu_add(vm: &mut VirtualMachine) {
    let operand1 = vm.pop_stack().unwrap();
    let operand2 = vm.pop_stack().unwrap();
    let result = operand1 + operand2;

    vm.push_stack(result.unwrap());
    // return self;
}

// Instruction sub
pub fn cpu_sub(vm: &mut VirtualMachine) {
    let operand1 = vm.pop_stack().unwrap();
    let operand2 = vm.pop_stack().unwrap();
    let result = operand1 - operand2;

    vm.push_stack(result.unwrap());
    // return self;
}
