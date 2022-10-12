    use c2rust_bitfields::BitfieldStruct;
use modular_bitfield::specifiers::{B1, B16, B2, B20, B23, B24, B28, B3, B30, B4, B6, B8};

// Representation of lisp objects
use crate::common::constants::QTag;
use super::constants::VLMPAGE_SIZE_QS;

#[derive(Copy, Clone)]
pub union QData {
    pub u: u32,
    pub s: i32,
    pub f: f32,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct QTagdata {
    pub tag: QTag,
    pub data: QData,
}

#[derive(Copy, Clone)]
pub union LispQ {
    pub whole: u64,
    pub parts: QTagdata,
}

#[derive(Debug, Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct Op {
    #[bitfield(name = "count", ty = "B24", bits = "0..=23")]
    #[bitfield(name = "opcode", ty = "B8", bits = "24..=31")]
    pub count_opcode: [u8; 4],
}

#[derive(Debug, Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    #[bitfield(name = "data", ty = "B16", bits = "0..=15")]
    #[bitfield(name = "padding", ty = "B16", bits = "16..=31")]
    pub data: [u8; 4],
}

#[derive(Debug, Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct Fep {
    #[bitfield(name = "status", ty = "B8", bits = "0..=7")]
    #[bitfield(name = "cursor", ty = "B1", bits = "8..=8")]
    #[bitfield(name = "busy", ty = "B1", bits = "9..=9")]
    #[bitfield(name = "error", ty = "B1", bits = "10..=10")]
    #[bitfield(name = "lisp_is_loaded", ty = "B1", bits = "11..=11")]
    #[bitfield(name = "padding", ty = "B20", bits = "12..=31")]
    pub status_cursor_busy_error_lisp_is_loaded: [u8; 4],
}

#[derive(Debug, Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct GeneraVersion {
    #[bitfield(name = "minor", ty = "B16", bits = "0..=15")]
    #[bitfield(name = "major", ty = "B16", bits = "16..=31")]
    pub minor_major: [u8; 4],
}

#[derive(Debug, Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct EmbDiskChannelFlags {
    #[bitfield(name = "host_byte_order", ty = "B1", bits = "0..=0")]
    #[bitfield(name = "read_only", ty = "B1", bits = "1..=1")]
    #[bitfield(name = "padding", ty = "B30", bits = "2..=31")]
    pub host_byte_order_read_only: [u8; 4],
}

#[derive(Debug, Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct EmbDiskOperation {
    #[bitfield(name = "cmd", ty = "B3", bits = "0..=2")]
    #[bitfield(name = "tagged", ty = "B1", bits = "3..=3")]
    #[bitfield(name = "buffered", ty = "B1", bits = "4..=4")]
    #[bitfield(name = "pad1", ty = "B3", bits = "5..=7")]
    #[bitfield(name = "suppress_error_recovery", ty = "B1", bits = "8..=8")]
    #[bitfield(name = "pad2", ty = "B23", bits = "9..=31")]
    pub cmd_tagged_buffered_suppresserrorrecovery: [u8; 4],
}

///////////////////////////////////////////////////////////////////////////////////////////////////
//
// FIXME What are those?
//
///////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug, Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct Control {
    #[bitfield(name = "argument_size", ty = "B8", bits = "0..=7")] // 0b00000000_00000000_00000000_11111111
    #[bitfield(name = "extra_argument", ty = "B1", bits = "8..=8")] // 0b00000000_00000000_00000001_00000000
    #[bitfield(name = "caller_frame_size", ty = "B6", bits = "9..=15")] // 0b00000000_00000000_01111110_00000000
    #[bitfield(name = "with_gap2", ty = "B2", bits = "16..=17")]
    #[bitfield(name = "apply", ty = "B1", bits = "18..=18")] // 0b00000000_00000010_00000000_00000000
    #[bitfield(name = "value_disposition", ty = "B2", bits = "19..=21")] // 0b00000000_00001100_00000000_00000000
    #[bitfield(name = "call_started", ty = "B1", bits = "22..=22")] // 0b00000000_00100000_00000000_00000000
    #[bitfield(name = "cleanup_bits", ty = "B3", bits = "23..=25")] // 0b00000000_11100000_00000000_00000000
    #[bitfield(name = "with_stub", ty = "B8", bits = "26..=31")]
    pub argsize_extraarg_callerframesize_apply_valuedisposition_callstarted_cleanbits: [u8; 4],
}

pub type PC = LispQ;
#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct InstructionCacheLine {
    pub pc: PC,
    pub next_pc: PC,
    pub code: u32,
    pub operand: u32,
    pub instruction: u32,
    pub next_cp: *mut InstructionCacheLine,
}

#[derive(Debug, Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct VLMPageBases {
    #[bitfield(name = "dataPageBase", ty = "B28", bits = "0..=27")]
    #[bitfield(name = "tagsPageBase", ty = "B4", bits = "28..=31")]
    pub dataPageBase_tagsPageBase: [u8; 4],
}

#[derive(Debug, Copy, Clone)]
pub struct VMState {
    command_register: u32,
    address_register: u32,
    extent_register: u32,
    attributes_register: u32,
    destination_register: u32,
    data_register: LispQ,
}

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct ProcessorState {
    pub sp: *mut LispQ,
    pub restarts_p: *mut LispQ,
    pub fp: *mut LispQ,
    pub lp: *mut LispQ,
    pub pc: PC,
    pub continuation: PC,
    pub instruction_cache: *mut InstructionCacheLine,
    pub stack_cache: *mut LispQ,
    pub stack_cache_limit: *mut LispQ,
    pub bar: [Bar; 4],
    pub list_cache_area: LispQ,
    pub list_cache_address: LispQ,
    pub structure_cache_area: LispQ,
    pub structure_cache_address: LispQ,
    pub catch_block_pointer: LispQ,
    pub control: u32,
    pub stack_cache_base: u32,
    pub array_event_count: u32,
    pub list_cache_length: u32,
    pub structure_cache_length: u32,
    pub binding_stack_pointer: u32,
    pub binding_stack_limit: u32,
    pub deep_bound_p: bool,
    pub preempt_register: u32,
    pub alu_and_rotate_control: u32,
    pub alu_op: u32,
    pub byte_size: u32,
    pub byte_rotate: u32,
    pub rotate_latch: u32,
    pub alu_verflow: bool,
    pub alu_borrow: bool,
    pub alu_less_than: bool,
    pub ephemeral_oldspace_register: u32,
    pub zone_oldspace_register: u32,
    pub control_stack_limit: u32,
    pub control_stack_extra_limit: u32,
    pub dynamic_binding_cache_base: u32,
    pub dynamic_binding_cache_mask: u32,
    pub fep_mode_trap_vector_address: u32,
    pub mapping_table_cachee: u32,
    pub mapping_table_length: u32,
    pub running: bool,
    pub instruction_count: u32,
}

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct Bar {
    pub address: LispQ,
    pub mapped: *mut LispQ,
}

pub type VMPageNumber = i32;
pub type VMPageData = [QData; VLMPAGE_SIZE_QS];
pub type VMPageTag = [QTag; VLMPAGE_SIZE_QS];
