#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn InstallSignalHandler(
        singalHandler: ProcPtrV,
        signalArgument: PtrV,
        inputP: Boole,
    ) -> SignalNumber;
    fn SignalLater(signal: SignalNumber);
    static mut EmbCommAreaPtr: *mut EmbCommArea;
    fn EmbQueueSpace(q: *mut EmbQueue) -> u32;
    fn EmbQueueFilled(q: *mut EmbQueue) -> u32;
    fn EmbQueuePutWord(q: *mut EmbQueue, element: EmbWord);
    fn EmbQueueTakeWord(q: *mut EmbQueue) -> EmbWord;
    fn EmbCommAreaAlloc(nBytes: size_t) -> EmbPtr;
    fn CreateQueue(nElements: u32, elementSize: u32) -> EmbPtr;
    fn ResetOutgoingQueue(q: *mut EmbQueue);
    fn ResetIncomingQueue(q: *mut EmbQueue);
    fn vpunt(section:&str, format:&str, _: ...);
    static mut activeMBINChannel: *mut EmbMBINChannel;
    fn SendMBINBuffers(mbinChannel: *mut EmbMBINChannel);
}
pub type size_t = libc::c_ulong;
pub type u8 = libc::c_uchar;
pub type i32 = u32;
pub type u32 = libc::c_uint;
pub type u64 = libc::c_ulong;
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
pub type u64 = u64;
pub type EmbWord = i32;
pub type uEmbWord = ui32;
pub type EmbPtr = EmbWord;
pub type SignalMask = uEmbWord;
pub type SignalNumber = EmbWord;
pub type PtrV = *mut libc::c_void;
pub type ProcPtrV = Option::<fn(PtrV) -> ()>;
pub type Boole = u8;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct in_addr {
    pub s_addr: in_addr_t,
}
pub type in_addr_t = ui32;
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
pub type EmbMessageChannelSubtype = libc::c_uint;
pub const EmbMessageChannelMBINSubtype: EmbMessageChannelSubtype = 3;
pub const EmbMessageChannelCommandSubtype: EmbMessageChannelSubtype = 2;
pub const EmbMessageChannelSerialSubtype: EmbMessageChannelSubtype = 1;
pub type EmbMessageImpulse = libc::c_uint;
pub const EmbMessageImpulseNone: EmbMessageImpulse = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct EmbMessageSubtypeData {
    pub header: C2RustUnnamed_4,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub nextActiveChannel: *mut EmbMessageChannel,
    pub commArea: *mut EmbCommArea,
    pub messageChannel: *mut EmbMessageChannel,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct EmbCommandChannel {
    pub header: C2RustUnnamed_5,
    pub guestToHostQueue: *mut EmbQueue,
    pub guestToHostReturnQueue: *mut EmbQueue,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub nextActiveChannel: *mut EmbMessageChannel,
    pub commArea: *mut EmbCommArea,
    pub messageChannel: *mut EmbMessageChannel,
}
pub type EmbCommandBufferOpcode = libc::c_uint;
pub const EmbCommandBufferStartMBIN: EmbCommandBufferOpcode = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct EmbCommandBuffer {
    pub header: C2RustUnnamed_6,
    pub resultCode: EmbWord,
    pub operands: [EmbWord; 1],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct EmbCommandStartMBINBuffer {
    pub header: C2RustUnnamed_7,
    pub resultCode: EmbWord,
    pub mbinChannel: EmbPtr,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct EmbMBINChannel {
    pub header: C2RustUnnamed_8,
    pub guestToHostQueue: *mut EmbQueue,
    pub guestToHostReturnQueue: *mut EmbQueue,
    pub hostToGuestQueue: *mut EmbQueue,
    pub hostToGuestSupplyQueue: *mut EmbQueue,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
    pub nextActiveChannel: *mut EmbMessageChannel,
    pub commArea: *mut EmbCommArea,
    pub messageChannel: *mut EmbMessageChannel,
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
#[no_mangle]
pub  fn InitializeMessageChannels(mut config: *mut VLMConfig) {
    let mut cp: EmbPtr = EmbCommAreaAlloc(
        ::std::mem::size_of::<EmbMessageChannel>() as libc::c_ulong,
    );
    let mut p: *mut EmbMessageChannel = &mut *(EmbCommAreaPtr as *mut EmbWord)
        .offset(cp as isize) as *mut EmbWord as PtrV as *mut EmbMessageChannel;
    let mut cc: *mut EmbCommandChannel = 0 as *mut EmbCommandChannel;
    (*p).type_0 = EmbMessageChannelType;
    (*p).unit = 0;
    (*EmbCommAreaPtr).command_channel = cp;
    (*p).next = (*EmbCommAreaPtr).channel_table;
    (*EmbCommAreaPtr).channel_table = cp;
    (*p).subtype = EmbMessageChannelCommandSubtype;
    (*p).hostToGuestQueue = -(1);
    (*p).hostToGuestSupplyQueue = -(1);
    cc = malloc(::std::mem::size_of::<EmbCommandChannel>() as libc::c_ulong)
        as *mut EmbCommandChannel;
    if cc.is_null() {
        vpunt(
             "" ,
            b"Couldn't allocate master command message channel\0" as *const u8
                as *const libc::c_char as&str,
        );
    }
    (*p)
        .subtypeData0 = (cc as u64 >> 32
        & 0xffffffff as libc::c_long as libc::c_ulong) as uEmbWord;
    (*p)
        .subtypeData1 = (cc as u64 & 0xffffffff as libc::c_long as libc::c_ulong)
        as uEmbWord;
    let ref mut fresh0 = (*cc).header.nextActiveChannel;
    *fresh0 = 0 as *mut EmbMessageChannel;
    let ref mut fresh1 = (*cc).header.commArea;
    *fresh1 = EmbCommAreaPtr;
    let ref mut fresh2 = (*cc).header.messageChannel;
    *fresh2 = p;
    (*p)
        .guestToHostQueue = CreateQueue(
        5,
        ::std::mem::size_of::<EmbPtr>() as libc::c_ulong,
    );
    let ref mut fresh3 = (*cc).guestToHostQueue;
    *fresh3 = &mut *(EmbCommAreaPtr as *mut EmbWord)
        .offset((*p).guestToHostQueue as isize) as *mut EmbWord as PtrV as *mut EmbQueue;
    (*(*cc).guestToHostQueue)
        .signal = InstallSignalHandler(
        ::std::mem::transmute::<
            Option::<fn(*mut EmbCommandChannel) -> ()>,
            ProcPtrV,
        >(
            Some(
                ExecuteGuestCommands
                    as fn(*mut EmbCommandChannel) -> (),
            ),
        ),
        cc as PtrV,
        0 as usize as Boole,
    );
    (*p)
        .guestToHostReturnQueue = CreateQueue(
        5,
        ::std::mem::size_of::<EmbPtr>() as libc::c_ulong,
    );
    let ref mut fresh4 = (*cc).guestToHostReturnQueue;
    *fresh4 = &mut *(EmbCommAreaPtr as *mut EmbWord)
        .offset((*p).guestToHostReturnQueue as isize) as *mut EmbWord as PtrV
        as *mut EmbQueue;
}
#[no_mangle]
pub  fn PollMessageChannels() {
    let mut messageChannel: *mut EmbMessageChannel = 0 as *mut EmbMessageChannel;
    let mut subtypeData: *mut EmbMessageSubtypeData = 0 as *mut EmbMessageSubtypeData;
    let mut guestToHostQueue: *mut EmbQueue = 0 as *mut EmbQueue;
    if -(1) == (*EmbCommAreaPtr).command_channel {
        return;
    }
    messageChannel = &mut *(EmbCommAreaPtr as *mut EmbWord)
        .offset((*EmbCommAreaPtr).command_channel as isize) as *mut EmbWord as PtrV
        as *mut EmbMessageChannel;
    while !messageChannel.is_null() {
        subtypeData = (((*messageChannel).subtypeData0 as u64) << 32
            | (*messageChannel).subtypeData1 as libc::c_ulong) as PtrV
            as *mut EmbMessageSubtypeData;
        guestToHostQueue = &mut *(EmbCommAreaPtr as *mut EmbWord)
            .offset((*messageChannel).guestToHostQueue as isize) as *mut EmbWord as PtrV
            as *mut EmbQueue;
        if (*messageChannel).guestToHostImpulse != 0
            && (*guestToHostQueue).signal != -(1)
        {
            let ref mut fresh5 = (*EmbCommAreaPtr).guest_to_host_signals;
            *fresh5
                |= ((1) << (*guestToHostQueue).signal) as libc::c_uint;
        }
        messageChannel = (*subtypeData).header.nextActiveChannel;
    }
}
 fn ThreadActiveMessageChannel(mut theChannel: *mut EmbMessageChannel) {
    let mut messageChannel: *mut EmbMessageChannel = 0 as *mut EmbMessageChannel;
    let mut subtypeData: *mut EmbMessageSubtypeData = 0 as *mut EmbMessageSubtypeData;
    let ref mut fresh6 = (*((((*theChannel).subtypeData0 as u64)
        << 32 as usize | (*theChannel).subtypeData1 as libc::c_ulong) as PtrV
        as *mut EmbMessageSubtypeData))
        .header
        .nextActiveChannel;
    *fresh6 = 0 as *mut EmbMessageChannel;
    messageChannel = &mut *(EmbCommAreaPtr as *mut EmbWord)
        .offset((*EmbCommAreaPtr).command_channel as isize) as *mut EmbWord as PtrV
        as *mut EmbMessageChannel;
    while !messageChannel.is_null() {
        subtypeData = (((*messageChannel).subtypeData0 as u64) << 32
            | (*messageChannel).subtypeData1 as libc::c_ulong) as PtrV
            as *mut EmbMessageSubtypeData;
        if ((*subtypeData).header.nextActiveChannel).is_null() {
            let ref mut fresh7 = (*subtypeData).header.nextActiveChannel;
            *fresh7 = theChannel;
            return;
        }
        messageChannel = (*subtypeData).header.nextActiveChannel;
    }
}
#[no_mangle]
pub  fn UnthreadMessageChannel(mut theChannel: *mut EmbMessageChannel) {
    let mut messageChannel: *mut EmbMessageChannel = 0 as *mut EmbMessageChannel;
    let mut subtypeData: *mut EmbMessageSubtypeData = 0 as *mut EmbMessageSubtypeData;
    let mut theSubtypeData: *mut EmbMessageSubtypeData = 0 as *mut EmbMessageSubtypeData;
    messageChannel = &mut *(EmbCommAreaPtr as *mut EmbWord)
        .offset((*EmbCommAreaPtr).command_channel as isize) as *mut EmbWord as PtrV
        as *mut EmbMessageChannel;
    while !messageChannel.is_null() {
        subtypeData = (((*messageChannel).subtypeData0 as u64) << 32
            | (*messageChannel).subtypeData1 as libc::c_ulong) as PtrV
            as *mut EmbMessageSubtypeData;
        if theChannel == (*subtypeData).header.nextActiveChannel {
            theSubtypeData = (((*theChannel).subtypeData0 as u64)
                << 32 as usize | (*theChannel).subtypeData1 as libc::c_ulong)
                as PtrV as *mut EmbMessageSubtypeData;
            let ref mut fresh8 = (*subtypeData).header.nextActiveChannel;
            *fresh8 = (*theSubtypeData).header.nextActiveChannel;
            let ref mut fresh9 = (*theSubtypeData).header.nextActiveChannel;
            *fresh9 = 0 as *mut EmbMessageChannel;
            return;
        }
        messageChannel = (*subtypeData).header.nextActiveChannel;
    }
}
 fn ExecuteGuestCommands(mut commandChannel: *mut EmbCommandChannel) {
    let mut commandQueue: *mut EmbQueue = (*commandChannel).guestToHostQueue;
    let mut resultsQueue: *mut EmbQueue = (*commandChannel).guestToHostReturnQueue;
    let mut commandPtr: EmbPtr = 0;
    let mut command: *mut EmbCommandBuffer = 0 as *mut EmbCommandBuffer;
    let mut startMBINCommand: *mut EmbCommandStartMBINBuffer = 0
        as *mut EmbCommandStartMBINBuffer;
    let mut mbinChannel: *mut EmbMessageChannel = 0 as *mut EmbMessageChannel;
    let mut mbinSubChannel: *mut EmbMBINChannel = 0 as *mut EmbMBINChannel;
    while EmbQueueFilled(commandQueue) != 0 {
        if 0 as usize == EmbQueueSpace(resultsQueue) {
            SignalLater((*commandQueue).signal);
            return;
        }
        commandPtr = EmbQueueTakeWord(commandQueue);
        if commandPtr != 0 {
            command = &mut *(EmbCommAreaPtr as *mut EmbWord).offset(commandPtr as isize)
                as *mut EmbWord as PtrV as *mut EmbCommandBuffer;
            match ((*command).header).opcode() {
                1 => {
                    startMBINCommand = command as *mut EmbCommandStartMBINBuffer;
                    mbinChannel = &mut *(EmbCommAreaPtr as *mut EmbWord)
                        .offset((*startMBINCommand).mbinChannel as isize) as *mut EmbWord
                        as PtrV as *mut EmbMessageChannel;
                    mbinSubChannel = malloc(
                        ::std::mem::size_of::<EmbMBINChannel>() as libc::c_ulong,
                    ) as *mut EmbMBINChannel;
                    if mbinSubChannel.is_null() {
                        (*command).resultCode = 12;
                    } else {
                        let ref mut fresh10 = (*mbinSubChannel).header.commArea;
                        *fresh10 = EmbCommAreaPtr;
                        let ref mut fresh11 = (*mbinSubChannel).header.messageChannel;
                        *fresh11 = mbinChannel;
                        let ref mut fresh12 = (*mbinSubChannel).guestToHostQueue;
                        *fresh12 = &mut *(EmbCommAreaPtr as *mut EmbWord)
                            .offset((*mbinChannel).guestToHostQueue as isize)
                            as *mut EmbWord as PtrV as *mut EmbQueue;
                        let ref mut fresh13 = (*mbinSubChannel).guestToHostReturnQueue;
                        *fresh13 = &mut *(EmbCommAreaPtr as *mut EmbWord)
                            .offset((*mbinChannel).guestToHostReturnQueue as isize)
                            as *mut EmbWord as PtrV as *mut EmbQueue;
                        let ref mut fresh14 = (*mbinSubChannel).hostToGuestQueue;
                        *fresh14 = &mut *(EmbCommAreaPtr as *mut EmbWord)
                            .offset((*mbinChannel).hostToGuestQueue as isize)
                            as *mut EmbWord as PtrV as *mut EmbQueue;
                        let ref mut fresh15 = (*mbinSubChannel).hostToGuestSupplyQueue;
                        *fresh15 = &mut *(EmbCommAreaPtr as *mut EmbWord)
                            .offset((*mbinChannel).hostToGuestSupplyQueue as isize)
                            as *mut EmbWord as PtrV as *mut EmbQueue;
                        (*mbinChannel)
                            .subtypeData0 = (mbinSubChannel as u64
                            >> 32
                            & 0xffffffff as libc::c_long as libc::c_ulong) as uEmbWord;
                        (*mbinChannel)
                            .subtypeData1 = (mbinSubChannel as u64
                            & 0xffffffff as libc::c_long as libc::c_ulong) as uEmbWord;
                        ThreadActiveMessageChannel(mbinChannel);
                        activeMBINChannel = mbinSubChannel;
                        (*(*mbinSubChannel).guestToHostQueue)
                            .signal = InstallSignalHandler(
                            ::std::mem::transmute::<
                                Option::<fn(*mut EmbMBINChannel) -> ()>,
                                ProcPtrV,
                            >(
                                Some(
                                    SendMBINBuffers
                                        as fn(*mut EmbMBINChannel) -> (),
                                ),
                            ),
                            mbinSubChannel as PtrV,
                            0 as usize as Boole,
                        );
                        (*command).resultCode = 0;
                    }
                }
                _ => {
                    (*command).resultCode = 22;
                }
            }
            EmbQueuePutWord(resultsQueue, commandPtr);
        }
    }
}
#[no_mangle]
pub  fn ResetMessageChannel(mut channel: *mut EmbChannel) {
    let mut messageChannel: *mut EmbMessageChannel = channel as *mut EmbMessageChannel;
    let mut subtypeData: *mut EmbMessageSubtypeData = 0 as *mut EmbMessageSubtypeData;
    let mut allocatedByVLM: Boole = 0;
    allocatedByVLM = ((channel as *mut EmbWord)
        .offset_from(EmbCommAreaPtr as *mut EmbWord) as libc::c_long as EmbPtr
        > (*EmbCommAreaPtr).host_buffer_start + (*EmbCommAreaPtr).host_buffer_size)
        as usize as Boole;
    (*messageChannel).guestToHostImpulse = EmbMessageImpulseNone;
    (*messageChannel).hostToGuestImpulse = EmbMessageImpulseNone;
    subtypeData = (((*messageChannel).subtypeData0 as u64) << 32
        | (*messageChannel).subtypeData1 as libc::c_ulong) as PtrV
        as *mut EmbMessageSubtypeData;
    let ref mut fresh16 = (*subtypeData).header.nextActiveChannel;
    *fresh16 = 0 as *mut EmbMessageChannel;
    match (*messageChannel).subtype {
        2 => {
            ResetIncomingQueue(
                &mut *(EmbCommAreaPtr as *mut EmbWord)
                    .offset((*messageChannel).guestToHostQueue as isize) as *mut EmbWord
                    as PtrV as *mut EmbQueue,
            );
            ResetOutgoingQueue(
                &mut *(EmbCommAreaPtr as *mut EmbWord)
                    .offset((*messageChannel).guestToHostReturnQueue as isize)
                    as *mut EmbWord as PtrV as *mut EmbQueue,
            );
        }
        3 => {
            ResetIncomingQueue(
                &mut *(EmbCommAreaPtr as *mut EmbWord)
                    .offset((*messageChannel).guestToHostQueue as isize) as *mut EmbWord
                    as PtrV as *mut EmbQueue,
            );
            ResetOutgoingQueue(
                &mut *(EmbCommAreaPtr as *mut EmbWord)
                    .offset((*messageChannel).guestToHostReturnQueue as isize)
                    as *mut EmbWord as PtrV as *mut EmbQueue,
            );
            ResetIncomingQueue(
                &mut *(EmbCommAreaPtr as *mut EmbWord)
                    .offset((*messageChannel).hostToGuestSupplyQueue as isize)
                    as *mut EmbWord as PtrV as *mut EmbQueue,
            );
            ResetOutgoingQueue(
                &mut *(EmbCommAreaPtr as *mut EmbWord)
                    .offset((*messageChannel).hostToGuestQueue as isize) as *mut EmbWord
                    as PtrV as *mut EmbQueue,
            );
            if allocatedByVLM as usize != 0
                && activeMBINChannel == subtypeData as *mut EmbMBINChannel
            {
                activeMBINChannel = 0 as *mut EmbMBINChannel;
            }
        }
        _ => {}
    }
    if allocatedByVLM as usize != 0 && !subtypeData.is_null() {
        free(subtypeData as *mut libc::c_void);
        (*messageChannel)
            .subtypeData0 = (0 as *mut libc::c_void as u64 >> 32
            & 0xffffffff as libc::c_long as libc::c_ulong) as uEmbWord;
        (*messageChannel)
            .subtypeData1 = (0 as *mut libc::c_void as u64
            & 0xffffffff as libc::c_long as libc::c_ulong) as uEmbWord;
    }
}
#[no_mangle]
pub  fn TerminateMessageChannels() {}
