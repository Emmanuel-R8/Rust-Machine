use crate::hardware::machine::VirtualMachine;

// Instruction add
impl VirtualMachine {
    pub fn cpu_interuptible(&mut self) -> &mut Self {
        return self;
    }
}
