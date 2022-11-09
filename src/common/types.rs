use c2rust_bitfields::BitfieldStruct;

use std::fmt::{Debug, Display, Formatter, Result};
use std::ops::{Add, AddAssign, Sub, SubAssign};
use std::{cell::RefCell, rc::Rc};

// Representation of lisp objects
use super::constants::{QTag, CDR, VLMPAGE_SIZE_QS};

// See I-Machine specs p. 4
#[repr(C)]
#[derive(Copy, Clone)]
pub union QList {
    pub c: u32,  // Conses TODO: decline
    pub l: u32,  // Compact list TODO: decline
    pub cl: u32, // Closure
    pub r: u32,  // Big ratio
    pub f: u32,  // f64 - Double precision floating point
    pub cx: u32, // Complex number
    pub g: u32,  // Generic function
}

impl Default for QList {
    fn default() -> Self {
        Self { r: 0 }
    }
}

// See I-Machine specs p. 4
#[repr(C)]
#[derive(Copy, Clone)]
pub union QStructure {
    pub c: u32, // Compiled function
    pub i: u32, // Instances
    pub s: u32, // Symbols
    pub b: u32, // Bignums
}

impl Default for QStructure {
    fn default() -> Self {
        Self { b: 0 }
    }
}

// See I-Machine specs p. 15
#[repr(C)]
#[derive(Copy, Clone)]
pub union QImmediate {
    pub u: u32, // Fixnum
    pub s: i32, // Signed fixed num
    pub f: f32, // 32-bit floating point (single precision)
    // pub c: f32, // Characters
    // pub r: f32, // small ratio
    pub a: u32, // Physical address
                // pub p: u32, // Packed instructions - TODO: bitfield
}

impl QImmediate {
    pub fn new() -> Self {
        Self { u: 0 }
    }
}

impl Default for QImmediate {
    fn default() -> Self {
        Self { u: 0 }
    }
}

impl PartialEq for QImmediate {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            match self {
                QImmediate { u: val1 } => match other {
                    QImmediate { u: val2 } => val1 == val2,
                    _ => false,
                },
                QImmediate { s: val1 } => match other {
                    QImmediate { s: val2 } => val1 == val2,
                    _ => false,
                },
                QImmediate { f: val1 } => match other {
                    QImmediate { f: val2 } => val1 == val2,
                    _ => false,
                },
                QImmediate { a: val1 } => match other {
                    QImmediate { a: val2 } => val1 == val2,
                    _ => false,
                },
            }
        }
    }
}

impl Eq for QImmediate {}

impl Display for QImmediate {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        unsafe {
            match self {
                QImmediate { u: val } => write!(f, "QData u32: {}", val),
                QImmediate { s: val } => write!(f, "QData i32: {}", val),
                QImmediate { f: val } => write!(f, "QData f32: {}", val),
                QImmediate { a: val } => write!(f, "QData address: {}", val),
            }
        }
    }
}

impl Debug for QImmediate {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        unsafe {
            match self {
                QImmediate { u: val } => write!(f, "QData u32: {}", val),
                QImmediate { s: val } => write!(f, "QData i32: {}", val),
                QImmediate { f: val } => write!(f, "QData f32: {}", val),
                QImmediate { a: val } => write!(f, "QData address: {}", val),
            }
        }
    }
}

impl Add for QImmediate {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        let mut q = self;

        unsafe {
            match self {
                QImmediate { u: val } => q.u += rhs.u,
                QImmediate { s: val } => q.s += rhs.s,
                QImmediate { f: val } => q.f += rhs.f,
                QImmediate { a: val } => q.a += rhs.a,
            }
        }

        return q;
    }
}

impl Sub for QImmediate {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        let mut q = self;

        unsafe {
            match self {
                QImmediate { u: val } => q.u -= rhs.u,
                QImmediate { s: val } => q.s -= rhs.s,
                QImmediate { f: val } => q.f -= rhs.f,
                QImmediate { a: val } => q.a -= rhs.a,
            }
        }

        return q;
    }
}

impl AddAssign for QImmediate {
    fn add_assign(&mut self, rhs: Self) {
        unsafe {
            match self {
                QImmediate { u: val } => self.u += rhs.u,
                QImmediate { s: val } => self.s += rhs.s,
                QImmediate { f: val } => self.f += rhs.f,
                QImmediate { a: val } => self.a += rhs.a,
            }
        }
    }
}

impl SubAssign for QImmediate {
    fn sub_assign(&mut self, rhs: Self) {
        unsafe {
            match self {
                QImmediate { u: val } => self.u -= rhs.u,
                QImmediate { s: val } => self.s -= rhs.s,
                QImmediate { f: val } => self.f -= rhs.f,
                QImmediate { a: val } => self.a -= rhs.a,
            }
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

#[repr(C)]
#[derive(Copy)]
pub union QWord {
    pub whole: u64,
    pub parts: QCDRTagData,
}

impl Default for QWord {
    fn default() -> Self {
        Self { whole: 0 }
    }
}

impl Clone for QWord {
    fn clone(&self) -> Self {
        unsafe { QWord { whole: self.whole } } // easier to copy the u64
    }
}

impl PartialEq for QWord {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            match self {
                QWord { whole: val1 } => match other {
                    QWord { whole: val2 } => val1 == val2,
                    QWord { parts: val2 } => false,
                },
                QWord { parts: val1 } => match other {
                    QWord { whole: val2 } => false,
                    QWord { parts: val2 } => val1 == val2,
                },
            }
        }
    }
}

impl Eq for QWord {}

impl Display for QWord {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        unsafe {
            match self {
                QWord { whole: val } => write!(f, "QWord u64: {}", val),
                QWord { parts: val } => write!(
                    f,
                    "QWord cdr: {}, tag: {}, data: {}",
                    val.cdr, val.tag, val.data
                ),
            }
        }
    }
}

impl Debug for QWord {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        unsafe {
            match self {
                QWord { whole: val } => write!(f, "QWord u64: {}", val),
                QWord { parts: val } => write!(
                    f,
                    "QWord cdr: {}, tag: {}, data: {}",
                    val.cdr, val.tag, val.data
                ),
            }
        }
    }
}

impl Add for QWord {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        let mut q = self;
        unsafe {
            q.parts.data += rhs.parts.data;
        }

        return q;
    }
}

impl Sub for QWord {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        let mut q = self;
        unsafe { q.parts.data -= rhs.parts.data };

        return q;
    }
}

impl AddAssign for QWord {
    fn add_assign(&mut self, rhs: Self) {
        unsafe { self.parts.data += rhs.parts.data };
    }
}

impl SubAssign for QWord {
    fn sub_assign(&mut self, rhs: Self) {
        unsafe { self.parts.data -= rhs.parts.data };
    }
}

impl QWord {
    pub fn u(self) -> u32 {
        return unsafe { self.parts.data.u };
    }
    pub fn s(self) -> i32 {
        return unsafe { self.parts.data.s };
    }
    pub fn f(self) -> f32 {
        return unsafe { self.parts.data.f };
    }
    pub fn a(self) -> u32 {
        return unsafe { self.parts.data.a };
    }

    pub fn inc(self) -> Self {
        let mut q = self.clone();
        unsafe { q.parts.data.a += 1 };

        return q;
    }

    pub fn dec(self) -> Self {
        let mut q = self.clone();
        unsafe { q.parts.data.a -= 1 };

        return q;
    }

    pub fn inc_mut(&mut self) {
        unsafe { self.parts.data.a += 1 };
    }

    pub fn dec_mut(&mut self) {
        unsafe { self.parts.data.a -= 1 };
    }
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
pub struct C2RustUnnamed_0 {
    #[bitfield(name = "data", ty = "u32", bits = "0..=15")]
    #[bitfield(name = "padding", ty = "u32", bits = "16..=31")]
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
    #[bitfield(name = "padding", ty = "u32", bits = "12..=31")]
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
    #[bitfield(name = "padding", ty = "u32", bits = "2..=31")]
    pub host_byte_order_read_only: [u8; 4],
}

#[derive(Default, Debug, Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct EmbDiskOperation {
    #[bitfield(name = "cmd", ty = "u32", bits = "0..=2")]
    #[bitfield(name = "tagged", ty = "u32", bits = "3..=3")]
    #[bitfield(name = "buffered", ty = "u32", bits = "4..=4")]
    #[bitfield(name = "pad1", ty = "u32", bits = "5..=7")]
    #[bitfield(name = "suppress_error_recovery", ty = "u32", bits = "8..=8")]
    #[bitfield(name = "pad2", ty = "u32", bits = "9..=31")]
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
    #[bitfield(name = "with_gap2", ty = "u32", bits = "16..=17")]
    #[bitfield(name = "apply", ty = "u32", bits = "18..=18")] // 0b00000000_00000010_00000000_00000000
    #[bitfield(name = "value_disposition", ty = "u32", bits = "19..=21")] // 0b00000000_00001100_00000000_00000000
    #[bitfield(name = "call_started", ty = "u32", bits = "22..=22")] // 0b00000000_00100000_00000000_00000000
    #[bitfield(name = "cleanup_bits", ty = "u32", bits = "23..=25")] // 0b00000000_11100000_00000000_00000000
    #[bitfield(name = "with_stub", ty = "u32", bits = "26..=31")]
    pub argsize_extraarg_callerframesize_apply_valuedisposition_callstarted_cleanbits: [u8; 4],
}

////
///
/// Instruction cache line
///
#[derive(Debug)]
#[repr(C)]
pub struct InstructionCacheLine {
    pub pc: QWord,
    pub next_pc: QWord,
    pub code: u32,
    pub operand: u32,
    pub instruction: u64,
    pub next_cp: Rc<RefCell<InstructionCacheLine>>,
}

impl Default for InstructionCacheLine {
    fn default() -> Self {
        InstructionCacheLine {
            pc: QWord::default(),
            next_pc: QWord::default(),
            code: 0,
            operand: 0,
            instruction: 0,
            next_cp: Rc::default(),
        }
    }
}

impl Clone for InstructionCacheLine {
    fn clone(&self) -> Self {
        InstructionCacheLine {
            pc: self.pc.clone(),
            next_pc: self.next_pc.clone(),
            code: self.code,
            operand: self.operand,
            instruction: self.instruction,
            next_cp: Rc::new(RefCell::new(self.clone())),
        }
    }
}

pub const INSTRUCTION_CACHE_SIZE: u32 = 0x800;
pub const INSTRUCTION_CACHE_LINE_SIZE: u32 = 0x40;

///
///
/// VLM Page bases
///
#[derive(Default, Debug, Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct VLMPageBases {
    #[bitfield(name = "dataPageBase", ty = "u32", bits = "0..=27")]
    #[bitfield(name = "tagsPageBase", ty = "u32", bits = "28..=31")]
    pub dataPageBase_tagsPageBase: [u8; 4],
}

#[derive(Default, Debug, Copy, Clone)]
pub struct VMState {
    command_register: u32,
    address_register: u32,
    extent_register: u32,
    attributes_register: u32,
    destination_register: u32,
    data_register: QWord,
}

#[derive(Default, Debug, Copy, Clone)]
#[repr(C)]
pub struct Bar {
    pub address: QWord,
    pub mapped: QWord,
}

pub type VMPageNumber = i32;
pub type VMPageData = [QImmediate; VLMPAGE_SIZE_QS as usize];
pub type VMPageTag = [QTag; VLMPAGE_SIZE_QS as usize];

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
    pub minorRevision_majorRevision_minorRelease_majorRelease_testReleaseP: [u8; 4],
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
    pub handlerThread: u64,
    pub handlerThreadSetup: bool,
    pub signal: SignalMask,
    pub handlerFunction: ProcPtrV,
    pub handlerArgument: PtrV,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct EmbCommArea {
    pub identifier: EmbWord,
    pub version: EmbWord,
    pub system_type: EmbWord,
    pub number_of_slots: EmbWord,
    pub comm_memory_size: EmbWord,
    pub generaVersion: MinorMajor,
    pub osfVersion: MinorMajorRelease,
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
    pub consoleChannel: EmbPtr,
    pub cold_load_channel: EmbPtr,
    pub command_channel: EmbPtr,
    pub virtualMemorySize: EmbWord,
    pub worldImageSize: EmbWord,
    pub bad_memory_map: EmbPtr,
    pub bad_memory_map_size: EmbWord,
    pub clock_signal: SignalNumber,
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
    pub hostVersion2: EmbWord,
    pub hostVersion3: EmbWord,
    pub MacIvory_NVRAM_settings: C2RustUnnamed_0,
    pub worldPathname: EmbPtr,
    pub unixLoginName: EmbPtr,
    pub unixUID: UEmbWord,
    pub unixGID: UEmbWord,
    pub pad0: EmbWord,
    pub pad1: [EmbWord; 15],
    pub guestStatus: EmbWord,
    pub pollThreadAttrs: u64,
    pub pollThreadAttrsSetup: bool,
    pub outputThreadAttrs: u64,
    pub outputThreadAttrsSetup: bool,
    pub inputThreadAttrs: u64,
    pub inputThreadAttrsSetup: bool,
    pub useSignalLocks: bool,
    pub signalHandler: [SignalHandler; 32],
    pub reawaken: SignalMask,
    pub signalLock: u64,
    pub signalLockSetup: bool,
    pub signalSignal: u64,
    pub signalSignalSetup: bool,
    pub pollingThread: u64,
    pub pollTime: libc::c_long,
    pub pollClockTime: libc::c_long,
    pub pollingThreadSetup: bool,
    pub clockThread: u64,
    pub clockTime: libc::c_long,
    pub clockLock: u64,
    pub clockLockSetup: bool,
    pub clockSignal: u64,
    pub clockSignalSetup: bool,
    pub clockThreadSetup: bool,
    pub resetRequestCount: EmbWord,
    pub restartApplicationsCount: EmbWord,
    pub inhibitDisk: bool,
    pub debugLevel: EmbWord,
    pub slaveTrigger: u64,
    pub XLock: u64,
    pub XLockSetup: bool,
    pub wakeupLock: u64,
    pub wakeupLockSetup: bool,
    pub wakeupSignal: u64,
    pub wakeupSignalSetup: bool,
}
