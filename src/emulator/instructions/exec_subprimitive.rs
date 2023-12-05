use crate::hardware::machine::VirtualMachine;

// Instruction add
impl VirtualMachine {
    pub fn cpu_ephemeralp(&mut self) -> &mut Self {
        return self;
    }

    pub fn cpu_unsigned_lessp(&mut self) -> &mut Self {
        return self;
    }

    pub fn cpu_unsigned_lessp_no_pop(&mut self) -> &mut Self {
        return self;
    }

    pub fn cpu_allocate_list_block(&mut self) -> &mut Self {
        return self;
    }

    pub fn cpu_allocate_structure_block(&mut self) -> &mut Self {
        return self;
    }

    pub fn cpu_pointer_plus(&mut self) -> &mut Self {
        return self;
    }

    pub fn cpu_pointer_difference(&mut self) -> &mut Self {
        return self;
    }

    pub fn cpu_pointer_increment(&mut self) -> &mut Self {
        return self;
    }

    pub fn cpu_read_internal_register(&mut self) -> &mut Self {
        return self;
    }

    pub fn cpu_write_internal_register(&mut self) -> &mut Self {
        return self;
    }

    pub fn cpu_no_op(&mut self) -> &mut Self {
        return self;
    }

    pub fn cpu_coprocessor_read(&mut self) -> &mut Self {
        return self;
    }

    pub fn cpu_coprocessor_write(&mut self) -> &mut Self {
        return self;
    }

    pub fn cpu_memory_read(&mut self) -> &mut Self {
        return self;
    }

    pub fn cpu_memory_read_address(&mut self) -> &mut Self {
        return self;
    }

    pub fn cpu_tag(&mut self) -> &mut Self {
        return self;
    }

    pub fn cpu_set_tag(&mut self) -> &mut Self {
        return self;
    }

    pub fn cpu_store_conditional(&mut self) -> &mut Self {
        return self;
    }

    pub fn cpu_p_store_contents(&mut self) -> &mut Self {
        return self;
    }

    pub fn cpu_memory_write(&mut self) -> &mut Self {
        return self;
    }

    pub fn cpu_set_cdr_code_1(&mut self) -> &mut Self {
        return self;
    }

    pub fn cpu_set_cdr_code_2(&mut self) -> &mut Self {
        return self;
    }

    pub fn cpu_merge_cdr_no_pop(&mut self) -> &mut Self {
        return self;
    }

    pub fn cpu_generic_dispatch(&mut self) -> &mut Self {
        return self;
    }

    pub fn cpu_message_dispatch(&mut self) -> &mut Self {
        return self;
    }

    pub fn cpu_jump(&mut self) -> &mut Self {
        return self;
    }

    pub fn cpu_check_preempt_request(&mut self) -> &mut Self {
        return self;
    }

    pub fn cpu_halt(&mut self) -> &mut Self {
        return self;
    }
}
