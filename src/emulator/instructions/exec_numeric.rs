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

        let result = (operand1 - operand2).unwrap();
        self.push_stack(result);

        return self;
    }

    pub fn cpu_unary_minus(&mut self) -> &mut Self {
        let operand = self.pop_stack().unwrap();

        let result = -operand;
        self.push_stack(result.unwrap());

        return self;
    }

    pub fn cpu_increment(&mut self) -> &mut Self {
        return self;
    }

    pub fn cpu_decrement(&mut self) -> &mut Self {
        return self;
    }

    pub fn cpu_multiply(&mut self) -> &mut Self {
        return self;
    }

    pub fn cpu_quotient(&mut self) -> &mut Self {
        return self;
    }

    pub fn cpu_ceiling(&mut self) -> &mut Self {
        return self;
    }

    pub fn cpu_floor(&mut self) -> &mut Self {
        return self;
    }

    pub fn cpu_truncate(&mut self) -> &mut Self {
        return self;
    }

    pub fn cpu_round(&mut self) -> &mut Self {
        return self;
    }

    pub fn cpu_remainder(&mut self) -> &mut Self {
        return self;
    }

    pub fn cpu_rational_quotient(&mut self) -> &mut Self {
        return self;
    }

    pub fn cpu_min(&mut self) -> &mut Self {
        return self;
    }

    pub fn cpu_max(&mut self) -> &mut Self {
        return self;
    }

    pub fn cpu_logand(&mut self) -> &mut Self {
        return self;
    }

    pub fn cpu_logxor(&mut self) -> &mut Self {
        return self;
    }

    pub fn cpu_logior(&mut self) -> &mut Self {
        return self;
    }

    pub fn cpu_rot(&mut self) -> &mut Self {
        return self;
    }

    pub fn cpu_lsh(&mut self) -> &mut Self {
        return self;
    }

    pub fn cpu_multiply_divide(&mut self) -> &mut Self {
        return self;
    }

    pub fn cpu_ash(&mut self) -> &mut Self {
        return self;
    }

    pub fn cpu_32_bit_plus(&mut self) -> &mut Self {
        return self;
    }

    pub fn cpu_32_bit_difference(&mut self) -> &mut Self {
        return self;
    }

    pub fn cpu_add_bignum_step(&mut self) -> &mut Self {
        return self;
    }

    pub fn cpu_sub_bignum_step(&mut self) -> &mut Self {
        return self;
    }

    pub fn cpu_multiply_bignum_step(&mut self) -> &mut Self {
        return self;
    }

    pub fn cpu_divide_bignum_step(&mut self) -> &mut Self {
        return self;
    }
}
