#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> u32;
    fn sched_yield() -> u32;
    fn pthread_detach(__th: u64) -> u32;
    fn pthread_self() -> u64;
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
    fn pthread_delay_np(interval: *const timespec) -> u32;
    fn pthread_get_expiration_np(
        delta: *const timespec,
        abstime: *mut timespec,
    ) -> u32;
    fn EmbSendSignal(signal: SignalNumber);
    static mut EmbCommAreaPtr: *mut EmbCommArea;
    fn ResetColdLoadChannel(channel: *mut EmbChannel);
    fn UpdateColdLoadNames();
    fn ResetConsoleChannel(channel: *mut EmbChannel);
    fn ResetDiskChannel(channel: *mut EmbChannel);
    fn PollMessageChannels();
    fn ResetMessageChannel(channel: *mut EmbChannel);
    fn ResetNetworkChannel(channel: *mut EmbChannel);
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
    fn(*mut libc::c_void) -> (),
>;
pub type EmbWord = i32;
pub type UEmbWord = u32;
pub type EmbPtr = EmbWord;
pub type SignalMask = UEmbWord;
pub type SignalNumber = EmbWord;
pub type PtrV = *mut libc::c_void;
pub type ProcPtrV = Option::<fn(PtrV) -> ()>;
pub type isize = u64;
pub type Boole = u8;
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
    pub generaVersion: C2RustUnnamed_3,
    pub osfVersion: C2RustUnnamed_2,
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
    pub fep: C2RustUnnamed_1,
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
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    #[bitfield(name = "data", ty = "EmbWord", bits = "0..=15")]
    #[bitfield(name = "c2rust_unnamed", ty = "EmbWord", bits = "16..=31")]
    pub data_c2rust_unnamed: [u8; 4],
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
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
pub struct C2RustUnnamed_2 {
    #[bitfield(name = "minorRevision", ty = "EmbWord", bits = "0..=7")]
    #[bitfield(name = "majorRevision", ty = "EmbWord", bits = "8..=15")]
    #[bitfield(name = "minorRelease", ty = "EmbWord", bits = "16..=23")]
    #[bitfield(name = "majorRelease", ty = "EmbWord", bits = "24..=30")]
    #[bitfield(name = "testReleaseP", ty = "EmbWord", bits = "31..=31")]
    pub minorRevision_majorRevision_minorRelease_majorRelease_testReleaseP: [u8; 4],
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    #[bitfield(name = "minor", ty = "EmbWord", bits = "0..=15")]
    #[bitfield(name = "major", ty = "EmbWord", bits = "16..=31")]
    pub minor_major: [u8; 4],
}
pub type ResetRequest = u32;
pub const LispResetRequest: ResetRequest = 1;
pub const NoResetRequest: ResetRequest = 0;
pub const FEPResetRequest: ResetRequest = -1;
pub const DevicePROMResetRequest: ResetRequest = -2;
pub const BootResetRequest: ResetRequest = -3;
pub const AreYouThereResetRequest: ResetRequest = -4;
pub const WriteNVRAMResetRequest: ResetRequest = -5;
pub const ReadNVRAMResetRequest: ResetRequest = -6;
pub type GuestStatus = u32;
pub const RunningGuestStatus: GuestStatus = 5;
pub const CrashedGuestStatus: GuestStatus = 4;
pub const StartedGuestStatus: GuestStatus = 3;
pub const InitializedGuestStatus: GuestStatus = 2;
pub const InitializingGuestStatus: GuestStatus = 1;
pub const UninitializedGuestStatus: GuestStatus = 0;
pub const BrokenGuestStatus: GuestStatus = -1;
pub const NonexistentGuestStatus: GuestStatus = -2;
pub type FEPStatus = libc::c_uint;
pub const IdleFEPStatus: FEPStatus = 1;
pub const RunningFEPStatus: FEPStatus = 0;
pub const HaltedFEPStatus: FEPStatus = 255;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct EmbChannel {
    pub type_0: EmbWord,
    pub unit: EmbWord,
    pub next: EmbPtr,
}
pub type EmbChannelType = libc::c_uint;
pub const EmbMessageChannelType: EmbChannelType = 8;
pub const EmbHostFileChannelType: EmbChannelType = 7;
pub const EmbColdLoadChannelType: EmbChannelType = 6;
pub const EmbSCSIChannelType: EmbChannelType = 5;
pub const EmbRPCChannelType: EmbChannelType = 4;
pub const EmbNetworkChannelType: EmbChannelType = 3;
pub const EmbConsoleChannelType: EmbChannelType = 2;
pub const EmbDiskChannelType: EmbChannelType = 1;
 fn VLMIsRunning(mut ep: *mut EmbCommArea) -> Boole {
    return ((*ep).spy_status == 0
        && ((*ep).fep).status()  != HaltedFEPStatus)
        as bool;
}
 fn VLMIsRunningLisp(mut ep: *mut EmbCommArea) -> Boole {
    return (VLMIsRunning(ep)  != 0
        && ((*ep).fep).status()  == IdleFEPStatus)
        as bool;
}
 fn UpdateVLMStatus() {
    let mut ep: *mut EmbCommArea = EmbCommAreaPtr;
    match (*ep).guestStatus {
        0 | 1 => {
            (*ep)
                .guestStatus = if VLMIsRunningLisp(ep)  != 0 {
                RunningGuestStatus
            } else if VLMIsRunning(ep)  != 0 {
                InitializedGuestStatus
            } else {
                (*ep).guestStatus
            };
        }
        2 | 3 => {
            (*ep)
                .guestStatus = if VLMIsRunningLisp(ep)  != 0 {
                RunningGuestStatus
            } else if VLMIsRunning(ep)  != 0 {
                (*ep).guestStatus
            } else {
                InitializingGuestStatus
            };
        }
        4 | 5 => {
            (*ep)
                .guestStatus = if VLMIsRunningLisp(ep)  != 0 {
                RunningGuestStatus
            } else if VLMIsRunning(ep)  != 0 {
                CrashedGuestStatus
            } else {
                InitializingGuestStatus
            };
        }
        -2 | -1 | _ => {}
    }
    UpdateColdLoadNames();
}
 fn ResetCommArea(mut fullReset: bool) {
    let mut channel: *mut EmbChannel = 0 as *mut EmbChannel;
    let mut channelP: EmbPtr = 0;
    channelP = (*EmbCommAreaPtr).channel_table;
    while channelP != -(1) {
        channel = &mut *(EmbCommAreaPtr as *mut EmbWord).offset(channelP )
            as *mut EmbWord as PtrV as *mut EmbChannel;
        match (*channel).type_0 {
            1 => {
                if fullReset != 0 {
                    ResetDiskChannel(channel);
                }
            }
            2 => {
                if fullReset != 0 {
                    ResetConsoleChannel(channel);
                }
            }
            3 => {
                if fullReset != 0 {
                    ResetNetworkChannel(channel);
                }
            }
            6 => {
                ResetColdLoadChannel(channel);
            }
            8 => {
                if fullReset != 0 {
                    ResetMessageChannel(channel);
                }
            }
            _ => {}
        }
        channelP = (*channel).next;
    }
    fullReset != 0;
}
 fn ProcessResetRequest() {
    match (*EmbCommAreaPtr).reset_request {
        -6 => {}
        -5 => {}
        -4 => {}
        -1 => {
            ResetCommArea(false);
        }
        1 => {
            ResetCommArea(true);
            let ref mut fresh0 = (*EmbCommAreaPtr).resetRequestCount;
            *fresh0 += 1;
            (*EmbCommAreaPtr).restart_applications = 1;
        }
        _ => {}
    }
    (*EmbCommAreaPtr).reset_request = NoResetRequest;
}

pub  fn IvoryLifePolling(mut argument: pthread_addr_t) {
    let mut self_0: u64 = pthread_self();
    let mut pollingSleep: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
    pollingSleep.tv_sec = 0  as __time_t;
    pollingSleep.tv_nsec = 0  as __syscall_slong_t;
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
    loop {
        let mut __cancel_buf_0: __pthread_unwind_buf_t = __pthread_unwind_buf_t {
            __cancel_jmp_buf: [__cancel_jmp_buf_tag {
                __cancel_jmp_buf: [0; 8],
                __mask_was_saved: 0,
            }; 1],
            __pad: [0 ; 4],
        };
        let mut __cancel_routine_0: Option::<
            fn(*mut libc::c_void) -> (),
        > = ::std::mem::transmute::<
            Option::<fn(*mut u64) -> u32>,
            pthread_cleanuproutine_t,
        >(
            Some(
                pthread_mutex_unlock
                    as fn(*mut u64) -> u32,
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
        (*EmbCommAreaPtr).pollTime += pollingSleep.tv_nsec;
        PollMessageChannels();
        if (*EmbCommAreaPtr).reset_request != NoResetRequest  {
            ProcessResetRequest();
        } else if (*EmbCommAreaPtr).pollTime > 250000000  {
            (*EmbCommAreaPtr).pollTime = 0  ;
            let ref mut fresh1 = (*EmbCommAreaPtr).guest_to_host_signals;
            *fresh1 |= (*EmbCommAreaPtr).live_guest_to_host_signals;
            if pthread_cond_broadcast(&mut (*EmbCommAreaPtr).signalSignal) != 0 {
                vpunt(

                    b"Unable to send Life Support signal signal in thread %lx\0"
                          as&str,
                    self_0,
                );
            }
        } else if (*EmbCommAreaPtr).reawaken != 0 {
            let ref mut fresh2 = (*EmbCommAreaPtr).guest_to_host_signals;
            *fresh2 |= (*EmbCommAreaPtr).reawaken;
            (*EmbCommAreaPtr).reawaken = 0  as SignalMask;
            if pthread_cond_broadcast(&mut (*EmbCommAreaPtr).signalSignal) != 0 {
                vpunt(

                    b"Unable to send Life Support signal signal in thread %lx\0"
                          as&str,
                    self_0,
                );
            }
        }
        if pthread_mutex_unlock(&mut (*EmbCommAreaPtr).signalLock) != 0 {
            vpunt(

                b"Unable to unlock the Life Support signalLock in thread %lx\0"
                      as&str,
                pthread_self(),
            );
        }
        __pthread_unregister_cancel(&mut __cancel_buf_0);
        if (*EmbCommAreaPtr).clock_interval > 0  {
            (*EmbCommAreaPtr).pollClockTime -= pollingSleep.tv_nsec;
            if (*EmbCommAreaPtr).pollClockTime <= 0   {
                EmbSendSignal((*EmbCommAreaPtr).clock_signal);
                (*EmbCommAreaPtr)
                    .pollClockTime = (1000
                    * (*EmbCommAreaPtr).clock_interval) ;
            }
            if (*EmbCommAreaPtr).pollClockTime > 250000000  {
                pollingSleep.tv_nsec = 250000000 ;
            } else {
                pollingSleep.tv_nsec = (*EmbCommAreaPtr).pollClockTime;
            }
        } else {
            pollingSleep.tv_nsec = 7500000 ;
        }
        UpdateVLMStatus();
        pollingSleep.tv_sec = 1  as __time_t;
        pollingSleep.tv_nsec = 0  as __syscall_slong_t;
        if pthread_delay_np(&mut pollingSleep) != 0 {
            vpunt(

                b"Unable to sleep in thread %lx\0"
                    as&str,
                self_0,
            );
        }
    };
}

pub  fn IntervalTimerDriver(mut argument: pthread_addr_t) {
    let mut self_0: u64 = pthread_self();
    let mut expirationTime: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
    let mut expirationInterval: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
    let mut result: u32 = 0;
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
    if pthread_mutex_lock(&mut (*EmbCommAreaPtr).signalLock) != 0 {
        vpunt(

            b"Unable to lock the Life Support signal lock in thread %lx\0"
                 as&str,
            pthread_self(),
        );
    }
    if pthread_mutex_unlock(&mut (*EmbCommAreaPtr).signalLock) != 0 {
        vpunt(

            b"Unable to unlock the Life Support signal lock in thread %lx\0"
                 as&str,
            pthread_self(),
        );
    }
    (*EmbCommAreaPtr).clockTime = -(1) ;
    let mut __cancel_buf_0: __pthread_unwind_buf_t = __pthread_unwind_buf_t {
        __cancel_jmp_buf: [__cancel_jmp_buf_tag {
            __cancel_jmp_buf: [0; 8],
            __mask_was_saved: 0,
        }; 1],
        __pad: [0 ; 4],
    };
    let mut __cancel_routine_0: Option::<
        fn(*mut libc::c_void) -> (),
    > = ::std::mem::transmute::<
        Option::<fn(*mut u64) -> u32>,
        pthread_cleanuproutine_t,
    >(
        Some(
            pthread_mutex_unlock
                as fn(*mut u64) -> u32,
        ),
    );
    let mut __cancel_arg_0: *mut libc::c_void = &mut (*EmbCommAreaPtr).clockLock
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
    if pthread_mutex_lock(&mut (*EmbCommAreaPtr).clockLock) != 0 {
        vpunt(

            b"Unable to lock the Life Support clockLock in thread %lx\0"
                 as&str,
            pthread_self(),
        );
    }
    loop {
        if (*EmbCommAreaPtr).clockTime >= 0   {
            expirationInterval.tv_sec = 0  as __time_t;
            expirationInterval
                .tv_nsec = 1000
                * (*EmbCommAreaPtr).clockTime;
            while expirationInterval.tv_nsec >= 1000000000  {
                expirationInterval.tv_sec += 1;
                expirationInterval.tv_nsec -= 1000000000 ;
            }
            if pthread_get_expiration_np(&mut expirationInterval, &mut expirationTime)
                < 0
            {
                vpunt(

                    b"Unable to compute interval timer expiration time\0"
                         as&str,
                );
            }
            result = u64imedwait(
                &mut (*EmbCommAreaPtr).clockSignal,
                &mut (*EmbCommAreaPtr).clockLock,
                &mut expirationTime,
            );
        } else {
            result = pthread_cond_wait(
                &mut (*EmbCommAreaPtr).clockSignal,
                &mut (*EmbCommAreaPtr).clockLock,
            );
        }
        if result == 110  {
            EmbSendSignal((*EmbCommAreaPtr).clock_signal);
            (*EmbCommAreaPtr).clockTime = -(1) ;
        }
    };
}

pub  fn SetIntervalTimer(mut relativeTimeout: isize) {
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
    let mut __cancel_arg: *mut libc::c_void = &mut (*EmbCommAreaPtr).clockLock
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
    if pthread_mutex_lock(&mut (*EmbCommAreaPtr).clockLock) != 0 {
        vpunt(

            b"Unable to lock the Life Support clockLock in thread %lx\0"
                 as&str,
            pthread_self(),
        );
    }
    (*EmbCommAreaPtr).clockTime = relativeTimeout ;
    if pthread_cond_broadcast(&mut (*EmbCommAreaPtr).clockSignal) < 0  {
        vpunt(

            b"Unable to send Life Support clock signal in thread %lx\0"
                 as&str,
            pthread_self(),
        );
    }
    if pthread_mutex_unlock(&mut (*EmbCommAreaPtr).clockLock) != 0 {
        vpunt(

            b"Unable to unlock the Life Support clockLock in thread %lx\0"
                 as&str,
            pthread_self(),
        );
    }
    __pthread_unregister_cancel(&mut __cancel_buf);
    sched_yield();
}
