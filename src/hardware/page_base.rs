use c2rust_bitfields::BitfieldStruct;

use crate::common::{
    constants::{QTag, VLMPAGE_SIZE_QS},
    memory_cell::MemoryCell,
    types::QImmediate,
};

///
///
/// VLM Page bases
///
#[derive(Default, Debug, Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct VLMPageBases {
    #[bitfield(name = "dataPageBase", ty = "u32", bits = "0..=27")]
    #[bitfield(name = "tagsPageBase", ty = "u32", bits = "28..=31")]
    pub datapagebase_tagspagebase: [u8; 4],
}

#[derive(Default, Debug, Copy, Clone)]
pub struct VMState {
    command_register: u32,
    address_register: u32,
    extent_register: u32,
    attributes_register: u32,
    destination_register: u32,
    data_register: MemoryCell,
}

#[derive(Default, Debug, Copy, Clone)]
#[repr(C)]
pub struct Bar {
    pub address: MemoryCell,
    pub mapped: MemoryCell,
}

pub type VMPageNumber = i32;
pub type VMPageData = [QImmediate; VLMPAGE_SIZE_QS as usize];
pub type VMPageTag = [QTag; VLMPAGE_SIZE_QS as usize];
