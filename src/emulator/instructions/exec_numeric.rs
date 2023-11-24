use crate::hardware::machine::VirtualMachine;

// Instruction add
impl VirtualMachine {
    pub fn cpu_add(&mut self) -> &mut Self {
        let operand1 = self.pop_stack().unwrap();
        let operand2 = self.pop_stack().unwrap();
        let result = operand1 + operand2;

        self.push_stack(result.unwrap());

        return self;
    }

    pub fn cpu_sub(&mut self) -> &mut Self {
        let operand1 = self.pop_stack().unwrap();
        let operand2 = self.pop_stack().unwrap();
        let result = operand1 - operand2;

        self.push_stack(result.unwrap());

        return self;
    }
}
