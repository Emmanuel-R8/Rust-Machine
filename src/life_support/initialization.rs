#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn uname(__name: *mut utsname) -> u32;
    fn sprintf(_:&str, _: *const libc::c_char, _: ...) -> u32;
    fn MakeLispObj(tag: ui32, data: ui32) -> *mut LispObj;
    fn pthread_create(
        __newthread: *mut pthread_t,
        __attr: *const pthread_attr_t,
        __start_routine: Option::<
            fn(*mut libc::c_void) -> *mut libc::c_void,
        >,
        __arg: *mut libc::c_void,
    ) -> u32;
    fn pthread_join(
        __th: pthread_t,
        __thread_return: *mut *mut libc::c_void,
    ) -> u32;
    fn pthread_self() -> pthread_t;
    fn pthread_attr_init(__attr: *mut pthread_attr_t) -> u32;
    fn pthread_attr_destroy(__attr: *mut pthread_attr_t) -> u32;
    fn pthread_attr_getstacksize(
        __attr: *const pthread_attr_t,
        __stacksize: *mut size_t,
    ) -> u32;
    fn pthread_attr_setstacksize(
        __attr: *mut pthread_attr_t,
        __stacksize: size_t,
    ) -> u32;
    fn pthread_cancel(__th: pthread_t) -> u32;
    fn pthread_mutex_init(
        __mutex: *mut pthread_mutex_t,
        __mutexattr: *const pthread_mutexattr_t,
    ) -> u32;
    fn pthread_mutex_destroy(__mutex: *mut pthread_mutex_t) -> u32;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> u32;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> u32;
    fn pthread_cond_init(
        __cond: *mut pthread_cond_t,
        __cond_attr: *const pthread_condattr_t,
    ) -> u32;
    fn pthread_cond_destroy(__cond: *mut pthread_cond_t) -> u32;
    fn pthread_key_create(
        __key: *mut pthread_key_t,
        __destr_function: Option::<fn(*mut libc::c_void) -> ()>,
    ) -> u32;
    fn atexit(__func: Option::<fn() -> ()>) -> u32;
    fn exit(_: u32) -> !;
    fn strtoul(
        _: *const libc::c_char,
        _: *mut&str,
        _: u32,
    ) -> libc::c_ulong;
    fn pthread_getspecific(__key: pthread_key_t) -> *mut libc::c_void;
    fn pthread_setspecific(
        __key: pthread_key_t,
        __pointer: *const libc::c_void,
    ) -> u32;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn getuid() -> __uid_t;
    fn pthread_delay_np(interval: *const timespec) -> u32;
    fn VirtualMemoryWriteBlockConstant(
        vma: isize,
        object: *mut LispObj,
        count: u32,
        increment: u32,
    ) -> u32;
    fn VirtualMemoryWrite(vma: isize, object: *mut LispObj) -> u32;
    fn MapVirtualAddressData(vma: isize) -> *mut isize;
    fn EnsureVirtualAddressRange(
        virtualaddress: isize,
        count: u32,
        faultp: Boole,
    ) -> isize;
    fn getgid() -> __gid_t;
    fn getlogin() ->&str;
    fn InitializeColdLoadChannel(config: *mut VLMConfig);
    fn TerminateColdLoadChannel();
    fn InitializeConsoleChannel(config: *mut VLMConfig);
    fn TerminateConsoleChannel();
    fn InitializeNetworkChannels(config: *mut VLMConfig);
    fn InitializeMessageChannels(config: *mut VLMConfig);
    fn IntervalTimerDriver(argument: pthread_addr_t);
    fn IvoryLifePolling(argument: pthread_addr_t);
    fn TerminateNetworkChannels();
    fn TerminateMessageChannels();
    fn TerminateDiskChannels();
    fn TerminateSignalHandlers();
    fn InitializeSignalHandlers();
    fn vpunt(section:&str, format:&str, _: ...);
}
pub type u8 = libc::c_uchar;
pub type i32 = u32;
pub type u32 = libc::c_uint;
pub type u64 = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __time_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type __caddr_t =&str;
pub type C2RustUnnamed = libc::c_uint;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct utsname {
    pub sysname: [libc::c_char; 65],
    pub nodename: [libc::c_char; 65],
    pub release: [libc::c_char; 65],
    pub version: [libc::c_char; 65],
    pub machine: [libc::c_char; 65],
    pub __domainname: [libc::c_char; 65],
}
pub type ptrdiff_t = libc::c_long;
pub type size_t = libc::c_ulong;
pub type i32 = i32;
pub type u8 = u8;
pub type ui32 = u32;
pub type u64 = u64;
pub type EmbWord = i32;
pub type uEmbWord = ui32;
pub type EmbPtr = EmbWord;
pub type SignalMask = uEmbWord;
pub type SignalNumber = EmbWord;
pub type PtrV = *mut libc::c_void;
pub type ProcPtrV = Option::<fn(PtrV) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BootCommArea {
    pub embCommArea: EmbWord,
    pub systemType: EmbWord,
    pub stackBase: EmbWord,
    pub stackSize: EmbWord,
    pub spyPC: EmbWord,
    pub spyCommandAddress: EmbWord,
    pub spyStatusAddress: EmbWord,
    pub spyBlockAddress: EmbWord,
    pub crashAddress: EmbWord,
    pub crashActionAddress: EmbWord,
    pub bootPROMVersion: EmbWord,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BootDataArea {
    pub bootSpyCommand: EmbWord,
    pub bootSpyStatus: EmbWord,
    pub crashAction: EmbWord,
    pub crashType: EmbWord,
    pub crashWord1: C2RustUnnamed_2,
    pub crashWord2: C2RustUnnamed_1,
    pub crashWord3: C2RustUnnamed_0,
    pub bootFEPKernelDPN: EmbWord,
    pub bootDevicePROMVersion: EmbWord,
    pub bootColorStartupFileDPN: EmbWord,
    pub bootSelectedConsoleType: EmbWord,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub crashFatalFEPVector: EmbWord,
    pub crashTrapArgs: EmbWord,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub crashFatalVMA: EmbWord,
    pub crashTrapPC: EmbWord,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_2 {
    pub crashFatalPC: EmbWord,
    pub crashTrapNumber: EmbWord,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FEPCommArea {
    pub fepVersionNumber: EmbWord,
    pub systemType: EmbWord,
    pub fepStartup: EmbWord,
    pub embCommunicationArea: EmbWord,
    pub memorySegmentFreeList: EmbWord,
    pub unallocatedPhysicalMemory: EmbWord,
    pub phtSize: EmbWord,
    pub phtCollisionCountsBase: EmbWord,
    pub phtCollisionCount: EmbWord,
    pub phtRehashes: EmbWord,
    pub unmappedMemoryBase: EmbWord,
    pub allocatePhyiscalMemoryAtAddress: EmbWord,
    pub allocatePhysicalMemory: EmbWord,
    pub deallocatePhysicalMemory: EmbWord,
    pub romPHTLookup: EmbWord,
    pub romPHTPut: EmbWord,
    pub romPHTRemove: EmbWord,
    pub romPHTRehash: EmbWord,
    pub romError: EmbWord,
    pub clearMapCache: EmbWord,
    pub localIPAddress0: EmbWord,
    pub diagnosticIPAddress: EmbWord,
    pub romMBINGetReceiveBuffer: EmbWord,
    pub romMBINReturnReceiveBuffer: EmbWord,
    pub romMBINGetTransmitBuffer: EmbWord,
    pub romMBINSendTransmitBuffer: EmbWord,
    pub initializeInteractor: EmbWord,
    pub localIPAddress1: EmbWord,
    pub localIPSubnetMask0: EmbWord,
    pub localIPSubnetMask1: EmbWord,
    pub gatewayIPAddress0: EmbWord,
    pub gatewayIPAddress1: EmbWord,
    pub loadServerIPAddress: EmbWord,
    pub hardwareECORegisters: EmbWord,
    pub ethernetDriver0: EmbWord,
    pub ethernetDriver1: EmbWord,
    pub romUpdateRendezvousParameters: EmbWord,
}
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
    pub data: C2RustUnnamed_3,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_3 {
    pub u: ui32,
    pub s: i32,
    pub f: libc::c_float,
}
pub type SystemCommArea = [EmbWord; 60];
pub type caddr_t = __caddr_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union __atomic_wide_counter {
    pub __value64: libc::c_ulonglong,
    pub __value32: C2RustUnnamed_4,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
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
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutexattr_t {
    pub __size: [libc::c_char; 4],
    pub __align: u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_condattr_t {
    pub __size: [libc::c_char; 4],
    pub __align: u32,
}
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
pub type pthread_addr_t = *mut libc::c_void;
pub type pthread_startroutine_t = Option::<
    fn(*mut libc::c_void) -> *mut libc::c_void,
>;
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
    pub generaVersion: C2RustUnnamed_8,
    pub osfVersion: C2RustUnnamed_7,
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
    pub fep: C2RustUnnamed_6,
    pub restart_applications: EmbWord,
    pub signal_interrupt_vector: EmbWord,
    pub base_register: EmbWord,
    pub hostVersion2: EmbWord,
    pub hostVersion3: EmbWord,
    pub MacIvory_NVRAM_settings: C2RustUnnamed_5,
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

pub type system_type = libc::c_uint;
pub const SystemTypeVLM: system_type = 526;
pub const SystemTypeNXP1000: system_type = 525;
pub const SystemTypeMacIvory3: system_type = 524;
pub const SystemTypeUX1200G: system_type = 523;
pub const SystemTypeUX1200S: system_type = 522;
pub const SystemTypeXL1200: system_type = 521;
pub const SystemTypeUX400S: system_type = 519;
pub const SystemTypeMacIvory2: system_type = 518;
pub const SystemTypeMacIvory1: system_type = 518;
pub const SystemTypeXL400: system_type = 516;
pub const SystemTypeUX400G: system_type = 515;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct EmbString {
    pub length: EmbWord,
    pub string: EmbWord,
}
#[no_mangle]
pub static mut BootCommAreaPtr: *mut BootCommArea = 0 as *const BootCommArea
    as *mut BootCommArea;
#[no_mangle]
pub static mut BootDataAreaPtr: *mut BootDataArea = 0 as *const BootDataArea
    as *mut BootDataArea;
#[no_mangle]
pub static mut FEPCommAreaPtr: *mut FEPCommArea = 0 as *const FEPCommArea
    as *mut FEPCommArea;
#[no_mangle]
pub static mut SystemCommAreaPtr: *mut SystemCommArea = 0 as *const SystemCommArea
    as *mut SystemCommArea;
#[no_mangle]
pub static mut EmbCommAreaPtr: *mut EmbCommArea = 0 as *const EmbCommArea
    as *mut EmbCommArea;
#[no_mangle]
pub static mut EmbCommAreaAllocPtr: EmbPtr = -(1);
#[no_mangle]
pub static mut EmbCommAreaAllocSize: size_t = 0 as usize as size_t;
#[no_mangle]
pub static mut dataAddress: caddr_t = 0 as *const libc::c_char as caddr_t;
#[no_mangle]
pub static mut tagsAddress: caddr_t = 0 as *const libc::c_char as caddr_t;
#[no_mangle]
pub static mut areasSize: size_t = 0 as usize as size_t;
static mut mainThread: pthread_key_t = 0;
#[no_mangle]
pub  fn EmbCommAreaAlloc(mut nBytes: size_t) -> EmbPtr {
    let mut nWords: size_t = nBytes
        .wrapping_add(::std::mem::size_of::<EmbWord>() as libc::c_ulong)
        .wrapping_sub(1 as usize as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<EmbWord>() as libc::c_ulong);
    let mut thePtr: EmbPtr = EmbCommAreaAllocPtr;
    if nWords & 1 as usize as libc::c_ulong != 0 {
        nWords = nWords.wrapping_add(1);
    }
    if nWords > EmbCommAreaAllocSize || nBytes <= 0 as usize as libc::c_ulong {
        vpunt(
             "" ,
            b"Couldn't allocate %d words in the embedded communications area\0"
                as *const u8 as *const libc::c_char as&str,
            nWords,
        );
    }
    EmbCommAreaAllocSize = (EmbCommAreaAllocSize as libc::c_ulong).wrapping_sub(nWords)
        as size_t as size_t;
    EmbCommAreaAllocPtr = (EmbCommAreaAllocPtr as libc::c_ulong).wrapping_add(nWords)
        as EmbPtr as EmbPtr;
    return thePtr;
}
#[no_mangle]
pub  fn MakeEmbString(mut aString:&str) -> EmbPtr {
    let mut theStringPtr: EmbPtr = 0;
    let mut theString: *mut EmbString = 0 as *mut EmbString;
    let mut nBytes: size_t = if aString.is_null() {
        0 as usize as libc::c_ulong
    } else {
        strlen(aString)
    };
    let mut datum: ui32 = 0;
    if 0 as usize as libc::c_ulong == nBytes {
        return -(1);
    }
    theStringPtr = EmbCommAreaAlloc(
        (::std::mem::size_of::<EmbString>() as libc::c_ulong).wrapping_add(nBytes),
    );
    theString = &mut *(EmbCommAreaPtr as *mut EmbWord).offset(theStringPtr as isize)
        as *mut EmbWord as PtrV as *mut EmbString;
    (*theString).length = nBytes as EmbWord;
    memcpy(
        &mut (*theString).string as *mut EmbWord as&str
            as *mut libc::c_void,
        aString as *const libc::c_void,
        nBytes,
    );
    return theStringPtr;
}
 fn ParseVersionNumber(
    mut versionString:&str,
    mut majorVersion: *mut u32,
    mut minorVersion: *mut u32,
) {
    let mut start: &str =  "" ;
    let mut end: &str =  "" ;
    let mut major: usize = 0;
    let mut minor: usize = -(1);
    *minorVersion = -(1);
    *majorVersion = *minorVersion;
    start = versionString;
    major = strtoul(start, &mut end, 10);
    if start == end {
        return;
    }
    if *end != 0 {
        if *end as usize == '.' as i32 {
            start = end.offset(1 as usize as isize);
            minor = strtoul(start, &mut end, 0);
            if start == end || *end as usize != 0 {
                return;
            }
        } else {
            return
        }
    }
    *majorVersion = major;
    *minorVersion = minor;
}
#[no_mangle]
pub  fn InitializeLifeSupport(mut config: *mut VLMConfig) {
    let mut osfName: utsname = utsname {
        sysname: [0; 65],
        nodename: [0; 65],
        release: [0; 65],
        version: [0; 65],
        machine: [0; 65],
        __domainname: [0; 65],
    };
    let mut worldPathname: [libc::c_char; 262] = [0; 262];
    let mut loginName: &str =  "" ;
    let mut identifier: &str =  "" ;
    let mut major: usize = 0;
    let mut minor: usize = 0;
    EnsureVirtualAddressRange(
        0xfffe0000 as libc::c_long as isize,
        ((64 as usize + 64) as libc::c_ulong)
            .wrapping_add((*config).commAreaSize),
        0 as usize as Boole,
    );
    BootCommAreaPtr = MapVirtualAddressData(0xfffe0000 as libc::c_long as isize)
        as *mut BootCommArea;
    BootDataAreaPtr = MapVirtualAddressData(0xfffe0040 as libc::c_long as isize)
        as *mut BootDataArea;
    EmbCommAreaPtr = MapVirtualAddressData(0xfffe0080 as libc::c_long as isize)
        as *mut EmbCommArea;
    VirtualMemoryWriteBlockConstant(
        0xfffe0000 as libc::c_long as isize,
        MakeLispObj(
            0 as usize as ui32,
            0xfffe0000 as libc::c_long as ui32,
        ),
        64 as usize + 64,
        1,
    );
    let mut lispDatum: LispObj = LispObj {
        parts: _LispObj {
            tag: 0,
            data: C2RustUnnamed_3 { u: 0 },
        },
    };
    lispDatum.parts.data.u = 0xfffe0080 as libc::c_long as isize as ui32;
    lispDatum.parts.tag = 25 as usize as Tag as ui32;
    VirtualMemoryWrite(
        (0xfffe0000 as libc::c_long as libc::c_ulong)
            .wrapping_add(
                (0 as libc::c_ulong)
                    .wrapping_div(::std::mem::size_of::<EmbWord>() as libc::c_ulong),
            ),
        &mut lispDatum,
    );
    let mut lispDatum_0: LispObj = LispObj {
        parts: _LispObj {
            tag: 0,
            data: C2RustUnnamed_3 { u: 0 },
        },
    };
    lispDatum_0.parts.data.u = SystemTypeVLM as usize as isize as ui32;
    lispDatum_0.parts.tag = 8 as usize as Tag as ui32;
    VirtualMemoryWrite(
        (0xfffe0000 as libc::c_long as libc::c_ulong)
            .wrapping_add(
                (4 as libc::c_ulong)
                    .wrapping_div(::std::mem::size_of::<EmbWord>() as libc::c_ulong),
            ),
        &mut lispDatum_0,
    );
    let mut lispDatum_1: LispObj = LispObj {
        parts: _LispObj {
            tag: 0,
            data: C2RustUnnamed_3 { u: 0 },
        },
    };
    lispDatum_1.parts.data.u = 0xf8000100 as libc::c_long as isize as ui32;
    lispDatum_1.parts.tag = 25 as usize as Tag as ui32;
    VirtualMemoryWrite(
        (0xfffe0000 as libc::c_long as libc::c_ulong)
            .wrapping_add(
                (8 as libc::c_ulong)
                    .wrapping_div(::std::mem::size_of::<EmbWord>() as libc::c_ulong),
            ),
        &mut lispDatum_1,
    );
    let mut lispDatum_2: LispObj = LispObj {
        parts: _LispObj {
            tag: 0,
            data: C2RustUnnamed_3 { u: 0 },
        },
    };
    lispDatum_2.parts.data.u = 0xf00 as libc::c_long as isize as ui32;
    lispDatum_2.parts.tag = 8 as usize as Tag as ui32;
    VirtualMemoryWrite(
        (0xfffe0000 as libc::c_long as libc::c_ulong)
            .wrapping_add(
                (12 as libc::c_ulong)
                    .wrapping_div(::std::mem::size_of::<EmbWord>() as libc::c_ulong),
            ),
        &mut lispDatum_2,
    );
    let mut lispDatum_3: LispObj = LispObj {
        parts: _LispObj {
            tag: 0,
            data: C2RustUnnamed_3 { u: 0 },
        },
    };
    lispDatum_3.parts.data.u = 0xfffe0040 as libc::c_long as isize as ui32;
    lispDatum_3.parts.tag = 25 as usize as Tag as ui32;
    VirtualMemoryWrite(
        (0xfffe0000 as libc::c_long as libc::c_ulong)
            .wrapping_add(
                (28 as libc::c_ulong)
                    .wrapping_div(::std::mem::size_of::<EmbWord>() as libc::c_ulong),
            ),
        &mut lispDatum_3,
    );
    let mut lispDatum_4: LispObj = LispObj {
        parts: _LispObj {
            tag: 0,
            data: C2RustUnnamed_3 { u: 0 },
        },
    };
    lispDatum_4
        .parts
        .data
        .u = (0xfffe0040 as libc::c_long as libc::c_ulong)
        .wrapping_add(
            (0 as libc::c_ulong)
                .wrapping_div(::std::mem::size_of::<EmbWord>() as libc::c_ulong),
        )
        .wrapping_add(46 as usize as libc::c_ulong) as ui32;
    lispDatum_4.parts.tag = 25 as usize as Tag as ui32;
    VirtualMemoryWrite(
        (0xfffe0000 as libc::c_long as libc::c_ulong)
            .wrapping_add(
                (20 as libc::c_ulong)
                    .wrapping_div(::std::mem::size_of::<EmbWord>() as libc::c_ulong),
            ),
        &mut lispDatum_4,
    );
    let mut lispDatum_5: LispObj = LispObj {
        parts: _LispObj {
            tag: 0,
            data: C2RustUnnamed_3 { u: 0 },
        },
    };
    lispDatum_5
        .parts
        .data
        .u = (0xfffe0040 as libc::c_long as libc::c_ulong)
        .wrapping_add(
            (4 as libc::c_ulong)
                .wrapping_div(::std::mem::size_of::<EmbWord>() as libc::c_ulong),
        )
        .wrapping_add(46 as usize as libc::c_ulong) as ui32;
    lispDatum_5.parts.tag = 25 as usize as Tag as ui32;
    VirtualMemoryWrite(
        (0xfffe0000 as libc::c_long as libc::c_ulong)
            .wrapping_add(
                (24 as libc::c_ulong)
                    .wrapping_div(::std::mem::size_of::<EmbWord>() as libc::c_ulong),
            ),
        &mut lispDatum_5,
    );
    EnsureVirtualAddressRange(
        0xf8041000 as libc::c_long as isize,
        256,
        0 as usize as Boole,
    );
    VirtualMemoryWriteBlockConstant(
        0xf8041000 as libc::c_long as isize,
        MakeLispObj(
            0 as usize as ui32,
            0xf8041000 as libc::c_long as ui32,
        ),
        256,
        1,
    );
    FEPCommAreaPtr = MapVirtualAddressData(0xf8041000 as libc::c_long as isize)
        as *mut FEPCommArea;
    EnsureVirtualAddressRange(
        0xf8041100 as libc::c_long as isize,
        256,
        0 as usize as Boole,
    );
    VirtualMemoryWriteBlockConstant(
        0xf8041100 as libc::c_long as isize,
        MakeLispObj(
            0 as usize as ui32,
            0xf8041100 as libc::c_long as ui32,
        ),
        256,
        1,
    );
    SystemCommAreaPtr = MapVirtualAddressData(0xf8041100 as libc::c_long as isize)
        as *mut SystemCommArea;
    VirtualMemoryWriteBlockConstant(
        0xfffe0080 as libc::c_long as isize,
        MakeLispObj(8 as usize as ui32, 0 as usize as ui32),
        (*config).commAreaSize,
        0,
    );
    identifier = b"EMBD\0" as *const u8 as *const libc::c_char as&str;
    (*EmbCommAreaPtr).identifier = *(identifier as *mut EmbWord);
    (*EmbCommAreaPtr).version = 1;
    (*EmbCommAreaPtr).system_type = SystemTypeVLM;
    (*EmbCommAreaPtr)
        .number_of_slots = ((&mut (*EmbCommAreaPtr).pad0 as *mut EmbWord as ptrdiff_t
        - EmbCommAreaPtr as ptrdiff_t) as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<EmbWord>() as libc::c_ulong) as EmbWord;
    (*EmbCommAreaPtr).comm_memory_size = (*config).commAreaSize as EmbWord;
    let ref mut fresh0 = (*EmbCommAreaPtr).generaVersion;
    (*fresh0).set_major(9);
    let ref mut fresh1 = (*EmbCommAreaPtr).generaVersion;
    (*fresh1).set_minor(0);
    if uname(&mut osfName) < 0 as usize {
        let ref mut fresh2 = (*EmbCommAreaPtr).osfVersion;
        (*fresh2).set_majorRelease(0);
    } else {
        let ref mut fresh3 = (*EmbCommAreaPtr).osfVersion;
        (*fresh3).set_testReleaseP(0);
        if *(*__ctype_b_loc())
            .offset(osfName.release[0 as usize as usize] as usize as isize)
            as usize & _ISdigit as usize as libc::c_ushort
            != 0
        {
            ParseVersionNumber((osfName.release).as_mut_ptr(), &mut major, &mut minor);
        } else {
            let ref mut fresh4 = (*EmbCommAreaPtr).osfVersion;
            (*fresh4)
                .set_testReleaseP(
                    (osfName.release[0 as usize as usize]
                        != 'V' as i32),
                );
            ParseVersionNumber(
                &mut *(osfName.release).as_mut_ptr().offset(1 as usize as isize),
                &mut major,
                &mut minor,
            );
        }
        let ref mut fresh5 = (*EmbCommAreaPtr).osfVersion;
        (*fresh5).set_majorRelease(major);
        let ref mut fresh6 = (*EmbCommAreaPtr).osfVersion;
        (*fresh6).set_minorRelease(minor);
        ParseVersionNumber((osfName.version).as_mut_ptr(), &mut major, &mut minor);
        let ref mut fresh7 = (*EmbCommAreaPtr).osfVersion;
        (*fresh7).set_majorRevision(major);
        let ref mut fresh8 = (*EmbCommAreaPtr).osfVersion;
        (*fresh8).set_minorRevision(minor);
    }
    (*EmbCommAreaPtr).channel_table = -(1);
    (*EmbCommAreaPtr).consoleChannel = -(1);
    (*EmbCommAreaPtr).cold_load_channel = -(1);
    (*EmbCommAreaPtr).command_channel = -(1);
    (*EmbCommAreaPtr).clock_signal = -(1);
    let ref mut fresh9 = (*EmbCommAreaPtr).slaveTrigger;
    *fresh9 = 0 as caddr_t;
    InitializeSignalHandlers();
    if pthread_key_create(&mut mainThread, None) != 0 {
        vpunt(
             "" ,
            b"Unable to establish per-thread data.\0" as *const u8 as *const libc::c_char
                as&str,
        );
    }
    pthread_setspecific(mainThread, 1 as usize as *mut libc::c_void);
    if atexit(Some(TerminateLifeSupport as fn() -> ())) != 0 {
        vpunt(
             "" ,
            b"Unable to establish cleanup handler for Life Support\0" as *const u8
                as *const libc::c_char as&str,
        );
    }
    SetupThreadAttrs(
        b"polling\0" as *const u8 as *const libc::c_char as&str,
        0,
        &mut (*EmbCommAreaPtr).pollThreadAttrs,
        &mut (*EmbCommAreaPtr).pollThreadAttrsSetup,
    );
    SetupThreadAttrs(
        b"output\0" as *const u8 as *const libc::c_char as&str,
        2,
        &mut (*EmbCommAreaPtr).outputThreadAttrs,
        &mut (*EmbCommAreaPtr).outputThreadAttrsSetup,
    );
    SetupThreadAttrs(
        b"input\0" as *const u8 as *const libc::c_char as&str,
        3,
        &mut (*EmbCommAreaPtr).inputThreadAttrs,
        &mut (*EmbCommAreaPtr).inputThreadAttrsSetup,
    );
    if pthread_mutex_init(
        &mut (*EmbCommAreaPtr).signalLock,
        0 as *const pthread_mutexattr_t,
    ) != 0
    {
        vpunt(
             "" ,
            b"Unable to create the Life Support signal lock\0" as *const u8
                as *const libc::c_char as&str,
        );
    }
    (*EmbCommAreaPtr).signalLockSetup = 1 as usize as Boole;
    if pthread_cond_init(
        &mut (*EmbCommAreaPtr).signalSignal,
        0 as *const pthread_condattr_t,
    ) != 0
    {
        vpunt(
             "" ,
            b"Unable to create the Life Support signal signal\0" as *const u8
                as *const libc::c_char as&str,
        );
    }
    (*EmbCommAreaPtr).signalSignalSetup = 1 as usize as Boole;
    if pthread_mutex_lock(&mut (*EmbCommAreaPtr).signalLock) != 0 {
        vpunt(
             "" ,
            b"Unable to lock the Life Support signal lock in thread %lx\0" as *const u8
                as *const libc::c_char as&str,
            pthread_self(),
        );
    }
    if pthread_create(
        &mut (*EmbCommAreaPtr).pollingThread,
        &mut (*EmbCommAreaPtr).pollThreadAttrs,
        ::std::mem::transmute::<
            Option::<fn(pthread_addr_t) -> ()>,
            pthread_startroutine_t,
        >(Some(IvoryLifePolling as fn(pthread_addr_t) -> ())),
        0 as *mut libc::c_void,
    ) != 0
    {
        vpunt(
             "" ,
            b"Unable to create the Life Support polling thread\0" as *const u8
                as *const libc::c_char as&str,
        );
    }
    (*EmbCommAreaPtr).pollingThreadSetup = 1 as usize as Boole;
    if pthread_mutex_init(
        &mut (*EmbCommAreaPtr).clockLock,
        0 as *const pthread_mutexattr_t,
    ) != 0
    {
        vpunt(
             "" ,
            b"Unable to create the Life Support clock lock\0" as *const u8
                as *const libc::c_char as&str,
        );
    }
    (*EmbCommAreaPtr).clockLockSetup = 1 as usize as Boole;
    if pthread_cond_init(
        &mut (*EmbCommAreaPtr).clockSignal,
        0 as *const pthread_condattr_t,
    ) != 0
    {
        vpunt(
             "" ,
            b"Unable to create the Life Support clock signal\0" as *const u8
                as *const libc::c_char as&str,
        );
    }
    (*EmbCommAreaPtr).clockSignalSetup = 1 as usize as Boole;
    if pthread_create(
        &mut (*EmbCommAreaPtr).clockThread,
        &mut (*EmbCommAreaPtr).pollThreadAttrs,
        ::std::mem::transmute::<
            Option::<fn(pthread_addr_t) -> ()>,
            pthread_startroutine_t,
        >(Some(IntervalTimerDriver as fn(pthread_addr_t) -> ())),
        0 as *mut libc::c_void,
    ) != 0
    {
        vpunt(
             "" ,
            b"Unable to create the Life Support interval timer thread\0" as *const u8
                as *const libc::c_char as&str,
        );
    }
    (*EmbCommAreaPtr).clockThreadSetup = 1 as usize as Boole;
    if pthread_mutex_init(&mut (*EmbCommAreaPtr).XLock, 0 as *const pthread_mutexattr_t)
        != 0
    {
        vpunt(
             "" ,
            b"Unable to create the Life Support X library lock\0" as *const u8
                as *const libc::c_char as&str,
        );
    }
    (*EmbCommAreaPtr).XLockSetup = 1 as usize as Boole;
    if pthread_mutex_init(
        &mut (*EmbCommAreaPtr).wakeupLock,
        0 as *const pthread_mutexattr_t,
    ) != 0
    {
        vpunt(
             "" ,
            b"Unable to create the VLM wakeup lock\0" as *const u8 as *const libc::c_char
                as&str,
        );
    }
    (*EmbCommAreaPtr).wakeupLockSetup = 1 as usize as Boole;
    if pthread_cond_init(
        &mut (*EmbCommAreaPtr).wakeupSignal,
        0 as *const pthread_condattr_t,
    ) != 0
    {
        vpunt(
             "" ,
            b"Unable to create the VLM wakeup signal\0" as *const u8
                as *const libc::c_char as&str,
        );
    }
    (*EmbCommAreaPtr).wakeupSignalSetup = 1 as usize as Boole;
    EmbCommAreaAllocPtr = (::std::mem::size_of::<EmbCommArea>() as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<EmbWord>() as libc::c_ulong) as EmbPtr;
    EmbCommAreaAllocSize = ((*EmbCommAreaPtr).comm_memory_size - EmbCommAreaAllocPtr)
        as size_t;
    if (*config).worldPath[0 as usize as usize] != 0 {
        sprintf(
            worldPathname.as_mut_ptr(),
            b"HOST:%s\0" as *const u8 as *const libc::c_char,
            ((*config).worldPath).as_mut_ptr(),
        );
    } else {
        worldPathname[0 as usize as usize] = 0 as usize as libc::c_char;
    }
    (*EmbCommAreaPtr).worldPathname = MakeEmbString(worldPathname.as_mut_ptr());
    loginName = getlogin();
    if !loginName.is_null() {
        (*EmbCommAreaPtr).unixLoginName = MakeEmbString(loginName);
    } else {
        (*EmbCommAreaPtr).unixLoginName = -(1);
    }
    (*EmbCommAreaPtr).unixUID = getuid();
    (*EmbCommAreaPtr).unixGID = getgid();
    InitializeColdLoadChannel(config);
    InitializeConsoleChannel(config);
    InitializeMessageChannels(config);
    InitializeNetworkChannels(config);
    (*EmbCommAreaPtr).host_buffer_start = EmbCommAreaAllocPtr;
    (*EmbCommAreaPtr).host_buffer_size = (*config).hostBufferSpace as EmbWord;
    (*EmbCommAreaPtr)
        .fep_buffer_start = EmbCommAreaAllocPtr + (*EmbCommAreaPtr).host_buffer_size;
    (*EmbCommAreaPtr).fep_buffer_size = 512;
    (*EmbCommAreaPtr)
        .guest_buffer_start = EmbCommAreaAllocPtr + (*EmbCommAreaPtr).host_buffer_size
        + (*EmbCommAreaPtr).fep_buffer_size;
    (*EmbCommAreaPtr)
        .guest_buffer_size = EmbCommAreaAllocSize
        .wrapping_sub((*EmbCommAreaPtr).host_buffer_size as libc::c_ulong)
        .wrapping_sub((*EmbCommAreaPtr).fep_buffer_size as libc::c_ulong) as EmbWord;
    if ((*EmbCommAreaPtr).guest_buffer_size as libc::c_ulong)
        < (*config).guestBufferSpace
    {
        vpunt(
             "" ,
            b"Couldn't allocate %d words for guest buffers in the communcations area; only %d words are available.\0"
                as *const u8 as *const libc::c_char as&str,
            (*config).guestBufferSpace,
            (*EmbCommAreaPtr).guest_buffer_size,
        );
    }
    (*EmbCommAreaPtr).useSignalLocks = 1 as usize as Boole;
    if pthread_mutex_unlock(&mut (*EmbCommAreaPtr).signalLock) != 0 {
        vpunt(
             "" ,
            b"Unable to unlock the Life Support signal lock in thread %lx\0" as *const u8
                as *const libc::c_char as&str,
            pthread_self(),
        );
    }
}
#[no_mangle]
pub  fn TerminateLifeSupport() {
    let mut killSleep: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
    let mut exit_code: *mut libc::c_void = 0 as *mut libc::c_void;
    if (pthread_getspecific(mainThread)).is_null() {
        return;
    }
    TerminateSignalHandlers();
    TerminateColdLoadChannel();
    TerminateConsoleChannel();
    TerminateDiskChannels();
    TerminateMessageChannels();
    exit(1);
}
 fn SetupThreadAttrs(
    mut thread_class:&str,
    mut priorityBoost: u32,
    mut threadAttrs: *mut pthread_attr_t,
    mut threadAttrsSetup: *mut Boole,
) {
    let mut stackSize: size_t = 0;
    let mut priority: usize = 0;
    if pthread_attr_init(threadAttrs) != 0 {
        vpunt(
             "" ,
            b"Unable to create attributes for Life Support %s threads\0" as *const u8
                as *const libc::c_char as&str,
            thread_class,
        );
    }
    *threadAttrsSetup = 1 as usize as Boole;
    pthread_attr_getstacksize(threadAttrs, &mut stackSize);
    if pthread_attr_setstacksize(
        threadAttrs,
        (4 as usize as libc::c_ulong).wrapping_mul(stackSize),
    ) != 0
    {
        vpunt(
             "" ,
            b"Unable to set stack size attribute for Life Support %s threads to %d bytes\0"
                as *const u8 as *const libc::c_char as&str,
            thread_class,
            (4 as usize as libc::c_ulong).wrapping_mul(stackSize),
        );
    }
}
