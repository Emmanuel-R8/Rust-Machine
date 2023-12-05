use crate::hardware::machine::VirtualMachine;

// Instruction add
impl VirtualMachine {
    pub fn cpu_start_call(&mut self) -> &mut Self {
        return self;
    }

    pub fn cpu_finish_call_n(&mut self) -> &mut Self {
        return self;
    }

    pub fn cpu_finish_call_n_apply(&mut self) -> &mut Self {
        return self;
    }

    pub fn cpu_finish_call_tos(&mut self) -> &mut Self {
        return self;
    }

    pub fn cpu_finish_call_tos_apply(&mut self) -> &mut Self {
        return self;
    }

    pub fn cpu_entry_rest_accepted(&mut self) -> &mut Self {
        return self;
    }

    pub fn cpu_entry_rest_not_accepted(&mut self) -> &mut Self {
        return self;
    }

    pub fn cpu_locate_locals(&mut self) -> &mut Self {
        return self;
    }

    pub fn cpu_return_single(&mut self) -> &mut Self {
        return self;
    }

    pub fn cpu_return_multiple(&mut self) -> &mut Self {
        return self;
    }

    pub fn cpu_return_kludge(&mut self) -> &mut Self {
        return self;
    }

    pub fn cpu_take_values(&mut self) -> &mut Self {
        return self;
    }
}
