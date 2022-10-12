#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn RemoveSignalHandler(signal: SignalNumber);
    fn EmbSendSignal(signal: SignalNumber);
    static mut EmbCommAreaPtr: *mut EmbCommArea;
    fn EmbCommAreaAlloc(nBytes: size_t) -> EmbPtr;
}
pub type size_t = libc::c_ulong;
pub type u8 = libc::c_uchar;
pub type i32 = u32;
pub type u32 = libc::c_uint;
pub type __caddr_t =&str;
pub type caddr_t = __caddr_t;
pub type i32 = i32;
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
pub type u8 = u8;
pub type ui32 = u32;
pub type EmbWord = i32;
pub type uEmbWord = ui32;
pub type EmbPtr = EmbWord;
pub type SignalMask = uEmbWord;
pub type SignalNumber = EmbWord;
pub type PtrV = *mut libc::c_void;
pub type ProcPtrV = Option::<fn(PtrV) -> ()>;
pub type Byte = u8;
pub type Boole = u8;
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
    #[bitfield(name = "status", ty = "uEmbWord", bits = "0..=7")]
    #[bitfield(name = "cursor", ty = "uEmbWord", bits = "8..=8")]
    #[bitfield(name = "busy", ty = "uEmbWord", bits = "9..=9")]
    #[bitfield(name = "error", ty = "uEmbWord", bits = "10..=10")]
    #[bitfield(name = "lisp_is_loaded", ty = "uEmbWord", bits = "11..=11")]
    #[bitfield(name = "c2rust_unnamed", ty = "uEmbWord", bits = "12..=31")]
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
#[no_mangle]
pub  fn CreateQueue(
    mut nElements: u32,
    mut elementSize: u32,
) -> EmbPtr {
    let mut cp: EmbPtr = EmbCommAreaAlloc(
        (::std::mem::size_of::<EmbQueue>() as libc::c_ulong)
            .wrapping_add((nElements * elementSize) as libc::c_ulong),
    );
    let mut q: *mut EmbQueue = &mut *(EmbCommAreaPtr as *mut EmbWord).offset(cp as isize)
        as *mut EmbWord as PtrV as *mut EmbQueue;
    (*q).element_size = elementSize;
    (*q).queue_size = nElements;
    (*q).put_index = 0;
    (*q).take_index = 0;
    (*q).signal = -(1);
    return cp;
}
#[no_mangle]
pub  fn EmbQueueSpace(mut qp: *mut EmbQueue) -> usize {
    let mut put: EmbWord = (*qp).put_index;
    let mut take: EmbWord = (*qp).take_index;
    if take > put {
        return take - put - 1
    } else {
        return take - put - 1 as usize + (*qp).queue_size
    };
}
#[no_mangle]
pub  fn EmbQueueFilled(mut qp: *mut EmbQueue) -> usize {
    let mut put: EmbWord = (*qp).put_index;
    let mut take: EmbWord = (*qp).take_index;
    if put >= take { return put - take } else { return put - take + (*qp).queue_size };
}
#[no_mangle]
pub  fn EmbQueuePut(mut qp_arg: *mut EmbQueue, mut ep: PtrV) {
    let mut qp: *mut EmbQueue = qp_arg;
    let mut put: EmbWord = (*qp).put_index;
    let mut original_put: EmbWord = put;
    let mut element_array: *mut Byte = &mut (*qp).first_element as *mut [EmbWord; 1]
        as *mut Byte;
    memcpy(
        &mut *element_array.offset((put * (*qp).element_size) as isize) as *mut Byte
            as *mut libc::c_void,
        ep as *const libc::c_void,
        (*qp).element_size as size_t,
    );
    put += 1;
    if put >= (*qp).queue_size {
        put = 0;
    }
    while put == (*qp).take_index {}
    (*qp).put_index = put;
    if original_put == (*qp).take_index {
        EmbSendSignal((*qp).signal);
    }
}
#[no_mangle]
pub  fn EmbQueuePutWord(mut qp_arg: *mut EmbQueue, mut elt: EmbWord) {
    let mut qp: *mut EmbQueue = qp_arg;
    let mut put: EmbWord = (*qp).put_index;
    let mut original_put: EmbWord = put;
    let mut element_array: *mut EmbWord = &mut (*qp).first_element as *mut [EmbWord; 1]
        as *mut EmbWord;
    *element_array.offset(put as isize) = elt;
    put += 1;
    if put >= (*qp).queue_size {
        put = 0;
    }
    while put == (*qp).take_index {}
    (*qp).put_index = put;
    if original_put == (*qp).take_index {
        EmbSendSignal((*qp).signal);
    }
}
#[no_mangle]
pub  fn EmbQueuePutByte(mut qp_arg: *mut EmbQueue, mut elt: Byte) {
    let mut qp: *mut EmbQueue = qp_arg;
    let mut put: EmbWord = (*qp).put_index;
    let mut original_put: EmbWord = put;
    let mut element_array: *mut Byte = &mut (*qp).first_element as *mut [EmbWord; 1]
        as *mut Byte;
    *element_array.offset(put as isize) = elt;
    put += 1;
    if put >= (*qp).queue_size {
        put = 0;
    }
    while put == (*qp).take_index {}
    (*qp).put_index = put;
    if original_put == (*qp).take_index {
        EmbSendSignal((*qp).signal);
    }
}
#[no_mangle]
pub  fn EmbQueueTake(
    mut qp_arg: *mut EmbQueue,
    mut ep: PtrV,
) -> Boole {
    let mut qp: *mut EmbQueue = qp_arg;
    let mut put: EmbWord = (*qp).put_index;
    let mut take: EmbWord = (*qp).take_index;
    let mut element_array: *mut Byte = &mut (*qp).first_element as *mut [EmbWord; 1]
        as *mut Byte;
    if put == take {
        return 0 as usize as Boole;
    }
    memcpy(
        ep,
        &mut *element_array.offset((take * (*qp).element_size) as isize) as *mut Byte
            as *const libc::c_void,
        (*qp).element_size as size_t,
    );
    take += 1;
    if take >= (*qp).queue_size {
        take = 0;
    }
    (*qp).take_index = take;
    return 1 as usize as Boole;
}
#[no_mangle]
pub  fn EmbQueueTakeWord(mut qp_arg: *mut EmbQueue) -> EmbWord {
    let mut qp: *mut EmbQueue = qp_arg;
    let mut put: EmbWord = (*qp).put_index;
    let mut take: EmbWord = (*qp).take_index;
    let mut element_array: *mut EmbWord = &mut (*qp).first_element as *mut [EmbWord; 1]
        as *mut EmbWord;
    let mut elt: EmbWord = 0;
    if put == take {
        return 0;
    }
    elt = *element_array.offset(take as isize);
    take += 1;
    if take >= (*qp).queue_size {
        take = 0;
    }
    (*qp).take_index = take;
    return elt;
}
#[no_mangle]
pub  fn EmbQueueTakeByte(mut qp_arg: *mut EmbQueue) -> Byte {
    let mut qp: *mut EmbQueue = qp_arg;
    let mut put: EmbWord = (*qp).put_index;
    let mut take: EmbWord = (*qp).take_index;
    let mut element_array: *mut Byte = &mut (*qp).first_element as *mut [EmbWord; 1]
        as *mut Byte;
    let mut elt: Byte = 0;
    if put == take {
        return 0 as usize as Byte;
    }
    elt = *element_array.offset(take as isize);
    take += 1;
    if take >= (*qp).queue_size {
        take = 0;
    }
    (*qp).take_index = take;
    return elt;
}
#[no_mangle]
pub  fn EmbQueuePutBytes(
    mut qp_arg: *mut EmbQueue,
    mut buffer: *mut Byte,
    mut length: u32,
) -> usize {
    let mut qp: *mut EmbQueue = qp_arg;
    let mut put: EmbWord = (*qp).put_index;
    let mut original_put: EmbWord = put;
    let mut take: EmbWord = 0;
    let mut element_array: *mut Byte = &mut (*qp).first_element as *mut [EmbWord; 1]
        as *mut Byte;
    let mut actual_length: usize = 0;
    let mut count: EmbWord = 0;
    while length > 0 as usize {
        take = (*qp).take_index;
        if take > put {
            count = take - put - 1;
        } else if take == 0 as usize {
            count = (*qp).queue_size - put - 1;
        } else {
            count = (*qp).queue_size - put;
        }
        if count > length {
            count = length;
        }
        memcpy(
            &mut *element_array.offset(put as isize) as *mut Byte as *mut libc::c_void,
            buffer as *const libc::c_void,
            count as size_t,
        );
        buffer = buffer.offset(count as isize);
        length -= count;
        actual_length += count;
        put += count;
        if put == (*qp).queue_size {
            put = 0;
        }
        (*qp).put_index = put;
    }
    if original_put == (*qp).take_index {
        EmbSendSignal((*qp).signal);
    }
    return actual_length;
}
#[no_mangle]
pub  fn EmbQueuePutWords(
    mut qp_arg: *mut EmbQueue,
    mut buffer: *mut EmbWord,
    mut length: u32,
) -> usize {
    let mut qp: *mut EmbQueue = qp_arg;
    let mut put: EmbWord = (*qp).put_index;
    let mut original_put: EmbWord = put;
    let mut take: EmbWord = 0;
    let mut element_array: *mut EmbWord = &mut (*qp).first_element as *mut [EmbWord; 1]
        as *mut EmbWord;
    let mut actual_length: usize = 0;
    let mut count: EmbWord = 0;
    while length > 0 as usize {
        take = (*qp).take_index;
        if take > put {
            count = take - put - 1;
        } else if take == 0 as usize {
            count = (*qp).queue_size - put - 1;
        } else {
            count = (*qp).queue_size - put;
        }
        if count > length {
            count = length;
        }
        memcpy(
            &mut *element_array.offset(put as isize) as *mut EmbWord
                as *mut libc::c_void,
            buffer as *const libc::c_void,
            (count as size_t)
                .wrapping_mul(::std::mem::size_of::<EmbWord>() as libc::c_ulong),
        );
        buffer = buffer.offset(count as isize);
        length -= count;
        actual_length += count;
        put += count;
        if put == (*qp).queue_size {
            put = 0;
        }
        (*qp).put_index = put;
    }
    if original_put == (*qp).take_index {
        EmbSendSignal((*qp).signal);
    }
    return actual_length;
}
#[no_mangle]
pub  fn EmbQueueTakeBytes(
    mut qp_arg: *mut EmbQueue,
    mut buffer: *mut Byte,
    mut length: u32,
) -> usize {
    let mut qp: *mut EmbQueue = qp_arg;
    let mut take: EmbWord = (*qp).take_index;
    let mut put: EmbWord = 0;
    let mut element_array: *mut Byte = &mut (*qp).first_element as *mut [EmbWord; 1]
        as *mut Byte;
    let mut actual_length: usize = 0;
    let mut count: EmbWord = 0;
    while length > 0 as usize {
        put = (*qp).put_index;
        if put >= take {
            count = put - take;
        } else {
            count = (*qp).queue_size - take;
        }
        if count > length {
            count = length;
        }
        memcpy(
            buffer as *mut libc::c_void,
            &mut *element_array.offset(take as isize) as *mut Byte
                as *const libc::c_void,
            count as size_t,
        );
        buffer = buffer.offset(count as isize);
        length -= count;
        actual_length += count;
        take += count;
        if take == (*qp).queue_size {
            take = 0;
        }
        (*qp).take_index = take;
    }
    return actual_length;
}
#[no_mangle]
pub  fn EmbQueueTakeWords(
    mut qp_arg: *mut EmbQueue,
    mut buffer: *mut EmbWord,
    mut length: u32,
) -> usize {
    let mut qp: *mut EmbQueue = qp_arg;
    let mut take: EmbWord = (*qp).take_index;
    let mut put: EmbWord = 0;
    let mut element_array: *mut EmbWord = &mut (*qp).first_element as *mut [EmbWord; 1]
        as *mut EmbWord;
    let mut actual_length: usize = 0;
    let mut count: EmbWord = 0;
    while length > 0 as usize {
        put = (*qp).put_index;
        if put >= take {
            count = put - take;
        } else {
            count = (*qp).queue_size - take;
        }
        if count > length {
            count = length;
        }
        memcpy(
            buffer as *mut libc::c_void,
            &mut *element_array.offset(take as isize) as *mut EmbWord
                as *const libc::c_void,
            (count as size_t)
                .wrapping_mul(::std::mem::size_of::<EmbWord>() as libc::c_ulong),
        );
        buffer = buffer.offset(count as isize);
        length -= count;
        actual_length += count;
        take += count;
        if take == (*qp).queue_size {
            take = 0;
        }
        (*qp).take_index = take;
    }
    return actual_length;
}
#[no_mangle]
pub  fn ResetIncomingQueue(mut q: *mut EmbQueue) {
    (*q).put_index = 0;
    (*q).take_index = 0;
    if (q as *mut EmbWord).offset_from(EmbCommAreaPtr as *mut EmbWord) as libc::c_long
        as EmbPtr
        > (*EmbCommAreaPtr).host_buffer_start + (*EmbCommAreaPtr).host_buffer_size
    {
        RemoveSignalHandler((*q).signal);
        (*q).signal = -(1);
    }
}
#[no_mangle]
pub  fn ResetOutgoingQueue(mut q: *mut EmbQueue) {
    (*q).put_index = 0;
    (*q).take_index = 0;
    (*q).signal = -(1);
}
