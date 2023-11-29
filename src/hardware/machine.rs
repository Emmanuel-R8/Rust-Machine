use crate::common::constants::VMAttribute;
use crate::common::memory_cell::MemoryCell;
use crate::hardware::cpu::CPU;
use crate::hardware::memory::VMMemory;

// Structure of a virtual machine
pub struct VirtualMachine {
    cpu: CPU,
    mem: VMMemory,

    vm_attribute_table: [VMAttribute; 0x8_00_00],

    unmapped_world_words: u32,
    mapped_world_words: u32,
    file_map_entries: u32,
    swap_map_entries: u32,
}

impl VirtualMachine {
    pub fn pop_stack(&mut self) -> Option<MemoryCell> {
        let sp = self.cpu.sp;
        let cell = self.mem.read(sp);
        self.cpu.sp.inc_mut();

        return cell;
    }

    pub fn push_stack(&mut self, val: MemoryCell) -> &Self {
        self.cpu.sp.dec_mut();

        let sp = self.cpu.sp;
        let cell = self.mem.write(sp, val);

        return self;
    }
}
