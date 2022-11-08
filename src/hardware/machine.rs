use crate::common::constants::VMAttribute;
use crate::hardware::cpu::{CPU};
use crate::hardware::memory::{VMMemory};

// Structure of a virtual machine
pub struct VirtualMachine {
    cpu: CPU,
    mem : VMMemory,

    vm_attribute_table: [VMAttribute; 0x8_00_00],

    unmapped_world_words: u32,
    mapped_world_words: u32,
    file_map_entries: u32,
    swap_map_entries: u32,
}
