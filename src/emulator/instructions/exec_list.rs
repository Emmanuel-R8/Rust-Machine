use crate::hardware::machine::VirtualMachine;

// Instruction add
impl VirtualMachine {
    pub fn cpu_car(&mut self) -> &mut Self {
        return self;
    }

    pub fn cpu_cdr(&mut self) -> &mut Self {
        return self;
    }

    pub fn cpu_set_to_car(&mut self) -> &mut Self {
        return self;
    }

    pub fn cpu_set_to_cdr(&mut self) -> &mut Self {
        return self;
    }

    pub fn cpu_set_to_cdr_push_car(&mut self) -> &mut Self {
        return self;
    }

    pub fn cpu_rplaca(&mut self) -> &mut Self {
        return self;
    }

    pub fn cpu_rplacd(&mut self) -> &mut Self {
        return self;
    }
}
