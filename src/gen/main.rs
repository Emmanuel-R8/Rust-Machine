#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(extern_types, register_tool)]
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stdin: *mut FILE;
    static mut stderr: *mut FILE;
    fn fflush(__stream: *mut FILE) -> u32;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> u32;
    fn getline(
        __lineptr: *mut&str,
        __n: *mut size_t,
        __stream: *mut FILE,
    ) -> __ssize_t;
    fn sigemptyset(__set: *mut sigset_t) -> u32;
    fn sigaction(
        __sig: u32,
        __act: *const sigaction,
        __oact: *mut sigaction,
    ) -> u32;
    fn exit(_: u32) -> !;
    fn pthread_key_create(
        __key: *mut pthread_key_t,
        __destr_function: Option::<fn(*mut libc::c_void) -> ()>,
    ) -> u32;
    fn pthread_getspecific(__key: pthread_key_t) -> *mut libc::c_void;
    fn pthread_setspecific(
        __key: pthread_key_t,
        __pointer: *const libc::c_void,
    ) -> u32;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> u32;
    fn _exit(_: u32) -> !;
    static mut processor: *mut ProcessorState;
    fn InitializeIvoryProcessor(dataBase: *mut isize, tagsBase: *mut Tag);
    fn Runningp() -> Boole;
    fn InstructionSequencer() -> u32;
    fn VirtualMemoryWrite(vma: isize, object: *mut LispObj) -> u32;
    fn MapVirtualAddressTag(vma: isize) -> *mut Tag;
    fn MapVirtualAddressData(vma: isize) -> *mut isize;
    fn InitializeLifeSupport(config: *mut VLMConfig);
    fn TerminateLifeSupport();
    fn LoadVLMDebugger(config: *mut VLMConfig);
    fn LoadWorld(config: *mut VLMConfig) -> isize;
    static mut EmbCommAreaPtr: *mut EmbCommArea;
    fn vpunt(section:&str, format:&str, _: ...);
    fn vwarn(section:&str, format:&str, _: ...);
    fn BuildConfiguration(
        config: *mut VLMConfig,
        argc: u32,
        argv: *mut&str,
    );
}
pub type size_t = libc::c_ulong;
pub type u8 = libc::c_uchar;
pub type i32 = u32;
pub type u32 = libc::c_uint;
pub type u64 = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __pid_t = u32;
pub type __clock_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type __caddr_t =&str;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: u32,
    pub _IO_read_ptr:&str,
    pub _IO_read_end:&str,
    pub _IO_read_base:&str,
    pub _IO_write_base:&str,
    pub _IO_write_ptr:&str,
    pub _IO_write_end:&str,
    pub _IO_buf_base:&str,
    pub _IO_buf_end:&str,
    pub _IO_save_base:&str,
    pub _IO_backup_base:&str,
    pub _IO_save_end:&str,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: u32,
    pub _flags2: u32,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: u32,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub type ssize_t = __ssize_t;
pub type i32 = i32;
pub type u8 = u8;
pub type ui32 = u32;
pub type u64 = u64;
pub type caddr_t = __caddr_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [libc::c_ulong; 16],
}
pub type sigset_t = __sigset_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub union __atomic_wide_counter {
    pub __value64: libc::c_ulonglong,
    pub __value32: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub __low: libc::c_uint,
    pub __high: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_internal_list {
    pub __prev: *mut __pthread_internal_list,
    pub __next: *mut __pthread_internal_list,
}
pub type __pthread_list_t = __pthread_internal_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_mutex_s {
    pub __lock: u32,
    pub __count: libc::c_uint,
    pub __owner: u32,
    pub __nusers: libc::c_uint,
    pub __kind: u32,
    pub __spins: libc::c_short,
    pub __elision: libc::c_short,
    pub __list: __pthread_list_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_cond_s {
    pub __wseq: __atomic_wide_counter,
    pub __g1_start: __atomic_wide_counter,
    pub __g_refs: [libc::c_uint; 2],
    pub __g_size: [libc::c_uint; 2],
    pub __g1_orig_size: libc::c_uint,
    pub __wrefs: libc::c_uint,
    pub __g_signals: [libc::c_uint; 2],
}
pub type pthread_t = libc::c_ulong;
pub type pthread_key_t = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_attr_t {
    pub __size: [libc::c_char; 56],
    pub __align: libc::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [libc::c_char; 40],
    pub __align: libc::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_cond_t {
    pub __data: __pthread_cond_s,
    pub __size: [libc::c_char; 48],
    pub __align: libc::c_longlong,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union sigval {
    pub sival_int: u32,
    pub sival_ptr: *mut libc::c_void,
}
pub type __sigval_t = sigval;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct siginfo_t {
    pub si_signo: u32,
    pub si_errno: u32,
    pub si_code: u32,
    pub __pad0: u32,
    pub _sifields: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub _pad: [u32; 28],
    pub _kill: C2RustUnnamed_9,
    pub _timer: C2RustUnnamed_8,
    pub _rt: C2RustUnnamed_7,
    pub _sigchld: C2RustUnnamed_6,
    pub _sigfault: C2RustUnnamed_3,
    pub _sigpoll: C2RustUnnamed_2,
    pub _sigsys: C2RustUnnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub _call_addr: *mut libc::c_void,
    pub _syscall: u32,
    pub _arch: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub si_band: libc::c_long,
    pub si_fd: u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub si_addr: *mut libc::c_void,
    pub si_addr_lsb: libc::c_short,
    pub _bounds: C2RustUnnamed_4,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_4 {
    pub _addr_bnd: C2RustUnnamed_5,
    pub _pkey: u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub _lower: *mut libc::c_void,
    pub _upper: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
    pub si_status: u32,
    pub si_utime: __clock_t,
    pub si_stime: __clock_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
    pub si_sigval: __sigval_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
    pub si_tid: u32,
    pub si_overrun: u32,
    pub si_sigval: __sigval_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_9 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
}
pub type __sighandler_t = Option::<fn(u32) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sigaction {
    pub __sigaction_handler: C2RustUnnamed_10,
    pub sa_mask: __sigset_t,
    pub sa_flags: u32,
    pub sa_restorer: Option::<fn() -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_10 {
    pub sa_handler: __sighandler_t,
    pub sa_sigaction: Option::<
        fn(u32, *mut siginfo_t, *mut libc::c_void) -> (),
    >,
}
pub type ptrdiff_t = libc::c_long;
pub type sa_handler_t = Option::<fn(u32) -> ()>;
pub type EmbWord = i32;
pub type uEmbWord = ui32;
pub type EmbPtr = EmbWord;
pub type SignalMask = uEmbWord;
pub type SignalNumber = EmbWord;
pub type PtrV = *mut libc::c_void;
pub type ProcPtrV = Option::<fn(PtrV) -> ()>;
pub type isize = u64;
pub type Boole = u8;
pub type Tag = u8;
#[derive(Copy, Clone)]
#[repr(C)]
pub union LispObj {
    pub parts: _LispObj,
    pub whole: u64,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _LispObj {
    pub tag: ui32,
    pub data: C2RustUnnamed_11,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_11 {
    pub u: ui32,
    pub s: i32,
    pub f: libc::c_float,
}
pub type PC = LispObj;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _InstructionCacheLine {
    pub pc: PC,
    pub next_pc: PC,
    pub code: u32,
    pub operand: u32,
    pub instruction: libc::c_uint,
    pub next_cp: *mut _InstructionCacheLine,
}
pub type InstructionCacheLine = _InstructionCacheLine;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _ProcessorState {
    pub sp: *mut LispObj,
    pub restartsp: *mut LispObj,
    pub fp: *mut LispObj,
    pub lp: *mut LispObj,
    pub pc: PC,
    pub continuation: PC,
    pub InstructionCache: *mut InstructionCacheLine,
    pub StackCache: *mut LispObj,
    pub StackCacheLimit: *mut LispObj,
    pub bar: [_bar; 4],
    pub ListCacheArea: LispObj,
    pub ListCacheAddress: LispObj,
    pub StructureCacheArea: LispObj,
    pub StructureCacheAddress: LispObj,
    pub CatchBlockPointer: LispObj,
    pub control: isize,
    pub StackCacheBase: isize,
    pub ArrayEventCount: isize,
    pub ListCacheLength: isize,
    pub StructureCacheLength: isize,
    pub BindingStackPointer: isize,
    pub BindingStackLimit: isize,
    pub DeepBoundP: Boole,
    pub PreemptRegister: isize,
    pub AluAndRotateControl: isize,
    pub AluOp: Option::<fn() -> isize>,
    pub ByteSize: isize,
    pub ByteRotate: isize,
    pub RotateLatch: isize,
    pub ALUOverflow: Boole,
    pub ALUBorrow: Boole,
    pub ALULessThan: Boole,
    pub EphemeralOldspaceRegister: isize,
    pub ZoneOldspaceRegister: isize,
    pub ControlStackLimit: isize,
    pub ControlStackExtraLimit: isize,
    pub DynamicBindingCacheBase: isize,
    pub DynamicBindingCacheMask: isize,
    pub FEPModeTrapVectorAddress: isize,
    pub MappingTableCache: isize,
    pub MappingTableLength: isize,
    pub running: Boole,
    pub instruction_count: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _bar {
    pub address: LispObj,
    pub mapped: *mut LispObj,
}
pub type ProcessorState = _ProcessorState;
pub type in_addr_t = ui32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct in_addr {
    pub s_addr: in_addr_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XParams {
    pub xpHostName:&str,
    pub xpHostAddress: libc::c_long,
    pub xpDisplay: u32,
    pub xpScreen: u32,
    pub xpInitialState: u32,
    pub xpGeometry:&str,
    pub xpForegroundColor:&str,
    pub xpBackgroundColor:&str,
    pub xpBorderColor:&str,
    pub xpBorderWidth: u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct NetworkInterface {
    pub present: Boole,
    pub device: [libc::c_char; 257],
    pub myProtocol: libc::c_ushort,
    pub myAddress: in_addr,
    pub myOptions: [libc::c_char; 257],
    pub anotherAddress: *mut NetworkInterface,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TraceConfig {
    pub traceP: Boole,
    pub tracePOST: Boole,
    pub bufferSize: u32,
    pub startPC: libc::c_uint,
    pub stopPC: libc::c_uint,
    pub outputFile:&str,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VLMConfig {
    pub enableSpy: Boole,
    pub tracing: TraceConfig,
    pub commAreaSize: size_t,
    pub hostBufferSpace: size_t,
    pub guestBufferSpace: size_t,
    pub vlmDebuggerPath: [libc::c_char; 257],
    pub worldPath: [libc::c_char; 257],
    pub worldSearchPath:&str,
    pub enableIDS: Boole,
    pub virtualMemory: size_t,
    pub coldLoadXParams: XParams,
    pub generaXParams: XParams,
    pub diagnosticIPAddress: in_addr,
    pub interfaces: [NetworkInterface; 8],
    pub testFunction: Boole,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SignalHandler {
    pub handlerThread: pthread_t,
    pub handlerThreadSetup: Boole,
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
    pub generaVersion: C2RustUnnamed_15,
    pub osfVersion: C2RustUnnamed_14,
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
    pub fep: C2RustUnnamed_13,
    pub restart_applications: EmbWord,
    pub signal_interrupt_vector: EmbWord,
    pub base_register: EmbWord,
    pub hostVersion2: EmbWord,
    pub hostVersion3: EmbWord,
    pub MacIvory_NVRAM_settings: C2RustUnnamed_12,
    pub worldPathname: EmbPtr,
    pub unixLoginName: EmbPtr,
    pub unixUID: uEmbWord,
    pub unixGID: uEmbWord,
    pub pad0: EmbWord,
    pub pad1: [EmbWord; 15],
    pub guestStatus: EmbWord,
    pub pollThreadAttrs: pthread_attr_t,
    pub pollThreadAttrsSetup: Boole,
    pub outputThreadAttrs: pthread_attr_t,
    pub outputThreadAttrsSetup: Boole,
    pub inputThreadAttrs: pthread_attr_t,
    pub inputThreadAttrsSetup: Boole,
    pub useSignalLocks: Boole,
    pub signalHandler: [SignalHandler; 32],
    pub reawaken: SignalMask,
    pub signalLock: pthread_mutex_t,
    pub signalLockSetup: Boole,
    pub signalSignal: pthread_cond_t,
    pub signalSignalSetup: Boole,
    pub pollingThread: pthread_t,
    pub pollTime: libc::c_long,
    pub pollClockTime: libc::c_long,
    pub pollingThreadSetup: Boole,
    pub clockThread: pthread_t,
    pub clockTime: libc::c_long,
    pub clockLock: pthread_mutex_t,
    pub clockLockSetup: Boole,
    pub clockSignal: pthread_cond_t,
    pub clockSignalSetup: Boole,
    pub clockThreadSetup: Boole,
    pub resetRequestCount: EmbWord,
    pub restartApplicationsCount: EmbWord,
    pub inhibitDisk: Boole,
    pub debugLevel: EmbWord,
    pub slaveTrigger: caddr_t,
    pub XLock: pthread_mutex_t,
    pub XLockSetup: Boole,
    pub wakeupLock: pthread_mutex_t,
    pub wakeupLockSetup: Boole,
    pub wakeupSignal: pthread_cond_t,
    pub wakeupSignalSetup: Boole,
}

pub type GuestStatus = u32;
pub const RunningGuestStatus: GuestStatus = 5;
pub const CrashedGuestStatus: GuestStatus = 4;
pub const StartedGuestStatus: GuestStatus = 3;
pub const InitializedGuestStatus: GuestStatus = 2;
pub const InitializingGuestStatus: GuestStatus = 1;
pub const UninitializedGuestStatus: GuestStatus = 0;
pub const BrokenGuestStatus: GuestStatus = -1;
pub const NonexistentGuestStatus: GuestStatus = -2;
pub type SystemCommAreaSlot = libc::c_uint;
pub const busMode: SystemCommAreaSlot = 59;
pub const lispReleaseString: SystemCommAreaSlot = 58;
pub const packageNameTable: SystemCommAreaSlot = 57;
pub const floatOperationStatus: SystemCommAreaSlot = 56;
pub const floatOperatingMode: SystemCommAreaSlot = 55;
pub const bindingStackLow: SystemCommAreaSlot = 54;
pub const controlStackLow: SystemCommAreaSlot = 53;
pub const inhibitSchedulingFlag: SystemCommAreaSlot = 52;
pub const currentStackGroupStatusBits: SystemCommAreaSlot = 51;
pub const stackGroupLock: SystemCommAreaSlot = 50;
pub const currentStackGroup: SystemCommAreaSlot = 49;
pub const initialStackGroup: SystemCommAreaSlot = 48;
pub const sysoutParentTimestamp2: SystemCommAreaSlot = 47;
pub const sysoutParentTimestamp1: SystemCommAreaSlot = 46;
pub const sysoutTimestamp2: SystemCommAreaSlot = 45;
pub const sysoutTimestamp1: SystemCommAreaSlot = 44;
pub const sysoutGenerationNumber: SystemCommAreaSlot = 43;
pub const enableSysoutAtColdBoot: SystemCommAreaSlot = 42;
pub const wiredVirtualAddressHigh: SystemCommAreaSlot = 41;
pub const wiredPhysicalAddressHigh: SystemCommAreaSlot = 40;
pub const flushableQueueModified: SystemCommAreaSlot = 39;
pub const flushableQueueTail: SystemCommAreaSlot = 38;
pub const flushableQueueHead: SystemCommAreaSlot = 37;
pub const storageColdBoot: SystemCommAreaSlot = 36;
pub const mmpt1: SystemCommAreaSlot = 35;
pub const phtCollisionCounts: SystemCommAreaSlot = 34;
pub const sysoutBitmaps: SystemCommAreaSlot = 33;
pub const swapMapDPN: SystemCommAreaSlot = 32;
pub const swapMap: SystemCommAreaSlot = 31;
pub const loadMapDPN: SystemCommAreaSlot = 30;
pub const loadMap: SystemCommAreaSlot = 29;
pub const loadBitmaps: SystemCommAreaSlot = 28;
pub const smpt: SystemCommAreaSlot = 27;
pub const mmpt: SystemCommAreaSlot = 26;
pub const mmptY: SystemCommAreaSlot = 25;
pub const pht: SystemCommAreaSlot = 24;
pub const defaultConsArea: SystemCommAreaSlot = 23;
pub const listCacheRegion: SystemCommAreaSlot = 22;
pub const structureCacheRegion: SystemCommAreaSlot = 21;
pub const pageConsAlarm: SystemCommAreaSlot = 20;
pub const regionConsAlarm: SystemCommAreaSlot = 19;
pub const regionFreePointerBeforeFlip: SystemCommAreaSlot = 18;
pub const regionCreatedPages: SystemCommAreaSlot = 17;
pub const regionArea: SystemCommAreaSlot = 16;
pub const regionListThread: SystemCommAreaSlot = 15;
pub const regionBits: SystemCommAreaSlot = 14;
pub const regionGCPointer: SystemCommAreaSlot = 13;
pub const regionFreePointer: SystemCommAreaSlot = 12;
pub const regionQuantumLength: SystemCommAreaSlot = 11;
pub const regionQuantumOrigin: SystemCommAreaSlot = 10;
pub const areaRegionBits: SystemCommAreaSlot = 9;
pub const areaRegionList: SystemCommAreaSlot = 8;
pub const areaRegionQuantumSize: SystemCommAreaSlot = 7;
pub const areaMaximumQuantumSize: SystemCommAreaSlot = 6;
pub const areaName: SystemCommAreaSlot = 5;
pub const oblastFreeSize: SystemCommAreaSlot = 4;
pub const addressSpaceMapAddress: SystemCommAreaSlot = 3;
pub const systemStartup: SystemCommAreaSlot = 2;
pub const syscomMinorVersionNumber: SystemCommAreaSlot = 1;
pub const syscomMajorVersionNumber: SystemCommAreaSlot = 0;
#[no_mangle]
pub static mut Trace: Boole = 0 as usize as Boole;
#[no_mangle]
pub static mut EnableIDS: Boole = 0 as usize as Boole;
#[no_mangle]
pub static mut TestFunction: Boole = 0 as usize as Boole;
static mut mainThread: pthread_key_t = 0;
 fn MaybeTerminateVLM(mut signal: u32) {
    let mut answer: &str =  "" ;
    let mut answerSize: size_t = 0 as usize as size_t;
    let mut answerSize_p: *mut size_t = &mut answerSize;
    let mut nRead: ssize_t = 0;
    if (pthread_getspecific(mainThread)).is_null() {
        return;
    }
    if (*EmbCommAreaPtr).guestStatus > StartedGuestStatus as usize {
        if RunningGuestStatus as usize == (*EmbCommAreaPtr).guestStatus {
            fprintf(
                stderr,
                b"\nLisp is running!\n\n\0" as *const u8 as *const libc::c_char,
            );
        } else {
            fprintf(
                stderr,
                b"\nLisp was running!\n\n\0" as *const u8 as *const libc::c_char,
            );
        }
        fprintf(
            stderr,
            b"If you exit, the current state of Lisp will be lost.\n\0" as *const u8
                as *const libc::c_char,
        );
        fprintf(
            stderr,
            b"All information in its memory image (e.g., any modified editor\n\0"
                as *const u8 as *const libc::c_char,
        );
        fprintf(
            stderr,
            b"buffers) will be irretrievably lost.  Further, Lisp will abandon\n\0"
                as *const u8 as *const libc::c_char,
        );
        fprintf(
            stderr,
            b"any tasks it is performing for its clients.\n\n\0" as *const u8
                as *const libc::c_char,
        );
        fprintf(
            stderr,
            b"Do you still wish to exit?  (yes or no) \0" as *const u8
                as *const libc::c_char,
        );
        fflush(stderr);
        loop {
            nRead = getline(&mut answer, answerSize_p, stdin);
            if nRead < 0 as usize as libc::c_long {
                vpunt(
                     "" ,
                    b"Unexpected EOF on standard input\0" as *const u8
                        as *const libc::c_char as&str,
                );
            }
            *answer
                .offset(
                    (nRead - 1 as usize as libc::c_long) as isize,
                ) = '\0' as i32 as libc::c_char;
            if 0
                == strcmp(answer, b"yes\0" as *const u8 as *const libc::c_char)
            {
                break;
            }
            if 0
                == strcmp(answer, b"no\0" as *const u8 as *const libc::c_char)
            {
                return
            } else {
                fprintf(
                    stderr,
                    b"Please answer 'yes' or 'no'.  \0" as *const u8
                        as *const libc::c_char,
                );
                fflush(stderr);
            }
        }
    }
    TerminateLifeSupport();
    _exit(0);
}
unsafe fn main_0(
    mut argc: u32,
    mut argv: *mut&str,
) -> usize {
    let mut config: VLMConfig = VLMConfig {
        enableSpy: 0,
        tracing: TraceConfig {
            traceP: 0,
            tracePOST: 0,
            bufferSize: 0,
            startPC: 0,
            stopPC: 0,
            outputFile:  "" ,
        },
        commAreaSize: 0,
        hostBufferSpace: 0,
        guestBufferSpace: 0,
        vlmDebuggerPath: [0; 257],
        worldPath: [0; 257],
        worldSearchPath:  "" ,
        enableIDS: 0,
        virtualMemory: 0,
        coldLoadXParams: XParams {
            xpHostName:  "" ,
            xpHostAddress: 0,
            xpDisplay: 0,
            xpScreen: 0,
            xpInitialState: 0,
            xpGeometry:  "" ,
            xpForegroundColor:  "" ,
            xpBackgroundColor:  "" ,
            xpBorderColor:  "" ,
            xpBorderWidth: 0,
        },
        generaXParams: XParams {
            xpHostName:  "" ,
            xpHostAddress: 0,
            xpDisplay: 0,
            xpScreen: 0,
            xpInitialState: 0,
            xpGeometry:  "" ,
            xpForegroundColor:  "" ,
            xpBackgroundColor:  "" ,
            xpBorderColor:  "" ,
            xpBorderWidth: 0,
        },
        diagnosticIPAddress: in_addr { s_addr: 0 },
        interfaces: [NetworkInterface {
            present: 0,
            device: [0; 257],
            myProtocol: 0,
            myAddress: in_addr { s_addr: 0 },
            myOptions: [0; 257],
            anotherAddress: 0 as *mut NetworkInterface,
        }; 8],
        testFunction: 0,
    };
    let mut sigAction: sigaction = sigaction {
        __sigaction_handler: C2RustUnnamed_10 {
            sa_handler: None,
        },
        sa_mask: __sigset_t { __val: [0; 16] },
        sa_flags: 0,
        sa_restorer: None,
    };
    let mut worldImageSize: isize = 0;
    let mut worldImageMB: isize = 0;
    let mut message: &str =  "" ;
    let mut reason: usize = 0;
    BuildConfiguration(&mut config, argc, argv);
    EnableIDS = config.enableIDS;
    TestFunction = config.testFunction;
    Trace = config.tracing.tracePOST;
    InitializeIvoryProcessor(
        MapVirtualAddressData(0 as usize as isize),
        MapVirtualAddressTag(0 as usize as isize),
    );
    InitializeLifeSupport(&mut config);
    if pthread_key_create(&mut mainThread, None) != 0 {
        vpunt(
             "" ,
            b"Unable to establish per-thread data.\0" as *const u8 as *const libc::c_char
                as&str,
        );
    }
    pthread_setspecific(mainThread, 1 as usize as *mut libc::c_void);
    sigAction
        .__sigaction_handler
        .sa_handler = ::std::mem::transmute::<
        Option::<fn(u32) -> ()>,
        sa_handler_t,
    >(Some(MaybeTerminateVLM as fn(u32) -> ()));
    sigemptyset(&mut sigAction.sa_mask);
    sigAction.sa_flags = 0;
    if sigaction(2, &mut sigAction, 0 as *mut sigaction) != 0 {
        vpunt(
             "" ,
            b"Unable to establish SIGINT handler.\0" as *const u8 as *const libc::c_char
                as&str,
        );
    }
    if sigaction(15, &mut sigAction, 0 as *mut sigaction) != 0 {
        vpunt(
             "" ,
            b"Unable to establish SIGTERM handler.\0" as *const u8 as *const libc::c_char
                as&str,
        );
    }
    if sigaction(1, &mut sigAction, 0 as *mut sigaction) != 0 {
        vpunt(
             "" ,
            b"Unable to establish SIGHUP handler.\0" as *const u8 as *const libc::c_char
                as&str,
        );
    }
    if sigaction(3, &mut sigAction, 0 as *mut sigaction) != 0 {
        vpunt(
             "" ,
            b"Unable to establish SIGQUIT handler.\0" as *const u8 as *const libc::c_char
                as&str,
        );
    }
    worldImageSize = LoadWorld(&mut config);
    LoadVLMDebugger(&mut config);
    worldImageMB = (5 as usize as libc::c_ulong)
        .wrapping_mul(worldImageSize)
        .wrapping_add((1024 as usize * 1024) as libc::c_ulong)
        .wrapping_sub(1 as usize as libc::c_ulong)
        .wrapping_div((1024 as usize * 1024) as libc::c_ulong);
    if worldImageMB > config.virtualMemory {
        vpunt(
             "" ,
            b"World file %s won't fit within the requested virtual memory (%dMB)\0"
                as *const u8 as *const libc::c_char as&str,
            (config.worldPath).as_mut_ptr(),
            config.virtualMemory,
        );
    }
    if (2 as usize as libc::c_ulong).wrapping_mul(worldImageMB)
        > config.virtualMemory
    {
        vwarn(
             "" ,
            b"Only %dMB of virtual memory unused after loading world file %s\n\0"
                as *const u8 as *const libc::c_char as&str,
            (config.virtualMemory).wrapping_sub(worldImageMB),
            (config.worldPath).as_mut_ptr(),
        );
    }
    VirtualMemoryWrite(
        (0xf8041100 as libc::c_long as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<EmbWord>() as libc::c_ulong)
            .wrapping_add(
                enableSysoutAtColdBoot as usize as ptrdiff_t as libc::c_ulong,
            ),
        if EnableIDS as usize != 0 {
            0xf8041208 as libc::c_uint as *mut LispObj
        } else {
            0xf8041200 as libc::c_uint as *mut LispObj
        },
    );
    (*EmbCommAreaPtr)
        .virtualMemorySize = (config.virtualMemory)
        .wrapping_mul(1024 as usize as libc::c_ulong)
        .wrapping_mul(1024 as usize as libc::c_ulong)
        .wrapping_add(4 as usize as libc::c_ulong)
        .wrapping_div(5 as usize as libc::c_ulong) as EmbWord;
    (*EmbCommAreaPtr).worldImageSize = worldImageSize as EmbWord;
    while Runningp() != 0 {
        reason = InstructionSequencer();
        if reason != 0 {
            match reason {
                1 => {
                    message = b"Unimplemented instruction\0" as *const u8
                        as *const libc::c_char as&str;
                }
                2 => {
                    message =  "" ;
                }
                3 => {
                    message =  "" ;
                }
                4 => {
                    message = b"Stack overflow while not in emulator mode\0" as *const u8
                        as *const libc::c_char as&str;
                }
                5 => {
                    message = b"Illegal trap vector contents\0" as *const u8
                        as *const libc::c_char as&str;
                }
                _ => {
                    message = b"Halted for unknown reason\0" as *const u8
                        as *const libc::c_char as&str;
                }
            }
            if !message.is_null() {
                vwarn(
                     "" ,
                    b"%s at PC %016x (%s)\0" as *const u8 as *const libc::c_char
                        as&str,
                    message,
                    (*processor).pc.whole >> 1,
                    if (*processor).pc.whole & 1 as usize as libc::c_ulong != 0 {
                        b"Odd\0" as *const u8 as *const libc::c_char
                    } else {
                        b"Even\0" as *const u8 as *const libc::c_char
                    },
                );
            }
        }
        if 2 as usize == reason {
            break;
        }
    }
    exit(0);
}
pub fn main() {
    let mut args: Vec::<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            (::std::ffi::CString::new(arg))
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::std::ptr::null_mut());
    unsafe {
        ::std::process::exit(
            main_0(
                (args.len() - 1),
                args.as_mut_ptr() as *mut&str,
            ) as i32,
        )
    }
}
