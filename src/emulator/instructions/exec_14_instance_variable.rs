use crate::hardware::machine::VirtualMachine;

// Instruction add
impl VirtualMachine {
    pub fn cpu_push_instance_variable(&mut self) -> &mut Self {
        return self;
    }

    pub fn cpu_pop_instance_variable(&mut self) -> &mut Self {
        return self;
    }

    pub fn cpu_movem_instance_variable(&mut self) -> &mut Self {
        return self;
    }

    pub fn cpu_push_address_instance_variable(&mut self) -> &mut Self {
        return self;
    }

    pub fn cpu_push_instance_variable_ordered(&mut self) -> &mut Self {
        return self;
    }

    pub fn cpu_pop_instance_variable_ordered(&mut self) -> &mut Self {
        return self;
    }

    pub fn cpu_movem_instance_variable_ordered(&mut self) -> &mut Self {
        return self;
    }

    pub fn cpu_push_address_instance_variable_ordered(&mut self) -> &mut Self {
        return self;
    }

    pub fn cpu_instance_ref(&mut self) -> &mut Self {
        return self;
    }

    pub fn cpu_instance_set(&mut self) -> &mut Self {
        return self;
    }

    pub fn cpu_instance_loc(&mut self) -> &mut Self {
        return self;
    }
}
