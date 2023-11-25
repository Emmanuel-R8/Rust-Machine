use crate::hardware::machine::VirtualMachine;

// Instruction add
impl VirtualMachine {
    pub fn cpu_data_movement(&mut self) -> &mut Self {
        return self;
    }

    pub fn cpu_push(&mut self) -> &mut Self {
        return self;
    }

    pub fn cpu_pop(&mut self) -> &mut Self {
        return self;
    }

    pub fn cpu_movem(&mut self) -> &mut Self {
        return self;
    }

    pub fn cpu_push_n_nils(&mut self) -> &mut Self {
        return self;
    }

    pub fn cpu_push_address(&mut self) -> &mut Self {
        return self;
    }

    pub fn cpu_set_sp_to_address(&mut self) -> &mut Self {
        return self;
    }

    pub fn cpu_set_sp_to_address_save_tos(&mut self) -> &mut Self {
        return self;
    }

    pub fn cpu_push_address_sp_relative(&mut self) -> &mut Self {
        return self;
    }

    pub fn cpu_stack_blt(&mut self) -> &mut Self {
        return self;
    }

    pub fn cpu_stack_blt_address(&mut self) -> &mut Self {
        return self;
    }
}
