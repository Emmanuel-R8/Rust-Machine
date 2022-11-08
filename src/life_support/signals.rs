#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    fn pthread_get_expiration_np(
        delta: *const timespec,
        abstime: *mut timespec,
    ) -> u32;
    fn pthread_create(
        __newthread: *mut u64,
        __attr: *const u64,
        __start_routine: Option::<
            unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
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
    fn pthread_mutex_lock(__mutex: *mut u64) -> u32;
    fn pthread_mutex_unlock(__mutex: *mut u64) -> u32;
    fn pthread_cond_broadcast(__cond: *mut u64) -> u32;
    fn pthread_cond_wait(
        __cond: *mut u64,
        __mutex: *mut u64,
    ) -> u32;
    fn u64imedwait(
        __cond: *mut u64,
        __mutex: *mut u64,
        __abstime: *const timespec,
    ) -> u32;
    static mut processor: *mut ProcessorState;
    fn SendInterruptToEmulator();
    static mut EmbCommAreaPtr: *mut EmbCommArea;
    fn vpunt(section:&str, format:&str, _: ...);
}
pub type u8 = libc::c_uchar;
pub type i32 = u32;
pub type u32 = libc::c_uint;
pub type u64 = libc::c_ulong;
pub type __time_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type __u64 =&str;
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
pub type pthread_addr_t = *mut libc::c_void;
pub type pthread_cleanuproutine_t = Option::<
    unsafe extern "C" fn(*mut libc::c_void) -> (),
>;
pub type pthread_startroutine_t = Option::<
    unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
>;
pub type EmbWord = i32;
pub type UEmbWord = u32;
pub type EmbPtr = EmbWord;
pub type SignalMask = UEmbWord;
pub type SignalNumber = EmbWord;
pub type PtrV = *mut libc::c_void;
pub type ProcPtrV = Option::<unsafe extern "C" fn(PtrV) -> ()>;
pub type isize = u64;
pub type Boole = u8;
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
    pub data: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub u: u32,
    pub s: i32,
    pub f: libc::c_float,
}
pub type PC = LispObj;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct InstructionCacheLine {
    pub pc: PC,
    pub next_pc: PC,
    pub code: u32,
    pub operand: u32,
    pub instruction: libc::c_uint,
    pub next_cp: *mut InstructionCacheLine,
}
pub type InstructionCacheLine = InstructionCacheLine;
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
    pub ListCacheArea: QWord,
    pub ListCacheAddress: QWord,
    pub StructureCacheArea: QWord,
    pub StructureCacheAddress: QWord,
    pub CatchBlockPointer: QWord,
    pub control: isize,
    pub StackCacheBase: isize,
    pub ArrayEventCount: isize,
    pub ListCacheLength: isize,
    pub StructureCacheLength: isize,
    pub BindingStackPointer: isize,
    pub BindingStackLimit: isize,
    pub DeepBoundP: bool,
    pub PreemptRegister: isize,
    pub AluAndRotateControl: isize,
    pub AluOp: Option::<unsafe extern "C" fn() -> isize>,
    pub ByteSize: isize,
    pub ByteRotate: isize,
    pub RotateLatch: isize,
    pub ALUOverflow: bool,
    pub ALUBorrow: bool,
    pub ALULessThan: bool,
    pub EphemeralOldspaceRegister: isize,
    pub ZoneOldspaceRegister: isize,
    pub ControlStackLimit: isize,
    pub ControlStackExtraLimit: isize,
    pub DynamicBindingCacheBase: isize,
    pub DynamicBindingCacheMask: isize,
    pub FEPModeTrapVectorAddress: isize,
    pub MappingTableCache: isize,
    pub MappingTableLength: isize,
    pub running: bool,
    pub instruction_count: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _bar {
    pub address: QWord,
    pub mapped: *mut LispObj,
}
pub type ProcessorState = _ProcessorState;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct EmbCommArea {
    pub identifier: EmbWord,
    pub version: EmbWord,
    pub system_type: EmbWord,
    pub number_of_slots: EmbWord,
    pub comm_memory_size: EmbWord,
    pub generaVersion: C2RustUnnamed_4,
    pub osfVersion: C2RustUnnamed_3,
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
    pub fep: C2RustUnnamed_2,
    pub restart_applications: EmbWord,
    pub signal_interrupt_vector: EmbWord,
    pub base_register: EmbWord,
    pub hostVersion2: EmbWord,
    pub hostVersion3: EmbWord,
    pub MacIvory_NVRAM_settings: C2RustUnnamed_1,
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
pub struct SignalHandler {
    pub handlerThread: u64,
    pub handlerThreadSetup: bool,
    pub signal: SignalMask,
    pub handlerFunction: ProcPtrV,
    pub handlerArgument: PtrV,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    #[bitfield(name = "data", ty = "EmbWord", bits = "0..=15")]
    #[bitfield(name = "c2rust_unnamed", ty = "EmbWord", bits = "16..=31")]
    pub data_c2rust_unnamed: [u8; 4],
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
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
pub struct C2RustUnnamed_3 {
    #[bitfield(name = "minorRevision", ty = "EmbWord", bits = "0..=7")]
    #[bitfield(name = "majorRevision", ty = "EmbWord", bits = "8..=15")]
    #[bitfield(name = "minorRelease", ty = "EmbWord", bits = "16..=23")]
    #[bitfield(name = "majorRelease", ty = "EmbWord", bits = "24..=30")]
    #[bitfield(name = "testReleaseP", ty = "EmbWord", bits = "31..=31")]
    pub minorRevision_majorRevision_minorRelease_majorRelease_testReleaseP: [u8; 4],
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    #[bitfield(name = "minor", ty = "EmbWord", bits = "0..=15")]
    #[bitfield(name = "major", ty = "EmbWord", bits = "16..=31")]
    pub minor_major: [u8; 4],
}

pub unsafe extern "C" fn InitializeSignalHandlers() {
    let mut i: u32 = 0;
    (*EmbCommAreaPtr).guest_to_host_signals = 0  as SignalMask;
    (*EmbCommAreaPtr).live_guest_to_host_signals = 0  as SignalMask;
    (*EmbCommAreaPtr).host_to_guest_signals = 0  as SignalMask;
    (*EmbCommAreaPtr).live_host_to_guest_signals = 0  as SignalMask;
    (*EmbCommAreaPtr).reawaken = 0  as SignalMask;
    (*EmbCommAreaPtr).useSignalLocks = false;
    i = 0;
    while i < 32  {
        (*EmbCommAreaPtr)
            .signalHandler[i ]
            .handlerThreadSetup = false;
        (*EmbCommAreaPtr)
            .signalHandler[i ]
            .signal = 0  as SignalMask;
        let ref mut fresh0 = (*EmbCommAreaPtr).signalHandler[i ].handlerFunction;
        *fresh0 = None;
        let ref mut fresh1 = (*EmbCommAreaPtr).signalHandler[i ].handlerArgument;
        *fresh1 = 0 ;
        i += 1;
    }
}

pub unsafe extern "C" fn InstallSignalHandler(
    mut signalHandler: ProcPtrV,
    mut signalArgument: PtrV,
    mut inputP: bool,
) -> SignalNumber {
    let mut policy: u32 = 0;
    let mut priority: u32 = 0;
    let mut i: u32 = 0;
    let mut signal: SignalMask = 0;
    if (*EmbCommAreaPtr).useSignalLocks != 0 {
        if pthread_mutex_lock(&mut (*EmbCommAreaPtr).signalLock) != 0 {
            vpunt(

                b"Unable to lock the Life Support signal lock in thread %lx\0"
                      as&str,
                pthread_self(),
            );
        }
    }
    i = 0;
    while i < 32  {
        signal = ((1) << i) as SignalMask;
        if (*EmbCommAreaPtr).live_guest_to_host_signals & signal
            == 0
        {
            let ref mut fresh2 = (*EmbCommAreaPtr).live_guest_to_host_signals;
            *fresh2 |= signal;
            (*EmbCommAreaPtr).signalHandler[i ].signal = signal;
            let ref mut fresh3 = (*EmbCommAreaPtr)
                .signalHandler[i ]
                .handlerFunction;
            *fresh3 = signalHandler;
            let ref mut fresh4 = (*EmbCommAreaPtr)
                .signalHandler[i ]
                .handlerArgument;
            *fresh4 = signalArgument;
            if (*EmbCommAreaPtr).signalHandler[i ].handlerThreadSetup == 0 {
                if pthread_create(
                    &mut (*((*EmbCommAreaPtr).signalHandler)
                        .as_mut_ptr()
                        .offset(i ))
                        .handlerThread,
                    if inputP  != 0 {
                        &mut (*EmbCommAreaPtr).inputThreadAttrs
                    } else {
                        &mut (*EmbCommAreaPtr).outputThreadAttrs
                    },
                    ::std::mem::transmute::<
                        Option::<unsafe extern "C" fn(pthread_addr_t) -> ()>,
                        pthread_startroutine_t,
                    >(
                        Some(
                            SignalHandlerTopLevel
                                as unsafe extern "C" fn(pthread_addr_t) -> (),
                        ),
                    ),
                    &mut *((*EmbCommAreaPtr).signalHandler)
                        .as_mut_ptr()
                        .offset(i ) as *mut SignalHandler ,
                ) != 0
                {
                    vpunt(

                        b"Unable to create thread to handle signal %d for %lx (%lx)\0"
                              as&str,
                        i,
                        signalHandler,
                        signalArgument,
                    );
                }
                (*EmbCommAreaPtr)
                    .signalHandler[i ]
                    .handlerThreadSetup = true;
            }
            break;
        } else {
            i += 1;
        }
    }
    if (*EmbCommAreaPtr).useSignalLocks != 0 {
        if pthread_mutex_unlock(&mut (*EmbCommAreaPtr).signalLock) != 0 {
            vpunt(

                b"Unable to unlock the Life Support signal lock in thread %lx\0"
                      as&str,
                pthread_self(),
            );
        }
    }
    if i < 32  {
        return i
    } else {
        vpunt(

            b"Signal table overflow\0"
                as&str,
        );
    }
    panic!("Reached end of non-void function without returning");
}

pub unsafe extern "C" fn SendInterruptToLifeSupport() {
    if pthread_cond_broadcast(&mut (*EmbCommAreaPtr).signalSignal) != 0 {
        vpunt(

            b"Unable to send Life Support an interrupt from the VLM\0"
                 as&str,
        );
    }
}

pub unsafe extern "C" fn WaitForLifeSupport() {
    let mut delta: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
    let mut abstime: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
    let mut result: u32 = 0;
    delta.tv_sec = 5  as __time_t;
    delta.tv_nsec = 0  as __syscall_slong_t;
    if (*EmbCommAreaPtr).host_to_guest_signals != 0
        && (*processor).control >> 30  & 3
            != 3
    {
        SendInterruptToEmulator();
    } else {
        let mut __cancel_buf: __pthread_unwind_buf_t = __pthread_unwind_buf_t {
            __cancel_jmp_buf: [__cancel_jmp_buf_tag {
                __cancel_jmp_buf: [0; 8],
                __mask_was_saved: 0,
            }; 1],
            __pad: [0 ; 4],
        };
        let mut __cancel_routine: Option::<
            unsafe extern "C" fn(*mut libc::c_void) -> (),
        > = ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut u64) -> u32>,
            pthread_cleanuproutine_t,
        >(
            Some(
                pthread_mutex_unlock
                    as unsafe extern "C" fn(*mut u64) -> u32,
            ),
        );
        let mut __cancel_arg: *mut libc::c_void = &mut (*EmbCommAreaPtr).wakeupLock
            as *mut u64 ;
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
        if pthread_mutex_lock(&mut (*EmbCommAreaPtr).wakeupLock) != 0 {
            vpunt(

                b"Unable to lock the VLM wakeup lock in thread %lx\0"
                     as&str,
                pthread_self(),
            );
        }
        if pthread_get_expiration_np(&mut delta, &mut abstime) != 0 {
            vpunt(

                b"Unable to get absolute time\0"
                    as&str,
            );
        }
        result = u64imedwait(
            &mut (*EmbCommAreaPtr).wakeupSignal,
            &mut (*EmbCommAreaPtr).wakeupLock,
            &mut abstime,
        );
        if result != 0 {
            if !(result == 110  || result == 4) {
                vpunt(

                    b"Unable to wait for a VLM wakeup signal in thread %lx\0"
                          as&str,
                    pthread_self(),
                );
            }
        }
        if pthread_mutex_unlock(&mut (*EmbCommAreaPtr).wakeupLock) != 0 {
            vpunt(

                b"Unable to unlock the VLM wakeup lock in thread %lx\0"
                     as&str,
                pthread_self(),
            );
        }
        __pthread_unregister_cancel(&mut __cancel_buf);
    };
}

pub unsafe extern "C" fn EmbSendSignal(mut signal: SignalNumber) {
    signal == 0;
    if pthread_mutex_lock(&mut (*EmbCommAreaPtr).wakeupLock) != 0 {
        vpunt(

            b"Unable to lock the VLM wakeup lock in thread %lx\0"
                 as&str,
            pthread_self(),
        );
    }
    if signal > -(1) && signal < 32  {
        let ref mut fresh5 = (*EmbCommAreaPtr).host_to_guest_signals;
        *fresh5 |= ((1) << signal) ;
        SendInterruptToEmulator();
    }
    if pthread_cond_broadcast(&mut (*EmbCommAreaPtr).wakeupSignal) != 0 {
        vpunt(

            b"Unable to wakeup the VLM from Life Support\0"
                 as&str,
        );
    }
    if pthread_mutex_unlock(&mut (*EmbCommAreaPtr).wakeupLock) != 0 {
        vpunt(

            b"Unable to unlock the VLM wakeup lock in thread %lx\0"
                 as&str,
            pthread_self(),
        );
    }
}

pub unsafe extern "C" fn SignalLater(mut signal: SignalNumber) {
    let mut self_0: u64 = pthread_self();
    if pthread_mutex_lock(&mut (*EmbCommAreaPtr).signalLock) != 0 {
        vpunt(

            b"Unable to lock the Life Support signal lock in thread %lx\0"
                 as&str,
            self_0,
        );
    }
    let ref mut fresh6 = (*EmbCommAreaPtr).reawaken;
    *fresh6 |= ((1) << signal) as SignalMask;
    if pthread_mutex_unlock(&mut (*EmbCommAreaPtr).signalLock) != 0 {
        vpunt(

            b"Unable to unlock the Life Support signal lock in thread %lx\0"
                 as&str,
            self_0,
        );
    }
}
unsafe extern "C" fn NullSignalHandler(mut ignore: PtrV) {}

pub unsafe extern "C" fn RemoveSignalHandler(mut signal: SignalNumber) {
    let mut mask: SignalMask = ((1) << signal) as SignalMask;
    if signal < 0  || signal >= 32  {
        return;
    }
    if (*EmbCommAreaPtr).useSignalLocks != 0 {
        if pthread_mutex_lock(&mut (*EmbCommAreaPtr).signalLock) != 0 {
            vpunt(

                b"Unable to lock the Life Support signal lock in thread %lx\0"
                      as&str,
                pthread_self(),
            );
        }
    }
    let ref mut fresh7 = (*EmbCommAreaPtr).live_guest_to_host_signals;
    *fresh7 &= !mask;
    let ref mut fresh8 = (*EmbCommAreaPtr).reawaken;
    *fresh8 &= !mask;
    let ref mut fresh9 = (*EmbCommAreaPtr).guest_to_host_signals;
    *fresh9 |= mask;
    if (*EmbCommAreaPtr).signalHandler[signal ].handlerThreadSetup != 0 {
        let ref mut fresh10 = (*EmbCommAreaPtr)
            .signalHandler[signal ]
            .handlerFunction;
        *fresh10 = Some(NullSignalHandler as unsafe extern "C" fn(PtrV) -> ());
        let ref mut fresh11 = (*EmbCommAreaPtr)
            .signalHandler[signal ]
            .handlerArgument;
        *fresh11 = 0 ;
    }
    if (*EmbCommAreaPtr).useSignalLocks != 0 {
        if pthread_mutex_unlock(&mut (*EmbCommAreaPtr).signalLock) != 0 {
            vpunt(

                b"Unable to unlock the Life Support signal lock in thread %lx\0"
                      as&str,
                pthread_self(),
            );
        }
    }
}

pub unsafe extern "C" fn TerminateSignalHandlers() {
    let mut i: u32 = 0;
    let mut exit_value: *mut libc::c_void = 0 ;
    i = 0;
    while i < 32  {
        if (*EmbCommAreaPtr).signalHandler[i ].handlerThreadSetup != 0 {
            pthread_cancel((*EmbCommAreaPtr).signalHandler[i ].handlerThread);
            pthread_join(
                (*EmbCommAreaPtr).signalHandler[i ].handlerThread,
                &mut exit_value,
            );
            (*EmbCommAreaPtr)
                .signalHandler[i ]
                .handlerThreadSetup = false;
        }
        i += 1;
    }
}
unsafe extern "C" fn SignalHandlerTopLevel(mut argument: pthread_addr_t) {
    let mut signalHandler: *mut SignalHandler = argument as *mut SignalHandler;
    let mut self_0: u64 = (*signalHandler).handlerThread;
    let mut __cancel_buf: __pthread_unwind_buf_t = __pthread_unwind_buf_t {
        __cancel_jmp_buf: [__cancel_jmp_buf_tag {
            __cancel_jmp_buf: [0; 8],
            __mask_was_saved: 0,
        }; 1],
        __pad: [0 ; 4],
    };
    let mut __cancel_routine: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()> = ::std::mem::transmute::<
        Option::<unsafe extern "C" fn(u64) -> u32>,
        pthread_cleanuproutine_t,
    >(Some(pthread_detach as unsafe extern "C" fn(u64) -> u32));
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
    let mut __cancel_buf_0: __pthread_unwind_buf_t = __pthread_unwind_buf_t {
        __cancel_jmp_buf: [__cancel_jmp_buf_tag {
            __cancel_jmp_buf: [0; 8],
            __mask_was_saved: 0,
        }; 1],
        __pad: [0 ; 4],
    };
    let mut __cancel_routine_0: Option::<
        unsafe extern "C" fn(*mut libc::c_void) -> (),
    > = ::std::mem::transmute::<
        Option::<unsafe extern "C" fn(*mut u64) -> u32>,
        pthread_cleanuproutine_t,
    >(
        Some(
            pthread_mutex_unlock
                as unsafe extern "C" fn(*mut u64) -> u32,
        ),
    );
    let mut __cancel_arg_0: *mut libc::c_void = &mut (*EmbCommAreaPtr).signalLock
        as *mut u64 ;
    let mut __not_first_call_0: u32 = __sigsetjmp(
        (__cancel_buf_0.__cancel_jmp_buf).as_mut_ptr()
            as *mut __jmp_buf_tag,
        0,
    );
    if __not_first_call_0  != 0 {
        __cancel_routine_0.expect("non-null function pointer")(__cancel_arg_0);
        __pthread_unwind_next(&mut __cancel_buf_0);
    }
    __pthread_register_cancel(&mut __cancel_buf_0);
    if pthread_mutex_lock(&mut (*EmbCommAreaPtr).signalLock) != 0 {
        vpunt(

            b"Unable to lock the Life Support signalLock in thread %lx\0"
                 as&str,
            pthread_self(),
        );
    }
    loop {
        if (*EmbCommAreaPtr).guest_to_host_signals & (*signalHandler).signal != 0 {
            let ref mut fresh12 = (*EmbCommAreaPtr).guest_to_host_signals;
            *fresh12 &= !(*signalHandler).signal;
            u64estcancel();
            if pthread_mutex_unlock(&mut (*EmbCommAreaPtr).signalLock) != 0 {
                vpunt(

                    b"Unable to unlock the Life Support signal lock in thread %lx\0"
                          as&str,
                    self_0,
                );
            }
            (Some(
                ((*signalHandler).handlerFunction).expect("non-null function pointer"),
            ))
                .expect("non-null function pointer")((*signalHandler).handlerArgument);
            if pthread_mutex_lock(&mut (*EmbCommAreaPtr).signalLock) != 0 {
                vpunt(

                    b"Unable to lock the Life Support signal lock in thread %lx\0"
                          as&str,
                    self_0,
                );
            }
        } else if pthread_cond_wait(
            &mut (*EmbCommAreaPtr).signalSignal,
            &mut (*EmbCommAreaPtr).signalLock,
        ) != 0
        {
            vpunt(

                b"Unable to wait for the Life Support signal signal in thread %lx\0"
                      as&str,
                self_0,
            );
        }
    };
}
