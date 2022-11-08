#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(extern_types, register_tool)]
extern "C" {
    pub type sockaddr_x25;
    pub type sockaddr_un;
    pub type sockaddr_ns;
    pub type sockaddr_iso;
    pub type sockaddr_ipx;
    pub type sockaddr_inarp;
    pub type sockaddr_eon;
    pub type sockaddr_dl;
    pub type sockaddr_ax25;
    pub type sockaddr_at;
    fn close(__fd: u32) -> u32;
    fn free(_: *mut libc::c_void);
    fn atexit(__func: Option::<fn() -> ()>) -> u32;
    fn pthread_create(
        __newthread: *mut u64,
        __attr: *const u64,
        __start_routine: Option::<
            fn(*mut libc::c_void) -> *mut libc::c_void,
        >,
        __arg: *mut libc::c_void,
    ) -> u32;
    fn pthread_join(
        __th: u64,
        __thread_return: *mut *mut libc::c_void,
    ) -> u32;
    fn pthread_detach(__th: u64) -> u32;
    fn pthread_self() -> u64;
    fn pthread_cancel(__th: u64) -> u32;
    fn u64estcancel();
    fn __pthread_register_cancel(__buf: *mut __pthread_unwind_buf_t);
    fn __pthread_unregister_cancel(__buf: *mut __pthread_unwind_buf_t);
    fn __pthread_unwind_next(__buf: *mut __pthread_unwind_buf_t) -> !;
    fn __sigsetjmp(__env: *mut __jmp_buf_tag, __savemask: u32) -> u32;
    fn pthread_mutex_init(
        __mutex: *mut u64,
        __mutexattr: *const pthread_mutexattr_t,
    ) -> u32;
    fn pthread_mutex_destroy(__mutex: *mut u64) -> u32;
    fn pthread_mutex_lock(__mutex: *mut u64) -> u32;
    fn pthread_mutex_unlock(__mutex: *mut u64) -> u32;
    fn pthread_key_create(
        __key: *mut pthread_key_t,
        __destr_function: Option::<fn(*mut libc::c_void) -> ()>,
    ) -> u32;
    fn pthread_getspecific(__key: pthread_key_t) -> *mut libc::c_void;
    fn pthread_setspecific(
        __key: pthread_key_t,
        __pointer: *const libc::c_void,
    ) -> u32;
    fn poll(__fds: *mut pollfd, __nfds: nfds_t, __timeout: u32) -> u32;
    fn signal(__sig: u32, __handler: __sighandler_t) -> __sighandler_t;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: u32,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn pthread_delay_np(interval: *const timespec) -> u32;
    fn _setjmp(_: *mut __jmp_buf_tag) -> u32;
    fn _longjmp(_: *mut __jmp_buf_tag, _: u32) -> !;
    fn recvfrom(
        __fd: u32,
        __buf: *mut libc::c_void,
        __n: size_t,
        __flags: u32,
        __addr: __SOCKADDR_ARG,
        __addr_len: *mut socklen_t,
    ) -> ssize_t;
    fn socket(
        __domain: u32,
        __type: u32,
        __protocol: u32,
    ) -> u32;
    fn bind(
        __fd: u32,
        __addr: __CONST_SOCKADDR_ARG,
        __len: socklen_t,
    ) -> u32;
    fn sendto(
        __fd: u32,
        __buf: *const libc::c_void,
        __n: size_t,
        __flags: u32,
        __addr: __CONST_SOCKADDR_ARG,
        __addr_len: socklen_t,
    ) -> ssize_t;
    fn ntohs(__netshort: uint16_t) -> uint16_t;
    fn htonl(__hostlong: u32) -> u32;
    fn htons(__hostshort: uint16_t) -> uint16_t;
    fn LispObjTag(lo: QWord) -> u32;
    fn LispObjData(lo: QWord) -> u32;
    fn WriteLispObjTag(lo: *mut LispObj, newtag: u32);
    fn WriteLispObjData(lo: *mut LispObj, newdata: u32);
    fn InitializeIvoryProcessor(dataBase: *mut isize, tagsBase: *mut Tag);
    fn IvoryProcessorSystemStartup(bootingP: bool) -> Boole;
    fn Runningp() -> Boole;
    fn PushOneFakeFrame();
    fn HaltMachine();
    fn StartMachine(resumeP: bool);
    fn ReadInternalRegister(regno: u32, val: *mut LispObj) -> Boole;
    fn WriteInternalRegister(regno: u32, val: *mut LispObj) -> Boole;
    fn WriteVirtualMemory(vma: isize, object: *mut LispObj) -> u32;
    fn ReadVirtualMemory(vma: isize, object: *mut LispObj) -> u32;
    fn MapVirtualAddressTag(vma: isize) -> *mut Tag;
    fn MapVirtualAddressData(vma: isize) -> *mut isize;
    fn EnsureVirtualAddressRange(
        virtualaddress: isize,
        count: u32,
        faultp: bool,
    ) -> isize;
    fn SignalLater(signal_0: SignalNumber);
    static mut EmbCommAreaPtr: *mut EmbCommArea;
    fn EmbQueueSpace(q: *mut EmbQueue) -> u32;
    fn EmbQueueFilled(q: *mut EmbQueue) -> u32;
    fn EmbQueuePutWord(q: *mut EmbQueue, element: EmbWord);
    fn EmbQueueTakeWord(q: *mut EmbQueue) -> EmbWord;
    fn UnthreadMessageChannel(theChannel: *mut EmbMessageChannel);
    fn ResetOutgoingQueue(q: *mut EmbQueue);
    fn ResetIncomingQueue(q: *mut EmbQueue);
    fn verror(section:&str, format:&str, _: ...);
    fn vpunt(section:&str, format:&str, _: ...);
    fn vwarn(section:&str, format:&str, _: ...);
}
pub type size_t = libc::c_ulong;
pub type u8 = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type i32 = u32;
pub type u32 = libc::c_uint;
pub type u64 = libc::c_ulong;
pub type __time_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type __u64 =&str;
pub type __socklen_t = libc::c_uint;
pub type ssize_t = __ssize_t;
pub type u64 = __u64;
pub type i32 = i32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [libc::c_ulong; 16],
}
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
    pub __value32: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct QData {
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
pub type u64 = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutexattr_t {
    pub __size: [libc::c_char; 4],
    pub __align: u32,
}
pub type pthread_key_t = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub union u64 {
    pub __size: [libc::c_char; 56],
    pub __align: libc::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union u64 {
    pub __data: __pthread_mutex_s,
    pub __size: [libc::c_char; 40],
    pub __align: libc::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union u64 {
    pub __data: __pthread_cond_s,
    pub __size: [libc::c_char; 48],
    pub __align: libc::c_longlong,
}
pub type u8 = u8;
pub type uint16_t = __uint16_t;
pub type u32 = u32;
pub type u64 = u64;
pub type __jmp_buf = [libc::c_long; 8];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __jmp_buf_tag {
    pub __jmpbuf: __jmp_buf,
    pub __mask_was_saved: u32,
    pub __saved_mask: __sigset_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __cancel_jmp_buf_tag {
    pub __cancel_jmp_buf: __jmp_buf,
    pub __mask_was_saved: u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_unwind_buf_t {
    pub __cancel_jmp_buf: [__cancel_jmp_buf_tag; 1],
    pub __pad: [*mut libc::c_void; 4],
}
pub type nfds_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pollfd {
    pub fd: u32,
    pub events: libc::c_short,
    pub revents: libc::c_short,
}
pub type jmp_buf = [__jmp_buf_tag; 1];
pub type __sighandler_t = Option::<fn(u32) -> ()>;
pub type socklen_t = __socklen_t;
pub type pthread_addr_t = *mut libc::c_void;
pub type pthread_cleanuproutine_t = Option::<
    fn(*mut libc::c_void) -> (),
>;
pub type pthread_startroutine_t = Option::<
    fn(*mut libc::c_void) -> *mut libc::c_void,
>;
pub type __socket_type = libc::c_uint;
pub const SOCK_NONBLOCK: __socket_type = 2048;
pub const SOCK_CLOEXEC: __socket_type = 524288;
pub const SOCK_PACKET: __socket_type = 10;
pub const SOCK_DCCP: __socket_type = 6;
pub const SOCK_SEQPACKET: __socket_type = 5;
pub const SOCK_RDM: __socket_type = 4;
pub const SOCK_RAW: __socket_type = 3;
pub const SOCK_DGRAM: __socket_type = 2;
pub const SOCK_STREAM: __socket_type = 1;
pub type sa_family_t = libc::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr {
    pub sa_family: sa_family_t,
    pub sa_data: [libc::c_char; 14],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union __SOCKADDR_ARG {
    pub __sockaddr__: *mut sockaddr,
    pub __sockaddr_at__: *mut sockaddr_at,
    pub __sockaddr_ax25__: *mut sockaddr_ax25,
    pub __sockaddr_dl__: *mut sockaddr_dl,
    pub __sockaddr_eon__: *mut sockaddr_eon,
    pub __sockaddr_in__: *mut sockaddr_in,
    pub __sockaddr_in6__: *mut sockaddr_in6,
    pub __sockaddr_inarp__: *mut sockaddr_inarp,
    pub __sockaddr_ipx__: *mut sockaddr_ipx,
    pub __sockaddr_iso__: *mut sockaddr_iso,
    pub __sockaddr_ns__: *mut sockaddr_ns,
    pub __sockaddr_un__: *mut sockaddr_un,
    pub __sockaddr_x25__: *mut sockaddr_x25,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_in6 {
    pub sin6_family: sa_family_t,
    pub sin6_port: in_port_t,
    pub sin6_flowinfo: u32,
    pub sin6_addr: in6_addr,
    pub sin6_scope_id: u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct in6_addr {
    pub __in6_u: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub __u6_addr8: [u8; 16],
    pub __u6_addr16: [uint16_t; 8],
    pub __u6_addr32: [u32; 4],
}
pub type in_port_t = uint16_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_in {
    pub sin_family: sa_family_t,
    pub sin_port: in_port_t,
    pub sin_addr: in_addr,
    pub sin_zero: [libc::c_uchar; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct in_addr {
    pub s_addr: in_addr_t,
}
pub type in_addr_t = u32;
#[derive(Copy, Clone)]
#[repr(C)]
pub union __CONST_SOCKADDR_ARG {
    pub __sockaddr__: *const sockaddr,
    pub __sockaddr_at__: *const sockaddr_at,
    pub __sockaddr_ax25__: *const sockaddr_ax25,
    pub __sockaddr_dl__: *const sockaddr_dl,
    pub __sockaddr_eon__: *const sockaddr_eon,
    pub __sockaddr_in__: *const sockaddr_in,
    pub __sockaddr_in6__: *const sockaddr_in6,
    pub __sockaddr_inarp__: *const sockaddr_inarp,
    pub __sockaddr_ipx__: *const sockaddr_ipx,
    pub __sockaddr_iso__: *const sockaddr_iso,
    pub __sockaddr_ns__: *const sockaddr_ns,
    pub __sockaddr_un__: *const sockaddr_un,
    pub __sockaddr_x25__: *const sockaddr_x25,
}
pub type EmbWord = i32;
pub type UEmbWord = u32;
pub type EmbPtr = EmbWord;
pub type SignalMask = UEmbWord;
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
    pub tag: u32,
    pub data: C2RustUnnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub u: u32,
    pub s: i32,
    pub f: libc::c_float,
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
    pub generaVersion: C2RustUnnamed_5,
    pub osfVersion: C2RustUnnamed_4,
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
    pub fep: C2RustUnnamed_3,
    pub restart_applications: EmbWord,
    pub signal_interrupt_vector: EmbWord,
    pub base_register: EmbWord,
    pub hostVersion2: EmbWord,
    pub hostVersion3: EmbWord,
    pub MacIvory_NVRAM_settings: C2RustUnnamed_2,
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

#[derive(Copy, Clone)]
#[repr(C)]
pub struct EmbQueue {
    pub element_size: EmbWord,
    pub queue_size: EmbWord,
    pub put_index: EmbWord,
    pub take_index: EmbWord,
    pub signal: SignalNumber,
    pub first_element: [EmbWord; 1],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct EmbMessageChannel {
    pub type_0: EmbWord,
    pub unit: EmbWord,
    pub next: EmbPtr,
    pub subtype: EmbWord,
    pub guestToHostQueue: EmbPtr,
    pub guestToHostReturnQueue: EmbPtr,
    pub guestToHostImpulse: EmbWord,
    pub hostToGuestQueue: EmbPtr,
    pub hostToGuestSupplyQueue: EmbPtr,
    pub hostToGuestImpulse: EmbWord,
    pub subtypeData0: UEmbWord,
    pub subtypeData1: UEmbWord,
}
pub type EmbMessageImpulse = libc::c_uint;
pub const EmbMessageImpulseNone: EmbMessageImpulse = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct EmbMBINChannel {
    pub header: C2RustUnnamed_6,
    pub guestToHostQueue: *mut EmbQueue,
    pub guestToHostReturnQueue: *mut EmbQueue,
    pub hostToGuestQueue: *mut EmbQueue,
    pub hostToGuestSupplyQueue: *mut EmbQueue,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
    pub nextActiveChannel: *mut EmbMessageChannel,
    pub commArea: *mut EmbCommArea,
    pub messageChannel: *mut EmbMessageChannel,
}
pub type EmbMBINImpulse = libc::c_uint;
pub const EmbMBINImpulseShutdown: EmbMBINImpulse = 1;
pub const rm_discard: C2RustUnnamed_8 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
    pub id: libc::c_uint,
    pub acked: bool,
}

pub const rm_mbin: C2RustUnnamed_8 = 9;
pub const rm_coprocessor: C2RustUnnamed_9 = 3;
pub const rm_register: C2RustUnnamed_9 = 2;
pub const rm_virtual: C2RustUnnamed_9 = 1;
pub const rm_physical: C2RustUnnamed_9 = 0;
pub const rm_write: C2RustUnnamed_8 = 3;
pub const rm_system_startup: C2RustUnnamed_8 = 5;
pub const rm_stop: C2RustUnnamed_8 = 10;
pub const rm_read: C2RustUnnamed_8 = 4;
pub const rm_noop: C2RustUnnamed_8 = 1;
pub const rm_create_pages: C2RustUnnamed_8 = 8;
pub const rm_boot: C2RustUnnamed_8 = 7;
pub const rm_ack: C2RustUnnamed_8 = 2;
pub const rm_trap: C2RustUnnamed_8 = 6;
pub type C2RustUnnamed_8 = libc::c_uint;
pub type C2RustUnnamed_9 = libc::c_uint;
static mut spy: u32 = -(1);
static mut send_trap: u32 = 0;
static mut mainThread: pthread_key_t = 0;
static mut spyLock: u64 = u64 {
    __data: __pthread_mutex_s {
        __lock: 0,
        __count: 0,
        __owner: 0,
        __nusers: 0,
        __kind: 0,
        __spins: 0,
        __elision: 0,
        __list: __pthread_list_t {
            __prev: 0 as *const __pthread_internal_list as *mut __pthread_internal_list,
            __next: 0 as *const __pthread_internal_list as *mut __pthread_internal_list,
        },
    },
};
static mut spyLockSetup: bool = false;
static mut spyThread: u64 = 0;
static mut spyThreadSetup: bool = false;

pub static mut activeMBINChannel: *mut EmbMBINChannel = 0 as *const EmbMBINChannel
    as *mut EmbMBINChannel;
static mut MBINHistory: [C2RustUnnamed_7; 16] = [C2RustUnnamed_7 {
    id: 0,
    acked: 0,
}; 16];
 fn divine_port_number(
    mut diagnosticAddress: libc::c_ulong,
) -> u32 {
    let mut port: libc::c_ulong = 0;
    port = (htonl(diagnosticAddress ))
        .wrapping_sub(0x80512900 );
    if port < 0 {
        port = port.wrapping_neg();
    }
    if port > 256 {
        port = port
            .wrapping_rem(256)
            .wrapping_add(256);
    } else {
        port = port.wrapping_rem(256);
    }
    port = port.wrapping_add(2900);
    return port;
}
 fn bind_a_port(
    mut spy_0: u32,
    mut sin: *mut sockaddr_in,
    mut len: u32,
) {
    if bind(
        spy_0,
        __CONST_SOCKADDR_ARG {
            __sockaddr__: sin as *mut sockaddr,
        },
        len as socklen_t,
    ) != 0
    {
        verror(
            b"spy\0"   as&str,

        );
        (*sin)
            .sin_port = htons(
            (ntohs((*sin).sin_port)  + 1) as uint16_t,
        );
        bind_a_port(spy_0, sin, len);
    } else {
        vwarn(

            b"Spy started on port %d.\0"
                as&str,
            ntohs((*sin).sin_port),
        );
    };
}
 fn spy_transmit(
    mut pkt: *mut rm_pkt,
    mut rm_length: u32,
    mut sin: *mut sockaddr_in,
) -> u32 {
    let mut result: u32 = 0;
    let mut __cancel_buf: __pthread_unwind_buf_t = __pthread_unwind_buf_t {
        __cancel_jmp_buf: [__cancel_jmp_buf_tag {
            __cancel_jmp_buf: [0; 8],
            __mask_was_saved: 0,
        }; 1],
        __pad: [0 ; 4],
    };
    let mut __cancel_routine: Option::<fn(*mut libc::c_void) -> ()> = ::std::mem::transmute::<
        Option::<fn(*mut u64) -> u32>,
        pthread_cleanuproutine_t,
    >(
        Some(
            pthread_mutex_unlock
                as fn(*mut u64) -> u32,
        ),
    );
    let mut __cancel_arg: *mut libc::c_void = &mut spyLock as *mut u64
        ;
    let mut __not_first_call: u32 = __sigsetjmp(
        (__cancel_buf.__cancel_jmp_buf).as_mut_ptr()
            as *mut __jmp_buf_tag,
        0,
    );
    if __not_first_call  != 0 {
        __cancel_routine.expect("non-null function pointer")(__cancel_arg);
        __pthread_unwind_next(&mut __cancel_buf);
    }
    __pthread_register_cancel(&mut __cancel_buf);
    if pthread_mutex_lock(&mut spyLock) != 0 {
        vpunt(
            b"spy\0"   as&str,
            b"Unable to lock the spy lock in thread %x\0"
                 as&str,
            pthread_self(),
        );
    }
    if sendto(
        spy,
        &mut *((*pkt).rm_pad).as_mut_ptr().offset(0 )
            as *mut libc::c_uchar ,
        rm_length,
        0,
        __CONST_SOCKADDR_ARG {
            __sockaddr__: sin as *mut sockaddr,
        },
        ::std::mem::size_of::<sockaddr_in>() as socklen_t,
    ) < 0
    {
        verror(
            b"spy\0"   as&str,

        );
        result = 1;
    }
    if pthread_mutex_unlock(&mut spyLock) != 0 {
        vpunt(
            b"spy\0"   as&str,
            b"Unable to unlock the spy lock in thread %x\0"
                 as&str,
            pthread_self(),
        );
    }
    __pthread_unregister_cancel(&mut __cancel_buf);
    return result;
}
 fn read_long(mut bytes: *mut libc::c_uchar) -> u32 {
    return (*bytes.offset(0 )
        | (*bytes.offset(1 )) << 8
        | (*bytes.offset(2 )) << 16
        | (*bytes.offset(3 )) << 24)
        ;
}
 fn write_long(mut bytes: *mut libc::c_uchar, mut data: libc::c_uint) {
    *bytes.offset(0 ) = data as libc::c_uchar;
    *bytes
        .offset(1 ) = (data >> 8) as libc::c_uchar;
    *bytes
        .offset(
            2 ,
        ) = (data >> 16) as libc::c_uchar;
    *bytes
        .offset(
            3 ,
        ) = (data >> 24) as libc::c_uchar;
}
static mut trap_sin: sockaddr_in = sockaddr_in {
    sin_family: 0,
    sin_port: 0,
    sin_addr: in_addr { s_addr: 0 },
    sin_zero: [0; 8],
};
static mut trap_sinValid: bool = false;
static mut mbin_sin: sockaddr_in = sockaddr_in {
    sin_family: 0,
    sin_port: 0,
    sin_addr: in_addr { s_addr: 0 },
    sin_zero: [0; 8],
};
static mut mbin_sinValid: bool = false;
static mut trap_environment: jmp_buf = [__jmp_buf_tag {
    __jmpbuf: [0; 8],
    __mask_was_saved: 0,
    __saved_mask: __sigset_t { __val: [0; 16] },
}; 1];
 fn segv_handler(mut number: u32) {
    _longjmp(trap_environment.as_mut_ptr(), -(1));
}
 fn SpyTopLevel(mut argument: pthread_addr_t) {
    let mut self_0: u64 = pthread_self();
    let mut __cancel_buf: __pthread_unwind_buf_t = __pthread_unwind_buf_t {
        __cancel_jmp_buf: [__cancel_jmp_buf_tag {
            __cancel_jmp_buf: [0; 8],
            __mask_was_saved: 0,
        }; 1],
        __pad: [0 ; 4],
    };
    let mut __cancel_routine: Option::<fn(*mut libc::c_void) -> ()> = ::std::mem::transmute::<
        Option::<fn(u64) -> u32>,
        pthread_cleanuproutine_t,
    >(Some(pthread_detach as fn(u64) -> u32));
    let mut __cancel_arg: *mut libc::c_void = self_0 ;
    let mut __not_first_call: u32 = __sigsetjmp(
        (__cancel_buf.__cancel_jmp_buf).as_mut_ptr()
            as *mut __jmp_buf_tag,
        0,
    );
    if __not_first_call  != 0 {
        __cancel_routine.expect("non-null function pointer")(__cancel_arg);
        __pthread_unwind_next(&mut __cancel_buf);
    }
    __pthread_register_cancel(&mut __cancel_buf);
    if pthread_mutex_lock(&mut spyLock) != 0 {
        vpunt(
            b"spy\0"   as&str,
            b"Unable to lock the spy lock in thread %x\0"
                 as&str,
            self_0,
        );
    }
    if pthread_mutex_unlock(&mut spyLock) != 0 {
        vpunt(
            b"spy\0"   as&str,
            b"Unable to unlock the spy lock in thread %x\0"
                 as&str,
            self_0,
        );
    }
    RemoteMemorySpyLoop();
    __pthread_unregister_cancel(&mut __cancel_buf);
    __cancel_routine.expect("non-null function pointer")(__cancel_arg);
}
 fn RemoteMemorySpyLoop() {
    let mut current_block: u64;
    let mut pkt: rm_pkt = rm_pkt {
        rm_pad: [0; 2],
        rm_id: [0; 4],
        rm_operand: [0; 3],
        rm_opcode: [0; 1],
        data: [0; 1284],
    };
    let mut reply: rm_pkt = rm_pkt {
        rm_pad: [0; 2],
        rm_id: [0; 4],
        rm_operand: [0; 3],
        rm_opcode: [0; 1],
        data: [0; 1284],
    };
    let mut buffer: [LispObj; 256] = [LispObj {
        parts: _LispObj {
            tag: 0,
            data: C2RustUnnamed_1 { u: 0 },
        },
    }; 256];
    let mut bufferp: *mut LispObj = 0 as *mut LispObj;
    let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut vma: libc::c_uint = 0;
    let mut operand: libc::c_uint = 0;
    let mut nwords: u32 = 0;
    let mut nchunks: u32 = 0;
    let mut i: u32 = 0;
    let mut pkt_length: u32 = 0;
    let mut reply_length: u32 = 0;
    let mut sinlen: u32 = 0;
    let mut booted: u32 = 0;
    let mut pkt_source: sockaddr_in = sockaddr_in {
        sin_family: 0,
        sin_port: 0,
        sin_addr: in_addr { s_addr: 0 },
        sin_zero: [0; 8],
    };
    let mut pollSpy: pollfd = pollfd {
        fd: 0,
        events: 0,
        revents: 0,
    };
    let mut forciblyHalted: bool = false;
    pollSpy.fd = spy;
    pollSpy.events = 0x1  as libc::c_short;
    loop {
        u64estcancel();
        if Runningp() == 0 {
            if send_trap != 0 {
                PushOneFakeFrame();
                PushOneFakeFrame();
                if trap_sinValid != 0 {
                    reply.set_rm_opcode(rm_trap);
                    spy_transmit(&mut reply, 10, &mut trap_sin);
                }
                send_trap = 0;
            }
        }
        i = poll(&mut pollSpy, 1  as nfds_t, 1000);
        if 0  == i {
            continue;
        }
        if i < 0  {
            vpunt(
                b"spy\0"   as&str,
                b"Waiting for a packet from the remote debugger\0"
                     as&str,
            );
        } else if pollSpy.revents
            & (0x10  | 0x20) != 0
        {
            break;
        }
        sinlen = ::std::mem::size_of::<sockaddr_in>();
        pkt_length = recvfrom(
            spy,
            &mut *(pkt.rm_pad).as_mut_ptr().offset(0 )
                as *mut libc::c_uchar ,
            (10  + 1284),
            0,
            __SOCKADDR_ARG {
                __sockaddr__: &mut pkt_source as *mut sockaddr_in as *mut sockaddr,
            },
            &mut sinlen as *mut u32 as *mut socklen_t,
        );
        if pkt_length < 0  {
            vpunt(
                b"spy\0"   as&str,
                b"Reading packet from remote debugger\0"
                     as&str,
            );
        }
        reply.rm_operand[0  ] = 0  as libc::c_uchar;
        reply.rm_operand[1  ] = 0  as libc::c_uchar;
        reply.rm_operand[2  ] = 0  as libc::c_uchar;
        reply_length = 10;
        reply.set_rm_opcode(rm_ack);
        memcpy(
            &mut *(reply.rm_id).as_mut_ptr().offset(0 )
                as *mut libc::c_uchar ,
            &mut *(pkt.rm_id).as_mut_ptr().offset(0 )
                as *mut libc::c_uchar ,
            4,
        );
        operand = read_long(
            &mut *(pkt.rm_operand).as_mut_ptr().offset(0 ),
        ) & 0xffffff;
        match pkt.rm_opcode() {
            7 => {
                if Runningp() != 0 {
                    forciblyHalted = true;
                    HaltMachine();
                    while Runningp() != 0 {}
                }
                spy_transmit(&mut reply, reply_length, &mut pkt_source);
                InitializeIvoryProcessor(
                    MapVirtualAddressData(0 ),
                    MapVirtualAddressTag(0 ),
                );
                booted = 1;
                if forciblyHalted != 0 {
                    forciblyHalted = false;
                    StartMachine(true);
                }
                continue;
            }
            8 => {
                if Runningp() != 0 {
                    forciblyHalted = true;
                    HaltMachine();
                    while Runningp() != 0 {}
                }
                vma = read_long(
                    &mut *(pkt.data).as_mut_ptr().offset(0 ),
                );
                nwords = read_long(
                    &mut *(pkt.data).as_mut_ptr().offset(4 ),
                );
                EnsureVirtualAddressRange(
                    vma ,
                    nwords,
                    false,
                );
                spy_transmit(&mut reply, reply_length, &mut pkt_source);
                if forciblyHalted != 0 {
                    forciblyHalted = false;
                    StartMachine(true);
                }
                continue;
            }
            1 => {
                spy_transmit(&mut reply, reply_length, &mut pkt_source);
                continue;
            }
            4 => {
                if Runningp() != 0 {
                    forciblyHalted = true;
                    HaltMachine();
                    while Runningp() != 0 {}
                }
                vma = read_long(
                    &mut *(pkt.data).as_mut_ptr().offset(0 ),
                );
                nwords = (operand & 0x3ff);
                nchunks = (nwords + 3) / 4;
                match operand >> 10  & 3 {
                    0 => {}
                    1 => {
                        current_block = 7385833325316299293;
                        match current_block {
                            7385833325316299293 => {
                                let mut old_segv_handler: Option::<
                                    fn() -> (),
                                > = ::std::mem::transmute::<
                                    __sighandler_t,
                                    Option::<fn() -> ()>,
                                >(
                                    signal(
                                        11,
                                        Some(
                                            segv_handler as fn(u32) -> (),
                                        ),
                                    ),
                                );
                                if _setjmp(trap_environment.as_mut_ptr()) != 0 {
                                    signal(
                                        11,
                                        ::std::mem::transmute::<
                                            Option::<fn() -> ()>,
                                            __sighandler_t,
                                        >(old_segv_handler),
                                    );
                                    current_block = 15889423180841675952;
                                } else {
                                    i = 0;
                                    while i < nwords {
                                        ReadVirtualMemory(
                                            vma.wrapping_add(i ) ,
                                            &mut *buffer.as_mut_ptr().offset(i ),
                                        ) != 0;
                                        i += 1;
                                    }
                                    signal(
                                        11,
                                        ::std::mem::transmute::<
                                            Option::<fn() -> ()>,
                                            __sighandler_t,
                                        >(old_segv_handler),
                                    );
                                    current_block = 517042441694919077;
                                }
                            }
                            13144677794153224956 => {
                                i = 0;
                                while i < nwords {
                                    ReadInternalRegister(
                                        vma.wrapping_add(i ),
                                        &mut *buffer.as_mut_ptr().offset(i ),
                                    );
                                    i += 1;
                                }
                                current_block = 517042441694919077;
                            }
                            11452846818681027168 => {
                                i = 0;
                                while i < nwords {
                                    vwarn(
                                        b"spy\0"
                                            as&str,
                                        b"Read of coprocessor register %d failed.\0"
                                             as&str,
                                        vma.wrapping_add(i ),
                                    );
                                    i += 1;
                                }
                                current_block = 517042441694919077;
                            }
                            _ => {}
                        }
                        match current_block {
                            15889423180841675952 => {}
                            _ => {
                                i = 0;
                                bufferp = &mut *buffer
                                    .as_mut_ptr()
                                    .offset(0 ) as *mut LispObj;
                                p = &mut *(reply.data)
                                    .as_mut_ptr()
                                    .offset(0 ) as *mut libc::c_uchar;
                                while i < nchunks {
                                    *p
                                        .offset(
                                            0 ,
                                        ) = LispObjTag(*bufferp.offset(0 ))
                                        as libc::c_uchar;
                                    *p
                                        .offset(
                                            1 ,
                                        ) = LispObjTag(*bufferp.offset(1 ))
                                        as libc::c_uchar;
                                    *p
                                        .offset(
                                            2 ,
                                        ) = LispObjTag(*bufferp.offset(2 ))
                                        as libc::c_uchar;
                                    *p
                                        .offset(
                                            3 ,
                                        ) = LispObjTag(*bufferp.offset(3 ))
                                        as libc::c_uchar;
                                    write_long(
                                        &mut *p.offset(4 ),
                                        LispObjData(*bufferp.offset(0 )),
                                    );
                                    write_long(
                                        &mut *p.offset(8 ),
                                        LispObjData(*bufferp.offset(1 )),
                                    );
                                    write_long(
                                        &mut *p.offset(12 ),
                                        LispObjData(*bufferp.offset(2 )),
                                    );
                                    write_long(
                                        &mut *p.offset(16 ),
                                        LispObjData(*bufferp.offset(3 )),
                                    );
                                    i += 1;
                                    bufferp = bufferp.offset(4 );
                                    p = p.offset(20 );
                                }
                                reply_length += nchunks * 20;
                                spy_transmit(&mut reply, reply_length, &mut pkt_source);
                                if forciblyHalted != 0 {
                                    forciblyHalted = false;
                                    StartMachine(true);
                                }
                                continue;
                            }
                        }
                    }
                    2 => {
                        current_block = 13144677794153224956;
                        match current_block {
                            7385833325316299293 => {
                                let mut old_segv_handler: Option::<
                                    fn() -> (),
                                > = ::std::mem::transmute::<
                                    __sighandler_t,
                                    Option::<fn() -> ()>,
                                >(
                                    signal(
                                        11,
                                        Some(
                                            segv_handler as fn(u32) -> (),
                                        ),
                                    ),
                                );
                                if _setjmp(trap_environment.as_mut_ptr()) != 0 {
                                    signal(
                                        11,
                                        ::std::mem::transmute::<
                                            Option::<fn() -> ()>,
                                            __sighandler_t,
                                        >(old_segv_handler),
                                    );
                                    current_block = 15889423180841675952;
                                } else {
                                    i = 0;
                                    while i < nwords {
                                        ReadVirtualMemory(
                                            vma.wrapping_add(i ) ,
                                            &mut *buffer.as_mut_ptr().offset(i ),
                                        ) != 0;
                                        i += 1;
                                    }
                                    signal(
                                        11,
                                        ::std::mem::transmute::<
                                            Option::<fn() -> ()>,
                                            __sighandler_t,
                                        >(old_segv_handler),
                                    );
                                    current_block = 517042441694919077;
                                }
                            }
                            13144677794153224956 => {
                                i = 0;
                                while i < nwords {
                                    ReadInternalRegister(
                                        vma.wrapping_add(i ),
                                        &mut *buffer.as_mut_ptr().offset(i ),
                                    );
                                    i += 1;
                                }
                                current_block = 517042441694919077;
                            }
                            11452846818681027168 => {
                                i = 0;
                                while i < nwords {
                                    vwarn(
                                        b"spy\0"
                                            as&str,
                                        b"Read of coprocessor register %d failed.\0"
                                             as&str,
                                        vma.wrapping_add(i ),
                                    );
                                    i += 1;
                                }
                                current_block = 517042441694919077;
                            }
                            _ => {}
                        }
                        match current_block {
                            15889423180841675952 => {}
                            _ => {
                                i = 0;
                                bufferp = &mut *buffer
                                    .as_mut_ptr()
                                    .offset(0 ) as *mut LispObj;
                                p = &mut *(reply.data)
                                    .as_mut_ptr()
                                    .offset(0 ) as *mut libc::c_uchar;
                                while i < nchunks {
                                    *p
                                        .offset(
                                            0 ,
                                        ) = LispObjTag(*bufferp.offset(0 ))
                                        as libc::c_uchar;
                                    *p
                                        .offset(
                                            1 ,
                                        ) = LispObjTag(*bufferp.offset(1 ))
                                        as libc::c_uchar;
                                    *p
                                        .offset(
                                            2 ,
                                        ) = LispObjTag(*bufferp.offset(2 ))
                                        as libc::c_uchar;
                                    *p
                                        .offset(
                                            3 ,
                                        ) = LispObjTag(*bufferp.offset(3 ))
                                        as libc::c_uchar;
                                    write_long(
                                        &mut *p.offset(4 ),
                                        LispObjData(*bufferp.offset(0 )),
                                    );
                                    write_long(
                                        &mut *p.offset(8 ),
                                        LispObjData(*bufferp.offset(1 )),
                                    );
                                    write_long(
                                        &mut *p.offset(12 ),
                                        LispObjData(*bufferp.offset(2 )),
                                    );
                                    write_long(
                                        &mut *p.offset(16 ),
                                        LispObjData(*bufferp.offset(3 )),
                                    );
                                    i += 1;
                                    bufferp = bufferp.offset(4 );
                                    p = p.offset(20 );
                                }
                                reply_length += nchunks * 20;
                                spy_transmit(&mut reply, reply_length, &mut pkt_source);
                                if forciblyHalted != 0 {
                                    forciblyHalted = false;
                                    StartMachine(true);
                                }
                                continue;
                            }
                        }
                    }
                    3 => {
                        current_block = 11452846818681027168;
                        match current_block {
                            7385833325316299293 => {
                                let mut old_segv_handler: Option::<
                                    fn() -> (),
                                > = ::std::mem::transmute::<
                                    __sighandler_t,
                                    Option::<fn() -> ()>,
                                >(
                                    signal(
                                        11,
                                        Some(
                                            segv_handler as fn(u32) -> (),
                                        ),
                                    ),
                                );
                                if _setjmp(trap_environment.as_mut_ptr()) != 0 {
                                    signal(
                                        11,
                                        ::std::mem::transmute::<
                                            Option::<fn() -> ()>,
                                            __sighandler_t,
                                        >(old_segv_handler),
                                    );
                                    current_block = 15889423180841675952;
                                } else {
                                    i = 0;
                                    while i < nwords {
                                        ReadVirtualMemory(
                                            vma.wrapping_add(i ) ,
                                            &mut *buffer.as_mut_ptr().offset(i ),
                                        ) != 0;
                                        i += 1;
                                    }
                                    signal(
                                        11,
                                        ::std::mem::transmute::<
                                            Option::<fn() -> ()>,
                                            __sighandler_t,
                                        >(old_segv_handler),
                                    );
                                    current_block = 517042441694919077;
                                }
                            }
                            13144677794153224956 => {
                                i = 0;
                                while i < nwords {
                                    ReadInternalRegister(
                                        vma.wrapping_add(i ),
                                        &mut *buffer.as_mut_ptr().offset(i ),
                                    );
                                    i += 1;
                                }
                                current_block = 517042441694919077;
                            }
                            11452846818681027168 => {
                                i = 0;
                                while i < nwords {
                                    vwarn(
                                        b"spy\0"
                                            as&str,
                                        b"Read of coprocessor register %d failed.\0"
                                             as&str,
                                        vma.wrapping_add(i ),
                                    );
                                    i += 1;
                                }
                                current_block = 517042441694919077;
                            }
                            _ => {}
                        }
                        match current_block {
                            15889423180841675952 => {}
                            _ => {
                                i = 0;
                                bufferp = &mut *buffer
                                    .as_mut_ptr()
                                    .offset(0 ) as *mut LispObj;
                                p = &mut *(reply.data)
                                    .as_mut_ptr()
                                    .offset(0 ) as *mut libc::c_uchar;
                                while i < nchunks {
                                    *p
                                        .offset(
                                            0 ,
                                        ) = LispObjTag(*bufferp.offset(0 ))
                                        as libc::c_uchar;
                                    *p
                                        .offset(
                                            1 ,
                                        ) = LispObjTag(*bufferp.offset(1 ))
                                        as libc::c_uchar;
                                    *p
                                        .offset(
                                            2 ,
                                        ) = LispObjTag(*bufferp.offset(2 ))
                                        as libc::c_uchar;
                                    *p
                                        .offset(
                                            3 ,
                                        ) = LispObjTag(*bufferp.offset(3 ))
                                        as libc::c_uchar;
                                    write_long(
                                        &mut *p.offset(4 ),
                                        LispObjData(*bufferp.offset(0 )),
                                    );
                                    write_long(
                                        &mut *p.offset(8 ),
                                        LispObjData(*bufferp.offset(1 )),
                                    );
                                    write_long(
                                        &mut *p.offset(12 ),
                                        LispObjData(*bufferp.offset(2 )),
                                    );
                                    write_long(
                                        &mut *p.offset(16 ),
                                        LispObjData(*bufferp.offset(3 )),
                                    );
                                    i += 1;
                                    bufferp = bufferp.offset(4 );
                                    p = p.offset(20 );
                                }
                                reply_length += nchunks * 20;
                                spy_transmit(&mut reply, reply_length, &mut pkt_source);
                                if forciblyHalted != 0 {
                                    forciblyHalted = false;
                                    StartMachine(true);
                                }
                                continue;
                            }
                        }
                    }
                    _ => {
                        current_block = 517042441694919077;
                        match current_block {
                            7385833325316299293 => {
                                let mut old_segv_handler: Option::<
                                    fn() -> (),
                                > = ::std::mem::transmute::<
                                    __sighandler_t,
                                    Option::<fn() -> ()>,
                                >(
                                    signal(
                                        11,
                                        Some(
                                            segv_handler as fn(u32) -> (),
                                        ),
                                    ),
                                );
                                if _setjmp(trap_environment.as_mut_ptr()) != 0 {
                                    signal(
                                        11,
                                        ::std::mem::transmute::<
                                            Option::<fn() -> ()>,
                                            __sighandler_t,
                                        >(old_segv_handler),
                                    );
                                    current_block = 15889423180841675952;
                                } else {
                                    i = 0;
                                    while i < nwords {
                                        ReadVirtualMemory(
                                            vma.wrapping_add(i ) ,
                                            &mut *buffer.as_mut_ptr().offset(i ),
                                        ) != 0;
                                        i += 1;
                                    }
                                    signal(
                                        11,
                                        ::std::mem::transmute::<
                                            Option::<fn() -> ()>,
                                            __sighandler_t,
                                        >(old_segv_handler),
                                    );
                                    current_block = 517042441694919077;
                                }
                            }
                            13144677794153224956 => {
                                i = 0;
                                while i < nwords {
                                    ReadInternalRegister(
                                        vma.wrapping_add(i ),
                                        &mut *buffer.as_mut_ptr().offset(i ),
                                    );
                                    i += 1;
                                }
                                current_block = 517042441694919077;
                            }
                            11452846818681027168 => {
                                i = 0;
                                while i < nwords {
                                    vwarn(
                                        b"spy\0"
                                            as&str,
                                        b"Read of coprocessor register %d failed.\0"
                                             as&str,
                                        vma.wrapping_add(i ),
                                    );
                                    i += 1;
                                }
                                current_block = 517042441694919077;
                            }
                            _ => {}
                        }
                        match current_block {
                            15889423180841675952 => {}
                            _ => {
                                i = 0;
                                bufferp = &mut *buffer
                                    .as_mut_ptr()
                                    .offset(0 ) as *mut LispObj;
                                p = &mut *(reply.data)
                                    .as_mut_ptr()
                                    .offset(0 ) as *mut libc::c_uchar;
                                while i < nchunks {
                                    *p
                                        .offset(
                                            0 ,
                                        ) = LispObjTag(*bufferp.offset(0 ))
                                        as libc::c_uchar;
                                    *p
                                        .offset(
                                            1 ,
                                        ) = LispObjTag(*bufferp.offset(1 ))
                                        as libc::c_uchar;
                                    *p
                                        .offset(
                                            2 ,
                                        ) = LispObjTag(*bufferp.offset(2 ))
                                        as libc::c_uchar;
                                    *p
                                        .offset(
                                            3 ,
                                        ) = LispObjTag(*bufferp.offset(3 ))
                                        as libc::c_uchar;
                                    write_long(
                                        &mut *p.offset(4 ),
                                        LispObjData(*bufferp.offset(0 )),
                                    );
                                    write_long(
                                        &mut *p.offset(8 ),
                                        LispObjData(*bufferp.offset(1 )),
                                    );
                                    write_long(
                                        &mut *p.offset(12 ),
                                        LispObjData(*bufferp.offset(2 )),
                                    );
                                    write_long(
                                        &mut *p.offset(16 ),
                                        LispObjData(*bufferp.offset(3 )),
                                    );
                                    i += 1;
                                    bufferp = bufferp.offset(4 );
                                    p = p.offset(20 );
                                }
                                reply_length += nchunks * 20;
                                spy_transmit(&mut reply, reply_length, &mut pkt_source);
                                if forciblyHalted != 0 {
                                    forciblyHalted = false;
                                    StartMachine(true);
                                }
                                continue;
                            }
                        }
                    }
                }
            }
            10 => {
                forciblyHalted = false;
                HaltMachine();
                spy_transmit(&mut reply, reply_length, &mut pkt_source);
                continue;
            }
            5 => {
                spy_transmit(&mut reply, reply_length, &mut pkt_source);
                memcpy(
                    &mut trap_sin as *mut sockaddr_in ,
                    &mut pkt_source as *mut sockaddr_in ,
                    ::std::mem::size_of::<sockaddr_in>(),
                );
                trap_sinValid = true;
                memcpy(
                    &mut mbin_sin as *mut sockaddr_in ,
                    &mut pkt_source as *mut sockaddr_in ,
                    ::std::mem::size_of::<sockaddr_in>(),
                );
                mbin_sinValid = true;
                if IvoryProcessorSystemStartup(booted as Boole) == 0 {
                    vwarn(
                        b"spy\0"
                            as&str,
                        b"Bad start routine.\0"
                            as&str,
                    );
                }
                send_trap = 1;
                continue;
            }
            3 => {
                if Runningp() != 0 {
                    forciblyHalted = true;
                    HaltMachine();
                    while Runningp() != 0 {}
                }
                vma = read_long(
                    &mut *(pkt.data).as_mut_ptr().offset(0 ),
                );
                nwords = (operand & 0x3ff);
                nchunks = (nwords + 3) / 4;
                i = 0;
                bufferp = &mut *buffer.as_mut_ptr().offset(0 )
                    as *mut LispObj;
                p = &mut *(pkt.data).as_mut_ptr().offset(4 )
                    as *mut libc::c_uchar;
                while i < nchunks {
                    WriteLispObjTag(
                        &mut *bufferp.offset(0 ),
                        *p.offset(0 ) ,
                    );
                    WriteLispObjTag(
                        &mut *bufferp.offset(1 ),
                        *p.offset(1 ) ,
                    );
                    WriteLispObjTag(
                        &mut *bufferp.offset(2 ),
                        *p.offset(2 ) ,
                    );
                    WriteLispObjTag(
                        &mut *bufferp.offset(3 ),
                        *p.offset(3 ) ,
                    );
                    WriteLispObjData(
                        &mut *bufferp.offset(0 ),
                        read_long(&mut *p.offset(4 )),
                    );
                    WriteLispObjData(
                        &mut *bufferp.offset(1 ),
                        read_long(&mut *p.offset(8 )),
                    );
                    WriteLispObjData(
                        &mut *bufferp.offset(2 ),
                        read_long(&mut *p.offset(12 )),
                    );
                    WriteLispObjData(
                        &mut *bufferp.offset(3 ),
                        read_long(&mut *p.offset(16 )),
                    );
                    i += 1;
                    bufferp = bufferp.offset(4 );
                    p = p.offset(20 );
                }
                match operand >> 10  & 3 {
                    0 => {}
                    1 => {
                        current_block = 9354678635443812511;
                        match current_block {
                            9354678635443812511 => {
                                let mut old_segv_handler_0: Option::<
                                    fn() -> (),
                                > = ::std::mem::transmute::<
                                    __sighandler_t,
                                    Option::<fn() -> ()>,
                                >(
                                    signal(
                                        11,
                                        Some(
                                            segv_handler as fn(u32) -> (),
                                        ),
                                    ),
                                );
                                if _setjmp(trap_environment.as_mut_ptr()) != 0 {
                                    signal(
                                        11,
                                        ::std::mem::transmute::<
                                            Option::<fn() -> ()>,
                                            __sighandler_t,
                                        >(old_segv_handler_0),
                                    );
                                    current_block = 15889423180841675952;
                                } else {
                                    i = 0;
                                    while i < nwords {
                                        WriteVirtualMemory(
                                            vma.wrapping_add(i ) ,
                                            &mut *buffer.as_mut_ptr().offset(i ),
                                        ) != 0;
                                        i += 1;
                                    }
                                    signal(
                                        11,
                                        ::std::mem::transmute::<
                                            Option::<fn() -> ()>,
                                            __sighandler_t,
                                        >(old_segv_handler_0),
                                    );
                                    current_block = 13918987447127423209;
                                }
                            }
                            4460221470432502560 => {
                                i = 0;
                                while i < nwords {
                                    if WriteInternalRegister(
                                        vma.wrapping_add(i ),
                                        &mut *buffer.as_mut_ptr().offset(i ),
                                    ) == 0
                                    {
                                        vwarn(
                                            b"spy\0"
                                                as&str,
                                            b"Write of internal register %d failed\0"
                                                 as&str,
                                            vma.wrapping_add(i ),
                                        );
                                    }
                                    i += 1;
                                }
                                current_block = 13918987447127423209;
                            }
                            13174701388269138530 => {
                                i = 0;
                                while i < nwords {
                                    vwarn(
                                        b"spy\0"
                                            as&str,
                                        b"Write of coprocessor register %d failed.\0"
                                             as&str,
                                        vma.wrapping_add(i ),
                                    );
                                    i += 1;
                                }
                                current_block = 13918987447127423209;
                            }
                            _ => {}
                        }
                        match current_block {
                            15889423180841675952 => {}
                            _ => {
                                spy_transmit(&mut reply, reply_length, &mut pkt_source);
                                if forciblyHalted != 0 {
                                    forciblyHalted = false;
                                    StartMachine(false);
                                }
                                continue;
                            }
                        }
                    }
                    2 => {
                        current_block = 4460221470432502560;
                        match current_block {
                            9354678635443812511 => {
                                let mut old_segv_handler_0: Option::<
                                    fn() -> (),
                                > = ::std::mem::transmute::<
                                    __sighandler_t,
                                    Option::<fn() -> ()>,
                                >(
                                    signal(
                                        11,
                                        Some(
                                            segv_handler as fn(u32) -> (),
                                        ),
                                    ),
                                );
                                if _setjmp(trap_environment.as_mut_ptr()) != 0 {
                                    signal(
                                        11,
                                        ::std::mem::transmute::<
                                            Option::<fn() -> ()>,
                                            __sighandler_t,
                                        >(old_segv_handler_0),
                                    );
                                    current_block = 15889423180841675952;
                                } else {
                                    i = 0;
                                    while i < nwords {
                                        WriteVirtualMemory(
                                            vma.wrapping_add(i ) ,
                                            &mut *buffer.as_mut_ptr().offset(i ),
                                        ) != 0;
                                        i += 1;
                                    }
                                    signal(
                                        11,
                                        ::std::mem::transmute::<
                                            Option::<fn() -> ()>,
                                            __sighandler_t,
                                        >(old_segv_handler_0),
                                    );
                                    current_block = 13918987447127423209;
                                }
                            }
                            4460221470432502560 => {
                                i = 0;
                                while i < nwords {
                                    if WriteInternalRegister(
                                        vma.wrapping_add(i ),
                                        &mut *buffer.as_mut_ptr().offset(i ),
                                    ) == 0
                                    {
                                        vwarn(
                                            b"spy\0"
                                                as&str,
                                            b"Write of internal register %d failed\0"
                                                 as&str,
                                            vma.wrapping_add(i ),
                                        );
                                    }
                                    i += 1;
                                }
                                current_block = 13918987447127423209;
                            }
                            13174701388269138530 => {
                                i = 0;
                                while i < nwords {
                                    vwarn(
                                        b"spy\0"
                                            as&str,
                                        b"Write of coprocessor register %d failed.\0"
                                             as&str,
                                        vma.wrapping_add(i ),
                                    );
                                    i += 1;
                                }
                                current_block = 13918987447127423209;
                            }
                            _ => {}
                        }
                        match current_block {
                            15889423180841675952 => {}
                            _ => {
                                spy_transmit(&mut reply, reply_length, &mut pkt_source);
                                if forciblyHalted != 0 {
                                    forciblyHalted = false;
                                    StartMachine(false);
                                }
                                continue;
                            }
                        }
                    }
                    3 => {
                        current_block = 13174701388269138530;
                        match current_block {
                            9354678635443812511 => {
                                let mut old_segv_handler_0: Option::<
                                    fn() -> (),
                                > = ::std::mem::transmute::<
                                    __sighandler_t,
                                    Option::<fn() -> ()>,
                                >(
                                    signal(
                                        11,
                                        Some(
                                            segv_handler as fn(u32) -> (),
                                        ),
                                    ),
                                );
                                if _setjmp(trap_environment.as_mut_ptr()) != 0 {
                                    signal(
                                        11,
                                        ::std::mem::transmute::<
                                            Option::<fn() -> ()>,
                                            __sighandler_t,
                                        >(old_segv_handler_0),
                                    );
                                    current_block = 15889423180841675952;
                                } else {
                                    i = 0;
                                    while i < nwords {
                                        WriteVirtualMemory(
                                            vma.wrapping_add(i ) ,
                                            &mut *buffer.as_mut_ptr().offset(i ),
                                        ) != 0;
                                        i += 1;
                                    }
                                    signal(
                                        11,
                                        ::std::mem::transmute::<
                                            Option::<fn() -> ()>,
                                            __sighandler_t,
                                        >(old_segv_handler_0),
                                    );
                                    current_block = 13918987447127423209;
                                }
                            }
                            4460221470432502560 => {
                                i = 0;
                                while i < nwords {
                                    if WriteInternalRegister(
                                        vma.wrapping_add(i ),
                                        &mut *buffer.as_mut_ptr().offset(i ),
                                    ) == 0
                                    {
                                        vwarn(
                                            b"spy\0"
                                                as&str,
                                            b"Write of internal register %d failed\0"
                                                 as&str,
                                            vma.wrapping_add(i ),
                                        );
                                    }
                                    i += 1;
                                }
                                current_block = 13918987447127423209;
                            }
                            13174701388269138530 => {
                                i = 0;
                                while i < nwords {
                                    vwarn(
                                        b"spy\0"
                                            as&str,
                                        b"Write of coprocessor register %d failed.\0"
                                             as&str,
                                        vma.wrapping_add(i ),
                                    );
                                    i += 1;
                                }
                                current_block = 13918987447127423209;
                            }
                            _ => {}
                        }
                        match current_block {
                            15889423180841675952 => {}
                            _ => {
                                spy_transmit(&mut reply, reply_length, &mut pkt_source);
                                if forciblyHalted != 0 {
                                    forciblyHalted = false;
                                    StartMachine(false);
                                }
                                continue;
                            }
                        }
                    }
                    _ => {
                        current_block = 13918987447127423209;
                        match current_block {
                            9354678635443812511 => {
                                let mut old_segv_handler_0: Option::<
                                    fn() -> (),
                                > = ::std::mem::transmute::<
                                    __sighandler_t,
                                    Option::<fn() -> ()>,
                                >(
                                    signal(
                                        11,
                                        Some(
                                            segv_handler as fn(u32) -> (),
                                        ),
                                    ),
                                );
                                if _setjmp(trap_environment.as_mut_ptr()) != 0 {
                                    signal(
                                        11,
                                        ::std::mem::transmute::<
                                            Option::<fn() -> ()>,
                                            __sighandler_t,
                                        >(old_segv_handler_0),
                                    );
                                    current_block = 15889423180841675952;
                                } else {
                                    i = 0;
                                    while i < nwords {
                                        WriteVirtualMemory(
                                            vma.wrapping_add(i ) ,
                                            &mut *buffer.as_mut_ptr().offset(i ),
                                        ) != 0;
                                        i += 1;
                                    }
                                    signal(
                                        11,
                                        ::std::mem::transmute::<
                                            Option::<fn() -> ()>,
                                            __sighandler_t,
                                        >(old_segv_handler_0),
                                    );
                                    current_block = 13918987447127423209;
                                }
                            }
                            4460221470432502560 => {
                                i = 0;
                                while i < nwords {
                                    if WriteInternalRegister(
                                        vma.wrapping_add(i ),
                                        &mut *buffer.as_mut_ptr().offset(i ),
                                    ) == 0
                                    {
                                        vwarn(
                                            b"spy\0"
                                                as&str,
                                            b"Write of internal register %d failed\0"
                                                 as&str,
                                            vma.wrapping_add(i ),
                                        );
                                    }
                                    i += 1;
                                }
                                current_block = 13918987447127423209;
                            }
                            13174701388269138530 => {
                                i = 0;
                                while i < nwords {
                                    vwarn(
                                        b"spy\0"
                                            as&str,
                                        b"Write of coprocessor register %d failed.\0"
                                             as&str,
                                        vma.wrapping_add(i ),
                                    );
                                    i += 1;
                                }
                                current_block = 13918987447127423209;
                            }
                            _ => {}
                        }
                        match current_block {
                            15889423180841675952 => {}
                            _ => {
                                spy_transmit(&mut reply, reply_length, &mut pkt_source);
                                if forciblyHalted != 0 {
                                    forciblyHalted = false;
                                    StartMachine(false);
                                }
                                continue;
                            }
                        }
                    }
                }
            }
            9 => {
                let mut bufferEmbPtr: EmbPtr = 0;
                let mut buffer_0: *mut rm_aligned_pkt = 0 as *mut rm_aligned_pkt;
                let mut id: libc::c_uint = 0;
                let mut historyID: u32 = 0;
                memcpy(
                    &mut mbin_sin as *mut sockaddr_in ,
                    &mut pkt_source as *mut sockaddr_in ,
                    ::std::mem::size_of::<sockaddr_in>(),
                );
                mbin_sinValid = true;
                if !activeMBINChannel.is_null() {
                    id = read_long(
                        &mut *(pkt.rm_id).as_mut_ptr().offset(0 ),
                    );
                    historyID = (id & 0xf);
                    if MBINHistory[historyID ].id == id {
                        if MBINHistory[historyID ].acked != 0 {
                            spy_transmit(&mut reply, reply_length, &mut pkt_source);
                        }
                    } else if EmbQueueFilled((*activeMBINChannel).hostToGuestSupplyQueue)
                        != 0 && EmbQueueSpace((*activeMBINChannel).hostToGuestQueue) != 0
                    {
                        bufferEmbPtr = EmbQueueTakeWord(
                            (*activeMBINChannel).hostToGuestSupplyQueue,
                        );
                        if bufferEmbPtr != 0 && bufferEmbPtr != -(1) {
                            buffer_0 = &mut *(EmbCommAreaPtr as *mut EmbWord)
                                .offset(bufferEmbPtr ) as *mut EmbWord as PtrV
                                as *mut rm_aligned_pkt;
                            memcpy(
                                &mut *((*buffer_0).rm_id)
                                    .as_mut_ptr()
                                    .offset(0 ) as *mut libc::c_uchar
                                    ,
                                &mut *(pkt.rm_id)
                                    .as_mut_ptr()
                                    .offset(0 ) as *mut libc::c_uchar
                                    ,
                                8,
                            );
                            memcpy(
                                &mut *((*buffer_0).data)
                                    .as_mut_ptr()
                                    .offset(0 ) as *mut libc::c_uchar
                                    ,
                                &mut *(pkt.data)
                                    .as_mut_ptr()
                                    .offset(0 ) as *mut libc::c_uchar
                                    ,
                                operand,
                            );
                            EmbQueuePutWord(
                                (*activeMBINChannel).hostToGuestQueue,
                                bufferEmbPtr,
                            );
                            MBINHistory[historyID ].id = id;
                            MBINHistory[historyID ]
                                .acked = false;
                        }
                    }
                }
                continue;
            }
            0 | _ => {
                continue;
            }
        }
        reply.rm_operand[0  ] = 1  as libc::c_uchar;
        spy_transmit(&mut reply, reply_length, &mut pkt_source);
        if forciblyHalted != 0 {
            forciblyHalted = false;
            StartMachine(
                (rm_read  == pkt.rm_opcode()) as bool,
            );
        }
    };
}

pub  fn InitializeSpy(
    mut sendTrapP: bool,
    mut diagnosticAddress: libc::c_ulong,
) {
    let mut sin: sockaddr_in = sockaddr_in {
        sin_family: 0,
        sin_port: 0,
        sin_addr: in_addr { s_addr: 0 },
        sin_zero: [0; 8],
    };
    let mut ipport_remote_memory: u32 = 0;
    if pthread_key_create(&mut mainThread, None) != 0 {
        vpunt(

            b"Unable to establish per-thread data.\0"
                as&str,
        );
    }
    pthread_setspecific(mainThread, 1  );
    atexit(Some(TerminateSpy as fn() -> ()));
    ipport_remote_memory = divine_port_number(diagnosticAddress);
    spy = socket(2, SOCK_DGRAM, 0);
    if spy < 0  {
        vpunt(
            b"spy\0"   as&str,
            b"Unable to create spy socket\0"
                as&str,
        );
    }
    sin.sin_family = 2  as sa_family_t;
    sin.sin_addr.s_addr = 0  as in_addr_t;
    sin.sin_port = htons(ipport_remote_memory as uint16_t);
    bind_a_port(
        spy,
        &mut sin,
        ::std::mem::size_of::<sockaddr_in>(),
    );
    send_trap = sendTrapP;
    memset(
        &mut *MBINHistory.as_mut_ptr().offset(0 )
            as *mut C2RustUnnamed_7 as &str ,
        0,
        ::std::mem::size_of::<[C2RustUnnamed_7; 16]>(),
    );
    if pthread_mutex_init(&mut spyLock, 0 as *const pthread_mutexattr_t) != 0 {
        vpunt(
            b"spy\0"   as&str,
            b"Unable to create the spy lock\0"
                as&str,
        );
    }
    spyLockSetup = true;
    if pthread_create(
        &mut spyThread,
        &mut (*EmbCommAreaPtr).pollThreadAttrs,
        ::std::mem::transmute::<
            Option::<fn(pthread_addr_t) -> ()>,
            pthread_startroutine_t,
        >(Some(SpyTopLevel as fn(pthread_addr_t) -> ())),
        0 ,
    ) != 0
    {
        vpunt(
            b"spy\0"   as&str,
            b"Unable to create the spy thread\0"
                as&str,
        );
    }
    spyThreadSetup = true;
}

pub  fn ReleaseSpyLock() {
    if pthread_mutex_unlock(&mut spyLock) != 0 {
        vpunt(
            b"spy\0"   as&str,
            b"Unable to unlock the spy lock in thread %x\0"
                 as&str,
            pthread_self(),
        );
    }
}

pub  fn SendMBINBuffers(mut mbinChannel: *mut EmbMBINChannel) {
    let mut gthQ: *mut EmbQueue = (*mbinChannel).guestToHostQueue;
    let mut gthrQ: *mut EmbQueue = (*mbinChannel).guestToHostReturnQueue;
    let mut bufferPtr: EmbPtr = 0;
    let mut buffer: *mut rm_aligned_pkt = 0 as *mut rm_aligned_pkt;
    let mut pkt: rm_pkt = rm_pkt {
        rm_pad: [0; 2],
        rm_id: [0; 4],
        rm_operand: [0; 3],
        rm_opcode: [0; 1],
        data: [0; 1284],
    };
    let mut nBytes: libc::c_uint = 0;
    let mut id: libc::c_uint = 0;
    let mut historyID: u32 = 0;
    if (*(*mbinChannel).header.messageChannel).guestToHostImpulse != 0 {
        match (*(*mbinChannel).header.messageChannel).guestToHostImpulse {
            1 => {
                activeMBINChannel = 0 as *mut EmbMBINChannel;
                ResetIncomingQueue(gthQ);
                ResetOutgoingQueue(gthrQ);
                ResetIncomingQueue((*mbinChannel).hostToGuestSupplyQueue);
                ResetOutgoingQueue((*mbinChannel).hostToGuestQueue);
                (*(*mbinChannel).header.messageChannel)
                    .guestToHostImpulse = EmbMessageImpulseNone;
                UnthreadMessageChannel((*mbinChannel).header.messageChannel);
                free(mbinChannel );
                return;
            }
            _ => {
                (*(*mbinChannel).header.messageChannel)
                    .guestToHostImpulse = EmbMessageImpulseNone;
            }
        }
    }
    while EmbQueueFilled(gthQ) != 0 {
        if 0  == EmbQueueSpace(gthrQ) {
            SignalLater((*gthQ).signal);
            return;
        }
        bufferPtr = EmbQueueTakeWord(gthQ);
        if bufferPtr != 0 && bufferPtr != -(1)
            && mbin_sinValid  != 0
        {
            buffer = &mut *(EmbCommAreaPtr as *mut EmbWord).offset(bufferPtr )
                as *mut EmbWord as PtrV as *mut rm_aligned_pkt;
            nBytes = read_long(
                &mut *((*buffer).rm_operand)
                    .as_mut_ptr()
                    .offset(0 ),
            ) & 0xffffff;
            memcpy(
                &mut *(pkt.rm_id).as_mut_ptr().offset(0 )
                    as *mut libc::c_uchar ,
                &mut *((*buffer).rm_id).as_mut_ptr().offset(0 )
                    as *mut libc::c_uchar ,
                8,
            );
            memcpy(
                &mut *(pkt.data).as_mut_ptr().offset(0 )
                    as *mut libc::c_uchar ,
                &mut *((*buffer).data).as_mut_ptr().offset(0 )
                    as *mut libc::c_uchar ,
                nBytes,
            );
            if rm_ack  == (*buffer).rm_opcode() {
                id = read_long(
                    &mut *((*buffer).rm_id)
                        .as_mut_ptr()
                        .offset(0 ),
                );
                historyID = (id & 0xf);
                MBINHistory[historyID ].id = id;
                MBINHistory[historyID ].acked = true;
            }
            spy_transmit(
                &mut pkt,
                (10).wrapping_add(nBytes),
                &mut mbin_sin,
            );
        }
        EmbQueuePutWord(gthrQ, bufferPtr);
    }
}

pub  fn TerminateSpy() {
    let mut killSleep: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
    let mut exit_code: *mut libc::c_void = 0 ;
    if (pthread_getspecific(mainThread)).is_null() {
        return;
    }
    if spyThreadSetup != 0 {
        pthread_cancel(spyThread);
        killSleep.tv_sec = 1  as __time_t;
        killSleep.tv_nsec = 250000000  as __syscall_slong_t;
        pthread_delay_np(&mut killSleep);
        pthread_join(spyThread, &mut exit_code);
        spyThreadSetup = false;
    }
    if spyLockSetup != 0 {
        pthread_mutex_destroy(&mut spyLock);
        spyLockSetup = false;
    }
    if spy != -(1) {
        close(spy);
        spy = -(1);
    }
}
