use c2rust_bitfields::BitfieldStruct;

use std::fmt::{ Debug, Display, Formatter, Result };
use std::ops::{ Add, AddAssign, Sub, SubAssign, Neg };
use std::{ cell::RefCell, rc::Rc };

// Representation of lisp objects
use super::constants::{ QTag, CDR, VLMPAGE_SIZE_QS, ADDRESS_T, ADDRESS_NIL };

// See I-Machine specs p. 4
#[repr(C)]
#[derive(Copy, Clone)]
pub enum QList {
    Cons(u32), // Conses TODO: decline
    CompactList(u32), // Compact list TODO: decline
    Closure(u32), // Closure
    BigRatio(u32), // Big ratio
    DoubleFloat(f32), // f64 - Double precision floating point
    Complex(u32), // Complex number
    Generic(u32), // Generic function
}

impl Default for QList {
    fn default() -> Self {
        Self::BigRatio(0)
    }
}

// See I-Machine specs p. 4
#[repr(C)]
#[derive(Copy, Clone)]
pub enum QStructure {
    CompiledFunc(u32), // Compiled function
    Instance(u32), // Instances
    Symbol(u32), // Symbols
    Bignum(u32), // Bignums
}

impl Default for QStructure {
    fn default() -> Self {
        Self::Bignum(0)
    }
}

// Address space is 32 bits
pub type Address = usize;

// See I-Machine specs p. 15
#[repr(C)]
#[derive(Copy, Clone)]
pub enum QImmediate {
    Unsigned(u32), // Fixnum
    Signed(i32), // Signed fixed num
    Float(f32), // 32-bit floating point (single precision)
    // pub c: f32, // Characters
    // pub r: f32, // small ratio
    Addr(u32), // Physical address
    PackedInst(u32), // Packed instructions - TODO: bitfield
}

impl QImmediate {
    pub fn new() -> Self {
        Self::Unsigned(0)
    }

    pub fn u(self) -> Option<u32> {
        match self {
            QImmediate::Unsigned(val) => Some(val),
            _ => None,
        }
    }

    pub fn s(self) -> Option<i32> {
        match self {
            QImmediate::Signed(val) => Some(val),
            _ => None,
        }
    }

    pub fn f(self) -> Option<f32> {
        match self {
            QImmediate::Float(val) => Some(val),
            _ => None,
        }
    }

    pub fn a(self) -> Option<u32> {
        match self {
            QImmediate::Addr(val) => Some(val),
            _ => None,
        }
    }

    pub fn set_u(self, u: u32) {
        match self {
            QImmediate::Unsigned(mut val) => {
                val = u;
            }
            _ => {}
        }
    }

    pub fn set_s(self, s: i32) {
        match self {
            QImmediate::Signed(mut val) => {
                val = s;
            }
            _ => {}
        }
    }

    pub fn set_f(self, f: f32) {
        match self {
            QImmediate::Float(mut val) => {
                val = f;
            }
            _ => {}
        }
    }

    pub fn set_a(self, a: u32) {
        match self {
            QImmediate::Addr(mut val) => {
                val = a;
            }
            _ => {}
        }
    }
}

impl Default for QImmediate {
    fn default() -> Self {
        Self::Unsigned(0)
    }
}

impl PartialEq for QImmediate {
    fn eq(&self, other: &Self) -> bool {
        match self {
            QImmediate::Unsigned(val1) =>
                match other {
                    QImmediate::Unsigned(val2) => val1 == val2,
                    _ => false,
                }
            QImmediate::Signed(val1) =>
                match other {
                    QImmediate::Signed(val2) => val1 == val2,
                    _ => false,
                }
            QImmediate::Float(val1) =>
                match other {
                    QImmediate::Float(val2) => val1 == val2,
                    _ => false,
                }
            QImmediate::Float(val1) =>
                match other {
                    QImmediate::Float(val2) => val1 == val2,
                    _ => false,
                }
            _ => false,
        }
    }
}

impl Eq for QImmediate {}

impl Display for QImmediate {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            QImmediate::Unsigned(val) => write!(f, "QData u32: {}", val),
            QImmediate::Signed(val) => write!(f, "QData s32: {}", val),
            QImmediate::Float(val) => write!(f, "QData f32: {}", val),
            QImmediate::Addr(val) => write!(f, "QData address: {}", val),
            _ => write!(f, "UNKNOWN"),
        }
    }
}

impl Debug for QImmediate {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            QImmediate::Unsigned(val) => write!(f, "QData u32: {}", val),
            QImmediate::Signed(val) => write!(f, "QData s32: {}", val),
            QImmediate::Float(val) => write!(f, "QData f32: {}", val),
            QImmediate::Addr(val) => write!(f, "QData address: {}", val),
            _ => write!(f, "UNKNOWN"),
        }
    }
}

impl Add for QImmediate {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        let mut q = self;

        match self {
            QImmediate::Unsigned(mut val1) =>
                match rhs {
                    QImmediate::Unsigned(val2) => {
                        val1 += val2;
                    }
                    _ => {}
                }
            QImmediate::Signed(mut val1) =>
                match rhs {
                    QImmediate::Signed(val2) => {
                        val1 += val2;
                    }
                    _ => {}
                }
            QImmediate::Float(mut val1) =>
                match rhs {
                    QImmediate::Float(val2) => {
                        val1 += val2;
                    }
                    _ => {}
                }
            QImmediate::Addr(mut val1) =>
                match rhs {
                    QImmediate::Addr(val2) => {
                        val1 += val2;
                    }
                    _ => {}
                }
            _ => {}
        }

        return q;
    }
}

impl Sub for QImmediate {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        let mut q = self;

        match self {
            QImmediate::Unsigned(mut val1) =>
                match rhs {
                    QImmediate::Unsigned(val2) => {
                        val1 -= val2;
                    }
                    _ => {}
                }
            QImmediate::Signed(mut val1) =>
                match rhs {
                    QImmediate::Signed(val2) => {
                        val1 -= val2;
                    }
                    _ => {}
                }
            QImmediate::Float(mut val1) =>
                match rhs {
                    QImmediate::Float(val2) => {
                        val1 -= val2;
                    }
                    _ => {}
                }
            QImmediate::Addr(mut val1) =>
                match rhs {
                    QImmediate::Addr(val2) => {
                        val1 -= val2;
                    }
                    _ => {}
                }
            _ => {}
        }

        return q;
    }
}

impl AddAssign for QImmediate {
    fn add_assign(&mut self, rhs: Self) {
        match self {
            QImmediate::Unsigned(mut val1) =>
                match rhs {
                    QImmediate::Unsigned(val2) => {
                        val1 += val2;
                    }
                    _ => {}
                }
            QImmediate::Signed(mut val1) =>
                match rhs {
                    QImmediate::Signed(val2) => {
                        val1 += val2;
                    }
                    _ => {}
                }
            QImmediate::Float(mut val1) =>
                match rhs {
                    QImmediate::Float(val2) => {
                        val1 += val2;
                    }
                    _ => {}
                }
            QImmediate::Addr(mut val1) =>
                match rhs {
                    QImmediate::Addr(val2) => {
                        val1 += val2;
                    }
                    _ => {}
                }
            _ => {}
        }
    }
}

impl SubAssign for QImmediate {
    fn sub_assign(&mut self, rhs: Self) {
        match self {
            QImmediate::Unsigned(mut val1) =>
                match rhs {
                    QImmediate::Unsigned(val2) => {
                        val1 -= val2;
                    }
                    _ => {}
                }
            QImmediate::Signed(mut val1) =>
                match rhs {
                    QImmediate::Signed(val2) => {
                        val1 -= val2;
                    }
                    _ => {}
                }
            QImmediate::Float(mut val1) =>
                match rhs {
                    QImmediate::Float(val2) => {
                        val1 -= val2;
                    }
                    _ => {}
                }
            QImmediate::Addr(mut val1) =>
                match rhs {
                    QImmediate::Addr(val2) => {
                        val1 -= val2;
                    }
                    _ => {}
                }
            _ => {}
        }
    }
}

#[derive(Default, Copy, Clone, PartialEq, Eq, Debug)]
pub struct QData {
    pub i: QImmediate, // Immediate data type
    // pub l: QList,      // List objects
    // pub s: QStructure, // Structures objects
}

#[derive(Default, Copy, Clone, PartialEq, Eq, Debug)]
pub struct QCDRTagData {
    pub cdr: CDR,
    pub tag: QTag,
    pub data: QImmediate,
}

#[derive(Default, Debug, Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct VirtualAddress {
    #[bitfield(name = "offset", ty = "u32", bits = "0..=7")]
    #[bitfield(name = "page", ty = "u32", bits = "8..=31")]
    #[bitfield(name = "zone", ty = "u32", bits = "27..=31")]
    pub count_opcode: [u8; 4],
}

#[derive(Default, Debug, Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct EphemeralAddress {
    #[bitfield(name = "address", ty = "u32", bits = "0..=20")]
    #[bitfield(name = "level", ty = "u32", bits = "21..=25")]
    #[bitfield(name = "group", ty = "u32", bits = "24..=25")]
    #[bitfield(name = "half", ty = "u32", bits = "26..=26")]
    pub count_opcode: [u8; 4],
}

#[derive(Default, Debug, Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct Op {
    #[bitfield(name = "count", ty = "u32", bits = "0..=23")]
    #[bitfield(name = "opcode", ty = "u32", bits = "24..=31")]
    pub count_opcode: [u8; 4],
}

#[derive(Default, Debug, Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct C2rustUnnamed0 {
    #[bitfield(name = "data", ty = "u32", bits = "0..=15")]
    #[bitfield(name = "_pad", ty = "u32", bits = "16..=31")]
    pub data: [u8; 4],
}

#[derive(Default, Debug, Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct Fep {
    #[bitfield(name = "status", ty = "u32", bits = "0..=7")]
    #[bitfield(name = "cursor", ty = "u32", bits = "8..=8")]
    #[bitfield(name = "busy", ty = "u32", bits = "9..=9")]
    #[bitfield(name = "error", ty = "u32", bits = "10..=10")]
    #[bitfield(name = "lisp_is_loaded", ty = "u32", bits = "11..=11")]
    #[bitfield(name = "_pad", ty = "u32", bits = "12..=31")]
    pub status_cursor_busy_error_lisp_is_loaded: [u8; 4],
}

#[derive(Default, Debug, Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct GeneraVersion {
    #[bitfield(name = "minor", ty = "u32", bits = "0..=15")]
    #[bitfield(name = "major", ty = "u32", bits = "16..=31")]
    pub minor_major: [u8; 4],
}

#[derive(Default, Debug, Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct EmbDiskChannelFlags {
    #[bitfield(name = "host_byte_order", ty = "u32", bits = "0..=0")]
    #[bitfield(name = "read_only", ty = "u32", bits = "1..=1")]
    #[bitfield(name = "_pad", ty = "u32", bits = "2..=31")]
    pub host_byte_order_read_only: [u8; 4],
}

#[derive(Default, Debug, Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct EmbDiskOperation {
    #[bitfield(name = "cmd", ty = "u32", bits = "0..=2")]
    #[bitfield(name = "tagged", ty = "u32", bits = "3..=3")]
    #[bitfield(name = "buffered", ty = "u32", bits = "4..=4")]
    #[bitfield(name = "_pad1", ty = "u32", bits = "5..=7")]
    #[bitfield(name = "suppress_error_recovery", ty = "u32", bits = "8..=8")]
    #[bitfield(name = "_pad2", ty = "u32", bits = "9..=31")]
    pub cmd_tagged_buffered_suppresserrorrecovery: [u8; 4],
}

///////////////////////////////////////////////////////////////////////////////////////////////////
//
// FIXME What are those?
//
///////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Default, Debug, Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct Control {
    #[bitfield(name = "argument_size", ty = "u32", bits = "0..=7")] // 0b00000000_00000000_00000000_11111111
    #[bitfield(name = "extra_argument", ty = "u32", bits = "8..=8")] // 0b00000000_00000000_00000001_00000000
    #[bitfield(name = "caller_frame_size", ty = "u32", bits = "9..=15")] // 0b00000000_00000000_01111110_00000000
    #[bitfield(name = "_pda1", ty = "u32", bits = "16..=17")]
    #[bitfield(name = "apply", ty = "u32", bits = "18..=18")] // 0b00000000_00000010_00000000_00000000
    #[bitfield(name = "value_disposition", ty = "u32", bits = "19..=21")] // 0b00000000_00001100_00000000_00000000
    #[bitfield(name = "call_started", ty = "u32", bits = "22..=22")] // 0b00000000_00100000_00000000_00000000
    #[bitfield(name = "cleanup_bits", ty = "u32", bits = "23..=25")] // 0b00000000_11100000_00000000_00000000
    #[bitfield(name = "_pad2", ty = "u32", bits = "26..=31")]
    pub argsize_extraarg_callerframesize_apply_valuedisposition_callstarted_cleanbits: [u8; 4],
}

pub type EmbWord = i32;
pub const SIZE_EMBWORD: u32 = 4;
pub type UEmbWord = u32;
pub type EmbPtr = EmbWord;
pub type SignalMask = UEmbWord;
pub type SignalNumber = EmbWord;
pub type PtrV = *mut libc::c_void;
pub type ProcPtrV = Option<fn(PtrV) -> ()>;

#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct EmbWord16 {
    #[bitfield(name = "data", ty = "EmbWord", bits = "0..=15")]
    #[bitfield(name = "c2rust_unnamed", ty = "EmbWord", bits = "16..=31")]
    pub data_c2rust_unnamed: [u8; 4],
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct StatusCursor {
    #[bitfield(name = "status", ty = "UEmbWord", bits = "0..=7")]
    #[bitfield(name = "cursor", ty = "UEmbWord", bits = "8..=8")]
    #[bitfield(name = "busy", ty = "UEmbWord", bits = "9..=9")]
    #[bitfield(name = "error", ty = "UEmbWord", bits = "10..=10")]
    #[bitfield(name = "lisp_is_loaded", ty = "UEmbWord", bits = "11..=11")]
    #[bitfield(name = "c2rust_unnamed", ty = "UEmbWord", bits = "12..=31")]
    pub status_cursor_busy_error_lisp_is_loaded_c2rust_unnamed: [u8; 4],
}

#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct MinorMajorRelease {
    #[bitfield(name = "minorRevision", ty = "EmbWord", bits = "0..=7")]
    #[bitfield(name = "majorRevision", ty = "EmbWord", bits = "8..=15")]
    #[bitfield(name = "minorRelease", ty = "EmbWord", bits = "16..=23")]
    #[bitfield(name = "majorRelease", ty = "EmbWord", bits = "24..=30")]
    #[bitfield(name = "testReleaseP", ty = "EmbWord", bits = "31..=31")]
    pub minorrev_majorrev_minorrelease_majorrelease_testreleasep: [u8; 4],
}

#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct MinorMajor {
    #[bitfield(name = "minor", ty = "EmbWord", bits = "0..=15")]
    #[bitfield(name = "major", ty = "EmbWord", bits = "16..=31")]
    pub minor_major: [u8; 4],
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct SignalHandler {
    pub handler_thread: u64,
    pub handler_thread_setup: bool,
    pub signal: SignalMask,
    pub handler_function: ProcPtrV,
    pub handler_argument: PtrV,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct EmbCommArea {
    pub identifier: EmbWord,
    pub version: EmbWord,
    pub system_type: EmbWord,
    pub number_of_slots: EmbWord,
    pub comm_memory_size: EmbWord,
    pub genera_version: MinorMajor,
    pub osf_version: MinorMajorRelease,
    pub guest_major_version: EmbWord,
    pub guest_minor_version: EmbWord,
    pub fep_major_version: EmbWord,
    pub fep_minor_version: EmbWord,
    pub guest_buffer_start: EmbPtr,
    pub guest_buffer_size: EmbWord,
    pub host_buffer_start: EmbPtr,
    pub host_buffer_size: EmbWord,
    pub fep_buffer_start: EmbPtr,
    pub fep_buffer_size: EmbWord,
    pub guest_to_host_signals: SignalMask,
    pub live_guest_to_host_signals: SignalMask,
    pub host_to_guest_signals: SignalMask,
    pub live_host_to_guest_signals: SignalMask,
    pub channel_table: EmbPtr,
    pub console_channel: EmbPtr,
    pub cold_load_channel: EmbPtr,
    pub command_channel: EmbPtr,
    pub virtual_memory_size: EmbWord,
    pub world_image_size: EmbWord,
    pub bad_memory_map: EmbPtr,
    pub bad_memory_map_size: EmbWord,
    pub clock_signal_number: SignalNumber,
    pub clock_interval: EmbWord,
    pub run_lights: EmbWord,
    pub reset_request: EmbWord,
    pub board_serial_number: EmbWord,
    pub board_major_version: EmbWord,
    pub board_minor_version: EmbWord,
    pub spy_command: EmbWord,
    pub spy_status: EmbWord,
    pub stop_request: EmbWord,
    pub fep: StatusCursor,
    pub restart_applications: EmbWord,
    pub signal_interrupt_vector: EmbWord,
    pub base_register: EmbWord,
    pub host_version2: EmbWord,
    pub host_version3: EmbWord,
    pub mac_ivory_nvram_settings: C2rustUnnamed0,
    pub world_pathname: EmbPtr,
    pub unix_login_name: EmbPtr,
    pub unix_uid: UEmbWord,
    pub unix_gid: UEmbWord,
    pub pad0: EmbWord,
    pub pad1: [EmbWord; 15],
    pub guest_status: EmbWord,
    pub poll_thread_attrs: u64,
    pub poll_thread_attrs_setup: bool,
    pub output_thread_attrs: u64,
    pub output_thread_attrs_setup: bool,
    pub input_thread_attrs: u64,
    pub input_thread_attrs_setup: bool,
    pub use_signal_locks: bool,
    pub signal_handler: [SignalHandler; 32],
    pub reawaken: SignalMask,
    pub signal_lock: u64,
    pub signal_lock_setup: bool,
    pub signal_signal: u64,
    pub signal_signal_setup: bool,
    pub polling_thread: u64,
    pub poll_time: libc::c_long,
    pub poll_clock_time: libc::c_long,
    pub polling_thread_setup: bool,
    pub clock_thread: u64,
    pub clock_time: libc::c_long,
    pub clock_lock: u64,
    pub clock_lock_setup: bool,
    pub clock_signal: u64,
    pub clock_signal_setup: bool,
    pub clock_thread_setup: bool,
    pub reset_request_count: EmbWord,
    pub restart_applications_count: EmbWord,
    pub inhibit_disk: bool,
    pub debug_level: EmbWord,
    pub slave_trigger: u64,
    pub xlock: u64,
    pub xlock_setup: bool,
    pub wakeup_lock: u64,
    pub wakeup_lock_setup: bool,
    pub wakeup_signal: u64,
    pub wakeup_signal_setup: bool,
}
