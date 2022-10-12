#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]

extern "C" {
    fn SignalLater(signal: SignalNumber);
    static mut EmbCommAreaPtr: *mut EmbCommArea;
    fn EmbQueueSpace(q: *mut EmbQueue) -> u32;
    fn EmbQueueFilled(q: *mut EmbQueue) -> u32;
    fn EmbQueuePutWord(q: *mut EmbQueue, element: EmbWord);
    fn EmbQueueTakeWord(q: *mut EmbQueue) -> EmbWord;
    fn ResetIncomingQueue(q: *mut EmbQueue);
    fn ResetOutgoingQueue(q: *mut EmbQueue);
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn UnthreadMessageChannel(theChannel: *mut EmbMessageChannel);
}

pub type EmbWord = i32;
pub type uEmbWord = ui32;
pub type EmbPtr = EmbWord;
pub type SignalMask = uEmbWord;
pub type SignalNumber = EmbWord;
pub type PtrV = *mut libc::c_void;
pub type ProcPtrV = Option::<fn(PtrV) -> ()>;



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
    pub subtypeData0: uEmbWord,
    pub subtypeData1: uEmbWord,
}
pub type EmbMessageImpulse = libc::c_uint;
pub const EmbMessageImpulseNone: EmbMessageImpulse = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct EmbMBINChannel {
    pub header: C2RustUnnamed_4,
    pub guestToHostQueue: *mut EmbQueue,
    pub guestToHostReturnQueue: *mut EmbQueue,
    pub hostToGuestQueue: *mut EmbQueue,
    pub hostToGuestSupplyQueue: *mut EmbQueue,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub nextActiveChannel: *mut EmbMessageChannel,
    pub commArea: *mut EmbCommArea,
    pub messageChannel: *mut EmbMessageChannel,
}
pub type EmbMBINImpulse = libc::c_uint;
pub const EmbMBINImpulseShutdown: EmbMBINImpulse = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub id: libc::c_uint,
    pub acked: Boole,
}
pub const rm_ack: C2RustUnnamed_6 = 2;
pub type C2RustUnnamed_6 = libc::c_uint;
pub const rm_stop: C2RustUnnamed_6 = 10;
pub const rm_mbin: C2RustUnnamed_6 = 9;
pub const rm_create_pages: C2RustUnnamed_6 = 8;
pub const rm_boot: C2RustUnnamed_6 = 7;
pub const rm_trap: C2RustUnnamed_6 = 6;
pub const rm_system_startup: C2RustUnnamed_6 = 5;
pub const rm_read: C2RustUnnamed_6 = 4;
pub const rm_write: C2RustUnnamed_6 = 3;
pub const rm_noop: C2RustUnnamed_6 = 1;
pub const rm_discard: C2RustUnnamed_6 = 0;
static mut MBINHistory: [C2RustUnnamed_5; 16] = [C2RustUnnamed_5 {
    id: 0,
    acked: 0,
}; 16];
static mut mbin_sinValid: Boole = 0 as usize as Boole;
#[no_mangle]
pub static mut activeMBINChannel: *mut EmbMBINChannel = 0 as *const EmbMBINChannel
    as *mut EmbMBINChannel;
 fn read_long(mut bytes: *mut libc::c_uchar) -> libc::c_uint {
    return (*bytes.offset(0 as usize as isize)
        | (*bytes.offset(1 as usize as isize)) << 8
        | (*bytes.offset(2 as usize as isize)) << 16
        | (*bytes.offset(3 as usize as isize)) << 24)
        as libc::c_uint;
}
#[no_mangle]
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
    let mut historyID: usize = 0;
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
                free(mbinChannel as *mut libc::c_void);
                return;
            }
            _ => {
                (*(*mbinChannel).header.messageChannel)
                    .guestToHostImpulse = EmbMessageImpulseNone;
            }
        }
    }
    while EmbQueueFilled(gthQ) != 0 {
        if 0 as usize == EmbQueueSpace(gthrQ) {
            SignalLater((*gthQ).signal);
            return;
        }
        bufferPtr = EmbQueueTakeWord(gthQ);
        if bufferPtr != 0 && bufferPtr != -(1)
            && mbin_sinValid as usize != 0
        {
            buffer = &mut *(EmbCommAreaPtr as *mut EmbWord).offset(bufferPtr as isize)
                as *mut EmbWord as PtrV as *mut rm_aligned_pkt;
            nBytes = read_long(
                &mut *((*buffer).rm_operand)
                    .as_mut_ptr()
                    .offset(0 as usize as isize),
            ) & 0xffffff as usize as libc::c_uint;
            memcpy(
                &mut *(pkt.rm_id).as_mut_ptr().offset(0 as usize as isize)
                    as *mut libc::c_uchar as *mut libc::c_void,
                &mut *((*buffer).rm_id).as_mut_ptr().offset(0 as usize as isize)
                    as *mut libc::c_uchar as *const libc::c_void,
                8 as usize as libc::c_ulong,
            );
            memcpy(
                &mut *(pkt.data).as_mut_ptr().offset(0 as usize as isize)
                    as *mut libc::c_uchar as *mut libc::c_void,
                &mut *((*buffer).data).as_mut_ptr().offset(0 as usize as isize)
                    as *mut libc::c_uchar as *const libc::c_void,
                nBytes as libc::c_ulong,
            );
            if rm_ack as usize == (*buffer).rm_opcode() {
                id = read_long(
                    &mut *((*buffer).rm_id)
                        .as_mut_ptr()
                        .offset(0 as usize as isize),
                );
                historyID = (id & 0xf as usize as libc::c_uint);
                MBINHistory[historyID as usize].id = id;
                MBINHistory[historyID as usize].acked = 1 as usize as Boole;
            }
        }
        EmbQueuePutWord(gthrQ, bufferPtr);
    }
}
