use std::mem::size_of;

use crate::common::constants::ADDRESS_SYSTEM_COMM_AREA;
use crate::common::types::MemoryCell;
use crate::world::world::virtual_memory_read;

pub fn read_system_comm_slot(slot: u32) -> MemoryCell {
    return virtual_memory_read(ADDRESS_SYSTEM_COMM_AREA / (size_of::<u32>() as u32) + slot);
}
