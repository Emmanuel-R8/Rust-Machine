use crate::hardware::machine::VirtualMachine;

// Instruction add
impl VirtualMachine {
    pub fn cpu_bind_locative_to_value(&mut self) -> &mut Self {
        return self;
    }

    pub fn cpu_bind_locative(&mut self) -> &mut Self {
        return self;
    }

    pub fn cpu_unbind_n(&mut self) -> &mut Self {
        return self;
    }

    pub fn cpu_restore_binding_stack(&mut self) -> &mut Self {
        return self;
    }
}
