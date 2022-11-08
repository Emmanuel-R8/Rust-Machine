#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]


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
pub type socklen_t = __socklen_t;
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
pub type C2RustUnnamed_0 = libc::c_uint;
pub const MSG_CMSG_CLOEXEC: C2RustUnnamed_0 = 1073741824;
pub const MSG_FASTOPEN: C2RustUnnamed_0 = 536870912;
pub const MSG_ZEROCOPY: C2RustUnnamed_0 = 67108864;
pub const MSG_BATCH: C2RustUnnamed_0 = 262144;
pub const MSG_WAITFORONE: C2RustUnnamed_0 = 65536;
pub const MSG_MORE: C2RustUnnamed_0 = 32768;
pub const MSG_NOSIGNAL: C2RustUnnamed_0 = 16384;
pub const MSG_ERRQUEUE: C2RustUnnamed_0 = 8192;
pub const MSG_RST: C2RustUnnamed_0 = 4096;
pub const MSG_CONFIRM: C2RustUnnamed_0 = 2048;
pub const MSG_SYN: C2RustUnnamed_0 = 1024;
pub const MSG_FIN: C2RustUnnamed_0 = 512;
pub const MSG_WAITALL: C2RustUnnamed_0 = 256;
pub const MSG_EOR: C2RustUnnamed_0 = 128;
pub const MSG_DONTWAIT: C2RustUnnamed_0 = 64;
pub const MSG_TRUNC: C2RustUnnamed_0 = 32;
pub const MSG_PROXY: C2RustUnnamed_0 = 16;
pub const MSG_CTRUNC: C2RustUnnamed_0 = 8;
pub const MSG_DONTROUTE: C2RustUnnamed_0 = 4;
pub const MSG_PEEK: C2RustUnnamed_0 = 2;
pub const MSG_OOB: C2RustUnnamed_0 = 1;
pub type in_addr_t = u32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct in_addr {
    pub s_addr: in_addr_t,
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
pub struct if_nameindex {
    pub if_index: libc::c_uint,
    pub if_name:&str,
}
pub type C2RustUnnamed_1 = libc::c_uint;
pub const IFF_DYNAMIC: C2RustUnnamed_1 = 32768;
pub const IFF_AUTOMEDIA: C2RustUnnamed_1 = 16384;
pub const IFF_PORTSEL: C2RustUnnamed_1 = 8192;
pub const IFF_MULTICAST: C2RustUnnamed_1 = 4096;
pub const IFF_SLAVE: C2RustUnnamed_1 = 2048;
pub const IFF_MASTER: C2RustUnnamed_1 = 1024;
pub const IFF_ALLMULTI: C2RustUnnamed_1 = 512;
pub const IFF_PROMISC: C2RustUnnamed_1 = 256;
pub const IFF_NOARP: C2RustUnnamed_1 = 128;
pub const IFF_RUNNING: C2RustUnnamed_1 = 64;
pub const IFF_NOTRAILERS: C2RustUnnamed_1 = 32;
pub const IFF_POINTOPOINT: C2RustUnnamed_1 = 16;
pub const IFF_LOOPBACK: C2RustUnnamed_1 = 8;
pub const IFF_DEBUG: C2RustUnnamed_1 = 4;
pub const IFF_BROADCAST: C2RustUnnamed_1 = 2;
pub const IFF_UP: C2RustUnnamed_1 = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ifmap {
    pub mem_start: libc::c_ulong,
    pub mem_end: libc::c_ulong,
    pub base_addr: libc::c_ushort,
    pub irq: libc::c_uchar,
    pub dma: libc::c_uchar,
    pub port: libc::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ifreq {
    pub ifr_ifrn: C2RustUnnamed_3,
    pub ifr_ifru: C2RustUnnamed_2,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_2 {
    pub ifru_addr: sockaddr,
    pub ifru_dstaddr: sockaddr,
    pub ifru_broadaddr: sockaddr,
    pub ifru_netmask: sockaddr,
    pub ifru_hwaddr: sockaddr,
    pub ifru_flags: libc::c_short,
    pub ifru_ivalue: u32,
    pub ifru_mtu: u32,
    pub ifru_map: ifmap,
    pub ifru_slave: [libc::c_char; 16],
    pub ifru_newname: [libc::c_char; 16],
    pub ifru_data: __u64,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_3 {
    pub ifrn_name: [libc::c_char; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ifconf {
    pub ifc_len: u32,
    pub ifc_ifcu: C2RustUnnamed_4,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_4 {
    pub ifcu_buf: __u64,
    pub ifcu_req: *mut ifreq,
}
pub type __u8 = libc::c_uchar;
pub type __u16 = libc::c_ushort;
pub type __u32 = libc::c_uint;
pub type __be16 = __u16;
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct ethhdr {
    pub h_dest: [libc::c_uchar; 6],
    pub h_source: [libc::c_uchar; 6],
    pub h_proto: __be16,
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct ether_header {
    pub ether_dhost: [u8; 6],
    pub ether_shost: [u8; 6],
    pub ether_type: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct arpreq {
    pub arp_pa: sockaddr,
    pub arp_ha: sockaddr,
    pub arp_flags: u32,
    pub arp_netmask: sockaddr,
    pub arp_dev: [libc::c_char; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_ll {
    pub sll_family: libc::c_ushort,
    pub sll_protocol: libc::c_ushort,
    pub sll_ifindex: u32,
    pub sll_hatype: libc::c_ushort,
    pub sll_pkttype: libc::c_uchar,
    pub sll_halen: libc::c_uchar,
    pub sll_addr: [libc::c_uchar; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sock_filter {
    pub code: __u16,
    pub jt: __u8,
    pub jf: __u8,
    pub k: __u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sock_fprog {
    pub len: libc::c_ushort,
    pub filter: *mut sock_filter,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct EmbNetFilter {
    pub fprog: sock_fprog,
    pub filters: [sock_filter; 6],
}
pub type EmbWord = i32;
pub type UEmbWord = u32;
pub type EmbPtr = EmbWord;
pub type SignalMask = UEmbWord;
pub type SignalNumber = EmbWord;
pub type PtrV = *mut libc::c_void;
pub type ProcPtrV = Option::<fn(PtrV) -> ()>;
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
pub type Byte = u8;
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
    pub data: C2RustUnnamed_5,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_5 {
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
    pub generaVersion: C2RustUnnamed_9,
    pub osfVersion: C2RustUnnamed_8,
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
    pub fep: C2RustUnnamed_7,
    pub restart_applications: EmbWord,
    pub signal_interrupt_vector: EmbWord,
    pub base_register: EmbWord,
    pub hostVersion2: EmbWord,
    pub hostVersion3: EmbWord,
    pub MacIvory_NVRAM_settings: C2RustUnnamed_6,
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
pub struct C2RustUnnamed_6 {
    #[bitfield(name = "data", ty = "EmbWord", bits = "0..=15")]
    #[bitfield(name = "c2rust_unnamed", ty = "EmbWord", bits = "16..=31")]
    pub data_c2rust_unnamed: [u8; 4],
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
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
pub struct C2RustUnnamed_8 {
    #[bitfield(name = "minorRevision", ty = "EmbWord", bits = "0..=7")]
    #[bitfield(name = "majorRevision", ty = "EmbWord", bits = "8..=15")]
    #[bitfield(name = "minorRelease", ty = "EmbWord", bits = "16..=23")]
    #[bitfield(name = "majorRelease", ty = "EmbWord", bits = "24..=30")]
    #[bitfield(name = "testReleaseP", ty = "EmbWord", bits = "31..=31")]
    pub minorRevision_majorRevision_minorRelease_majorRelease_testReleaseP: [u8; 4],
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct C2RustUnnamed_9 {
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
pub struct EmbNetARPReq {
    pub next: *mut EmbNetARPReq,
    pub arp: arpreq,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct EmbNetChannel {
    pub type_0: EmbWord,
    pub unit: EmbWord,
    pub next: EmbPtr,
    pub status: EmbWord,
    pub guestToHostQueue: EmbPtr,
    pub guestToHostReturnQueue: EmbPtr,
    pub hostToGuestSupplyQueue: EmbPtr,
    pub hostToGuestQueue: EmbPtr,
    pub name0: EmbWord,
    pub name1: EmbWord,
    pub hardwareAddressHigh: EmbWord,
    pub hardwareAddressLow: EmbWord,
    pub hostPrimaryProtocol: EmbWord,
    pub hostPrimaryAddress: EmbWord,
    pub guestPrimaryProtocol: EmbWord,
    pub guestPrimaryAddress: EmbWord,
    pub nTransmitFailures: EmbWord,
    pub nReceiveFailures: EmbWord,
    pub nFalseReceiverWakeups: EmbWord,
    pub nReceivedPacketsLost: EmbWord,
    pub unusedMeters: [EmbWord; 4],
    pub addressString: EmbPtr,
    pub guestToHostQ: *mut EmbQueue,
    pub guestToHostReturnQ: *mut EmbQueue,
    pub hostToGuestSupplyQ: *mut EmbQueue,
    pub hostToGuestQ: *mut EmbQueue,
    pub fd: u32,
    pub sll: sockaddr_ll,
    pub arpReq: *mut EmbNetARPReq,
    pub filter: EmbNetFilter,
    pub receiverThread: u64,
    pub receiverThreadSetup: bool,
    pub alignmentPad: u32,
    pub receiveBuffer: [Byte; 1516],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct EmbNetPacket {
    pub nBytes: EmbWord,
    pub data: [EmbWord; 1],
}
pub type ptrdiff_t = libc::c_long;
pub type nfds_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pollfd {
    pub fd: u32,
    pub events: libc::c_short,
    pub revents: libc::c_short,
}
pub type pthread_addr_t = *mut libc::c_void;
pub type pthread_cleanuproutine_t = Option::<
    fn(*mut libc::c_void) -> (),
>;
pub type pthread_startroutine_t = Option::<
    fn(*mut libc::c_void) -> *mut libc::c_void,
>;
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
    pub present: bool,
    pub device: [libc::c_char; 257],
    pub myProtocol: libc::c_ushort,
    pub myAddress: in_addr,
    pub myOptions: [libc::c_char; 257],
    pub anotherAddress: *mut NetworkInterface,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TraceConfig {
    pub traceP: bool,
    pub tracePOST: bool,
    pub bufferSize: u32,
    pub startPC: libc::c_uint,
    pub stopPC: libc::c_uint,
    pub outputFile:&str,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VLMConfig {
    pub enableSpy: bool,
    pub tracing: TraceConfig,
    pub commAreaSize: size_t,
    pub hostBufferSpace: size_t,
    pub guestBufferSpace: size_t,
    pub vlmDebuggerPath: [libc::c_char; 257],
    pub worldPath: [libc::c_char; 257],
    pub worldSearchPath:&str,
    pub enableIDS: bool,
    pub virtualMemory: size_t,
    pub coldLoadXParams: XParams,
    pub generaXParams: XParams,
    pub diagnosticIPAddress: in_addr,
    pub interfaces: [NetworkInterface; 8],
    pub testFunction: bool,
}
pub type ifconf_t = ifconf;
pub type ifreq_t = ifreq;
static mut pInputChannel: *mut EmbNetChannel = 0 as *const EmbNetChannel
    as *mut EmbNetChannel;

pub  fn InitializeNetworkChannels(mut config: *mut VLMConfig) {
    let mut ifc: ifconf_t = ifconf_t {
        ifc_len: 0,
        ifc_ifcu: C2RustUnnamed_4 {
            ifcu_buf:
        },
    };
    let mut ipSocket: u32 = 0;
    let mut savedLen: u32 = 0;
    let mut i: u32 = 0;
    let mut tryAgain: bool = 0;
    printf(b"InitializeNetworkChannels()\n\0"  );
    ipSocket = socket(2, SOCK_STREAM, 0);
    if ipSocket == -(1) {
        vpunt(

            b"Unable to open IP socket to gather network interface information\0"
                  as&str,
        );
    }
    ifc
        .ifc_len = (32)
        .wrapping_mul(::std::mem::size_of::<ifreq_t>());
    ifc.ifc_ifcu.ifcu_buf = 0 as __u64;
    tryAgain = true;
    while tryAgain != 0 {
        ifc
            .ifc_ifcu
            .ifcu_buf = realloc(
            ifc.ifc_ifcu.ifcu_buf ,
            ifc.ifc_len,
        ) as __u64;
        if (ifc.ifc_ifcu.ifcu_buf).is_null() {
            vpunt(

                b"Unable to obtain space to read IP addresses of network interfaces\0"
                      as&str,
            );
        }
        savedLen = ifc.ifc_len;
        if ioctl(
            ipSocket,
            0x8912,
            &mut ifc as *mut ifconf_t,
        ) < 0
        {
            vpunt(

                b"Unable to obtain IP addresses assigned to network interfaces\0"
                      as&str,
            );
        }
        if ifc.ifc_len == savedLen {
            ifc.ifc_len = 2  * ifc.ifc_len;
        } else {
            tryAgain = false;
        }
    }
    ifc
        .ifc_len = (ifc.ifc_len)
        .wrapping_div(::std::mem::size_of::<ifreq_t>());
    printf(
        b"MaxNetworkInterfaces %d\n\0"  ,
        8,
    );
    printf(
        b"0 myAddress %08x\n\0"  ,
        (*config).interfaces[0  ].myAddress.s_addr,
    );
    i = 0;
    while i < 8  {
        if (*config).interfaces[i ].present != 0 {
            InitializeNetChannel(
                &mut *((*config).interfaces).as_mut_ptr().offset(i ),
                i,
                ipSocket,
                &mut ifc,
            );
        }
        i += 1;
    }
    close(ipSocket);
    let mut lispDatum: QWord = LispObj {
        parts: _LispObj {
            tag: 0,
            data: C2RustUnnamed_5 { u: 0 },
        },
    };
    lispDatum.parts.data.u = 0  ;
    lispDatum.parts.tag = 8  as Tag ;
    VirtualMemoryWrite(
        (0xf8041000 )
            .wrapping_add(
                (80)
                    .wrapping_div(::std::mem::size_of::<EmbWord>()),
            ),
        &mut lispDatum,
    );
    let mut lispDatum_0: QWord = LispObj {
        parts: _LispObj {
            tag: 0,
            data: C2RustUnnamed_5 { u: 0 },
        },
    };
    lispDatum_0
        .parts
        .data
        .u = htonl((*config).diagnosticIPAddress.s_addr)  ;
    lispDatum_0.parts.tag = 8  as Tag ;
    VirtualMemoryWrite(
        (0xf8041000 )
            .wrapping_add(
                (84)
                    .wrapping_div(::std::mem::size_of::<EmbWord>()),
            ),
        &mut lispDatum_0,
    );
    let mut lispDatum_1: QWord = LispObj {
        parts: _LispObj {
            tag: 0,
            data: C2RustUnnamed_5 { u: 0 },
        },
    };
    lispDatum_1.parts.data.u = 0  ;
    lispDatum_1.parts.tag = 8  as Tag ;
    VirtualMemoryWrite(
        (0xf8041000 )
            .wrapping_add(
                (108)
                    .wrapping_div(::std::mem::size_of::<EmbWord>()),
            ),
        &mut lispDatum_1,
    );
    let mut lispDatum_2: QWord = LispObj {
        parts: _LispObj {
            tag: 0,
            data: C2RustUnnamed_5 { u: 0 },
        },
    };
    lispDatum_2.parts.data.u = 0  ;
    lispDatum_2.parts.tag = 8  as Tag ;
    VirtualMemoryWrite(
        (0xf8041000 )
            .wrapping_add(
                (112)
                    .wrapping_div(::std::mem::size_of::<EmbWord>()),
            ),
        &mut lispDatum_2,
    );
    let mut lispDatum_3: QWord = LispObj {
        parts: _LispObj {
            tag: 0,
            data: C2RustUnnamed_5 { u: 0 },
        },
    };
    lispDatum_3.parts.data.u = 0  ;
    lispDatum_3.parts.tag = 8  as Tag ;
    VirtualMemoryWrite(
        (0xf8041000 )
            .wrapping_add(
                (116)
                    .wrapping_div(::std::mem::size_of::<EmbWord>()),
            ),
        &mut lispDatum_3,
    );
    let mut lispDatum_4: QWord = LispObj {
        parts: _LispObj {
            tag: 0,
            data: C2RustUnnamed_5 { u: 0 },
        },
    };
    lispDatum_4.parts.data.u = 0  ;
    lispDatum_4.parts.tag = 8  as Tag ;
    VirtualMemoryWrite(
        (0xf8041000 )
            .wrapping_add(
                (120)
                    .wrapping_div(::std::mem::size_of::<EmbWord>()),
            ),
        &mut lispDatum_4,
    );
    let mut lispDatum_5: QWord = LispObj {
        parts: _LispObj {
            tag: 0,
            data: C2RustUnnamed_5 { u: 0 },
        },
    };
    lispDatum_5.parts.data.u = 0  ;
    lispDatum_5.parts.tag = 8  as Tag ;
    VirtualMemoryWrite(
        (0xf8041000 )
            .wrapping_add(
                (124)
                    .wrapping_div(::std::mem::size_of::<EmbWord>()),
            ),
        &mut lispDatum_5,
    );
    let mut lispDatum_6: QWord = LispObj {
        parts: _LispObj {
            tag: 0,
            data: C2RustUnnamed_5 { u: 0 },
        },
    };
    lispDatum_6.parts.data.u = 0  ;
    lispDatum_6.parts.tag = 8  as Tag ;
    VirtualMemoryWrite(
        (0xf8041000 )
            .wrapping_add(
                (128)
                    .wrapping_div(::std::mem::size_of::<EmbWord>()),
            ),
        &mut lispDatum_6,
    );
}
 fn InitializeNetChannel(
    mut interface: *mut NetworkInterface,
    mut unitNumber: u32,
    mut ipSocket: u32,
    mut ifc: *mut ifconf_t,
) {
    let mut cp: EmbPtr = EmbCommAreaAlloc(
        ::std::mem::size_of::<EmbNetChannel>(),
    );
    let mut p: *mut EmbNetChannel = &mut *(EmbCommAreaPtr as *mut EmbWord)
        .offset(cp ) as *mut EmbWord as PtrV as *mut EmbNetChannel;
    let mut ifr: ifreq = ifreq {
        ifr_ifrn: C2RustUnnamed_3 {
            ifrn_name: [0; 16],
        },
        ifr_ifru: C2RustUnnamed_2 {
            ifru_addr: sockaddr {
                sa_family: 0,
                sa_data: [0; 14],
            },
        },
    };
    let mut saved_ifs: *mut if_nameindex = 0 as *mut if_nameindex;
    let mut ifs: *mut if_nameindex = 0 as *mut if_nameindex;
    let mut localFilters: [sock_filter; 7] = [
        {
            let mut init = sock_filter {
                code: (0  + 0x8  + 0x20)
                    as libc::c_ushort,
                jt: 0  as __u8,
                jf: 0  as __u8,
                k: 12  as __u32,
            };
            init
        },
        {
            let mut init = sock_filter {
                code: (0x5  + 0x10  + 0)
                    as libc::c_ushort,
                jt: 3  as __u8,
                jf: 0  as __u8,
                k: 0x806  as __u32,
            };
            init
        },
        {
            let mut init = sock_filter {
                code: (0x5  + 0x10  + 0)
                    as libc::c_ushort,
                jt: 0  as __u8,
                jf: 3  as __u8,
                k: 0x800  as __u32,
            };
            init
        },
        {
            let mut init = sock_filter {
                code: (0  + 0  + 0x20)
                    as libc::c_ushort,
                jt: 0  as __u8,
                jf: 0  as __u8,
                k: 30  as __u32,
            };
            init
        },
        {
            let mut init = sock_filter {
                code: (0x5  + 0x10  + 0)
                    as libc::c_ushort,
                jt: 0  as __u8,
                jf: 1  as __u8,
                k: 0  as __u32,
            };
            init
        },
        {
            let mut init = sock_filter {
                code: (0x6  + 0) as libc::c_ushort,
                jt: 0  as __u8,
                jf: 0  as __u8,
                k: -(1) ,
            };
            init
        },
        {
            let mut init = sock_filter {
                code: (0x6  + 0) as libc::c_ushort,
                jt: 0  as __u8,
                jf: 0  as __u8,
                k: 0  as __u32,
            };
            init
        },
    ];
    let mut etherTypeOffset: libc::c_ushort = (12)
        .wrapping_div(::std::mem::size_of::<libc::c_ushort>())
        as libc::c_ushort;
    let mut ipAddressOffset: libc::c_ushort = (16)
        .wrapping_add(::std::mem::size_of::<ether_header>())
        .wrapping_div(::std::mem::size_of::<libc::c_ushort>())
        as libc::c_ushort;
    let mut interfaceIndex: u32 = 0;
    let mut i: u32 = 0;
    let mut pInterface: *mut NetworkInterface = 0 as *mut NetworkInterface;
    let mut guestAddress: in_addr = in_addr { s_addr: 0 };
    let mut addressAsString: [libc::c_char; 4096] = [0; 4096];
    let mut firstInterface: bool = 0;
    pInputChannel = p;
    (*p).type_0 = EmbNetworkChannelType;
    (*p).unit = unitNumber;
    (*p).fd = -(1);
    (*p).receiverThreadSetup = false;
    (*p).next = (*EmbCommAreaPtr).channel_table;
    (*EmbCommAreaPtr).channel_table = cp;
    if (*interface).device[0  ] != 0 {
        let ref mut fresh0 = (*p).name1;
        *fresh0 = 0;
        (*p).name0 = *fresh0;
        memcpy(
            &mut (*p).name0 as *mut EmbWord as &str ,
            ((*interface).device).as_mut_ptr() ,
            (2)
                .wrapping_mul(::std::mem::size_of::<EmbWord>()),
        );
        printf(
            b"device %s\n\0"  ,
            ((*interface).device).as_mut_ptr(),
        );
        strncpy(
            (ifr.ifr_ifrn.ifrn_name).as_mut_ptr(),
            ((*interface).device).as_mut_ptr(),
            16,
        );
        if ioctl(
            ipSocket,
            0x8933,
            &mut ifr as *mut ifreq,
        ) < 0
        {
            vpunt(

                b"Unable to determine interface index of network device %s\0"
                      as&str,
                ((*interface).device).as_mut_ptr(),
            );
        }
        interfaceIndex = ifr.ifr_ifru.ifru_ivalue;
        if ioctl(
            ipSocket,
            0x8913,
            &mut ifr as *mut ifreq,
        ) < 0
        {
            vpunt(

                b"Unable to determine attributes of network device %s\0"
                     as&str,
                ((*interface).device).as_mut_ptr(),
            );
        }
        if ifr.ifr_ifru.ifru_flags  & IFF_LOOPBACK  != 0 {
            vpunt(

                b"Unable to attach VLM network interface #%d to device %s as it is a loopback device\0"
                      as&str,
                unitNumber,
                ((*interface).device).as_mut_ptr(),
            );
        }
        if ifr.ifr_ifru.ifru_flags
            & (IFF_UP  | IFF_RUNNING)
            != IFF_UP  | IFF_RUNNING
        {
            vpunt(

                b"Unable to attach VLM network interface #%d to device %s as it is not up and running\0"
                      as&str,
                unitNumber,
                ((*interface).device).as_mut_ptr(),
            );
        }
        if ioctl(
            ipSocket,
            0x8927,
            &mut ifr as *mut ifreq,
        ) < 0
        {
            vpunt(

                b"Unable to determine hardware interface address for network device %s\0"
                      as&str,
                ((*interface).device).as_mut_ptr(),
            );
        }
        if ifr.ifr_ifru.ifru_hwaddr.sa_family  != 1  {
            vpunt(

                b"Unable to attach VLM network interface #%d to device %s as it does not use Ethernet packet formats\0"
                      as&str,
                unitNumber,
                ((*interface).device).as_mut_ptr(),
            );
        }
        let ref mut fresh1 = (*p).hardwareAddressLow;
        *fresh1 = 0;
        (*p).hardwareAddressHigh = *fresh1;
        memcpy(
            &mut (*p).hardwareAddressHigh as *mut EmbWord as&str
                ,
            (ifr.ifr_ifru.ifru_hwaddr.sa_data).as_mut_ptr() ,
            (2)
                .wrapping_mul(::std::mem::size_of::<EmbWord>()),
        );
        printf(
            b"hw address %d %d\n\0"  ,
            (*p).hardwareAddressHigh,
            (*p).hardwareAddressLow,
        );
    } else {
        interfaceIndex = -(1);
        saved_ifs = if_nameindex();
        ifs = saved_ifs;
        while (*ifs).if_index != 0
            && !((*ifs).if_name).is_null()
        {
            strncpy(
                (ifr.ifr_ifrn.ifrn_name).as_mut_ptr(),
                (*ifs).if_name,
                16,
            );
            if ioctl(
                ipSocket,
                0x8913,
                &mut ifr as *mut ifreq,
            ) < 0
            {
                vpunt(

                    b"Unable to determine attributes of network device %s\0"
                         as&str,
                    (ifr.ifr_ifrn.ifrn_name).as_mut_ptr(),
                );
            }
            if ifr.ifr_ifru.ifru_flags
                & (IFF_UP  | IFF_RUNNING
                    | IFF_LOOPBACK)
                == IFF_UP  | IFF_RUNNING
            {
                if ioctl(
                    ipSocket,
                    0x8927,
                    &mut ifr as *mut ifreq,
                ) < 0
                {
                    vpunt(

                        b"Unable to determine hardware address for network device %s\0"
                              as&str,
                        (ifr.ifr_ifrn.ifrn_name).as_mut_ptr(),
                    );
                }
                if ifr.ifr_ifru.ifru_hwaddr.sa_family  == 1
                {
                    interfaceIndex = (*ifs).if_index;
                    strncpy(
                        ((*interface).device).as_mut_ptr(),
                        (*ifs).if_name,
                        16,
                    );
                    let ref mut fresh2 = (*p).name1;
                    *fresh2 = 0;
                    (*p).name0 = *fresh2;
                    memcpy(
                        &mut (*p).name0 as *mut EmbWord as&str
                            ,
                        (*ifs).if_name ,
                        (2)
                            .wrapping_mul(
                                ::std::mem::size_of::<EmbWord>(),
                            ),
                    );
                    let ref mut fresh3 = (*p).hardwareAddressLow;
                    *fresh3 = 0;
                    (*p).hardwareAddressHigh = *fresh3;
                    memcpy(
                        &mut (*p).hardwareAddressHigh as *mut EmbWord
                            as &str ,
                        (ifr.ifr_ifru.ifru_hwaddr.sa_data).as_mut_ptr()
                            ,
                        (2)
                            .wrapping_mul(
                                ::std::mem::size_of::<EmbWord>(),
                            ),
                    );
                    break;
                }
            }
            ifs = ifs.offset(1);
        }
        if_freenameindex(saved_ifs);
        if interfaceIndex < 0  {
            vpunt(

                b"Unable to find an Ethernet interface to attach to VLM network interface #%d\0"
                      as&str,
                unitNumber,
            );
        }
    }
    (*p).hostPrimaryProtocol = -(1);
    i = 0;
    while i < (*ifc).ifc_len {
        if strncmp(
            ((*interface).device).as_mut_ptr(),
            ((*((*ifc).ifc_ifcu.ifcu_req).offset(i )).ifr_ifrn.ifrn_name)
                .as_mut_ptr(),
            16,
        ) == 0
        {
            (*p).hostPrimaryProtocol = 0x800;
            (*p)
                .hostPrimaryAddress = (*(&mut (*((*ifc).ifc_ifcu.ifcu_req)
                .offset(i ))
                .ifr_ifru
                .ifru_addr as *mut sockaddr as *mut sockaddr_in))
                .sin_addr
                .s_addr as EmbWord;
            break;
        } else {
            i += 1;
        }
    }
    if (*p).hostPrimaryProtocol == -(1) {
        vpunt(

            b"Unable to determine IP address assigned to network device %s\0"
                  as&str,
            ((*interface).device).as_mut_ptr(),
        );
    }
    printf(
        b"hostPrimaryAddress %d\n\0"  ,
        (*p).hostPrimaryAddress,
    );
    printf(
        b"guestPrimaryAddress %d\n\0"  ,
        (*p).guestPrimaryAddress,
    );
    (*p)
        .fd = socket(
        17,
        SOCK_RAW,
        htons(0x3  as uint16_t),
    );
    if (*p).fd < 0  {
        vpunt(

            b"Unable to open packet socket for VLM network interface #%d\0"
                 as&str,
            unitNumber,
        );
    }
    memset(
        &mut (*p).sll as *mut sockaddr_ll ,
        0,
        ::std::mem::size_of::<sockaddr_ll>(),
    );
    (*p).sll.sll_family = 17  as libc::c_ushort;
    (*p).sll.sll_ifindex = interfaceIndex;
    (*p).sll.sll_protocol = htons(0x3  as uint16_t);
    if bind(
        (*p).fd,
        &mut (*p).sll as *mut sockaddr_ll as *mut sockaddr,
        ::std::mem::size_of::<sockaddr_ll>() as socklen_t,
    ) < 0
    {
        vpunt(

            b"Unable to attach VLM network interface #%d to device %s\0"
                 as&str,
            unitNumber,
            ((*interface).device).as_mut_ptr(),
        );
    }
    (*p).sll.sll_protocol = 0  as libc::c_ushort;
    (*p).sll.sll_halen = 6  as libc::c_uchar;
    printf(
        b"filter myAddress %08x\n\0"  ,
        (*interface).myAddress.s_addr,
    );
    localFilters[4  ].k = (*interface).myAddress.s_addr;
    memcpy(
        ((*p).filter.filters).as_mut_ptr() ,
        localFilters.as_mut_ptr() ,
        ::std::mem::size_of::<[sock_filter; 7]>(),
    );
    (*p).filter.fprog.len = 7  as libc::c_ushort;
    let ref mut fresh4 = (*p).filter.fprog.filter;
    *fresh4 = &mut (*p).filter.filters as *mut [sock_filter; 6] as *mut sock_filter;
    printf(b"attach filter\n\0"  );
    if setsockopt(
        (*p).fd,
        1,
        26,
        &mut (*p).filter.fprog as *mut sock_fprog ,
        ::std::mem::size_of::<sock_fprog>() as socklen_t,
    ) != 0
    {
        vpunt(

            b"Unable to set packet filter program for VLM network interface #%d\0"
                  as&str,
            unitNumber,
        );
    }
    let ref mut fresh5 = (*p).arpReq;
    *fresh5 = 0 as *mut EmbNetARPReq;
    pInterface = interface;
    while !pInterface.is_null() {
        if (*pInterface).myProtocol  == 0x800  {
            let mut arpReqPtr: EmbPtr = EmbCommAreaAlloc(
                ::std::mem::size_of::<EmbNetARPReq>(),
            );
            let mut pARP: *mut EmbNetARPReq = &mut *(EmbCommAreaPtr as *mut EmbWord)
                .offset(arpReqPtr ) as *mut EmbWord as PtrV as *mut EmbNetARPReq;
            let ref mut fresh6 = (*pARP).next;
            *fresh6 = (*p).arpReq;
            let ref mut fresh7 = (*p).arpReq;
            *fresh7 = pARP;
            (*pARP).arp.arp_pa.sa_family = 2  as sa_family_t;
            (*(&mut (*pARP).arp.arp_pa as *mut sockaddr as *mut sockaddr_in))
                .sin_addr
                .s_addr = htonl((*pInterface).myAddress.s_addr);
            (*pARP).arp.arp_ha.sa_family = 1  as sa_family_t;
            memcpy(
                ((*pARP).arp.arp_ha.sa_data).as_mut_ptr() ,
                &mut (*p).hardwareAddressHigh as *mut EmbWord ,
                (2)
                    .wrapping_mul(::std::mem::size_of::<EmbWord>()),
            );
            (*pARP).arp.arp_flags = 0x2  | 0x4;
            memcpy(
                ((*pARP).arp.arp_dev).as_mut_ptr() ,
                ((*interface).device).as_mut_ptr() ,
                ::std::mem::size_of::<[libc::c_char; 16]>(),
            );
            if ioctl(
                ipSocket,
                0x8955,
                &mut (*pARP).arp as *mut arpreq,
            ) < 0
            {
                vpunt(

                    b"Unable to establish ARP mappings for VLM network interface #%d\0"
                          as&str,
                    unitNumber,
                );
            }
        }
        pInterface = (*pInterface).anotherAddress;
    }
    (*p).status = 0;
    (*p).guestPrimaryProtocol = (*interface).myProtocol as EmbWord;
    (*p).guestPrimaryAddress = htonl((*interface).myAddress.s_addr) as EmbWord;
    let ref mut fresh8 = (*p).nReceiveFailures;
    *fresh8 = 0;
    (*p).nTransmitFailures = *fresh8;
    (*p)
        .guestToHostQueue = CreateQueue(
        20,
        ::std::mem::size_of::<EmbPtr>(),
    );
    let ref mut fresh9 = (*p).guestToHostQ;
    *fresh9 = &mut *(EmbCommAreaPtr as *mut EmbWord)
        .offset((*p).guestToHostQueue ) as *mut EmbWord as PtrV as *mut EmbQueue;
    (*(*p).guestToHostQ)
        .signal = InstallSignalHandler(
        ::std::mem::transmute::<
            Option::<fn(*mut EmbNetChannel) -> ()>,
            ProcPtrV,
        >(
            Some(
                NetworkChannelTransmitter
                    as fn(*mut EmbNetChannel) -> (),
            ),
        ),
        p as PtrV,
        false,
    );
    (*p)
        .guestToHostReturnQueue = CreateQueue(
        20,
        ::std::mem::size_of::<EmbPtr>(),
    );
    let ref mut fresh10 = (*p).guestToHostReturnQ;
    *fresh10 = &mut *(EmbCommAreaPtr as *mut EmbWord)
        .offset((*p).guestToHostReturnQueue ) as *mut EmbWord as PtrV
        as *mut EmbQueue;
    (*p)
        .hostToGuestSupplyQueue = CreateQueue(
        100,
        ::std::mem::size_of::<EmbPtr>(),
    );
    let ref mut fresh11 = (*p).hostToGuestSupplyQ;
    *fresh11 = &mut *(EmbCommAreaPtr as *mut EmbWord)
        .offset((*p).hostToGuestSupplyQueue ) as *mut EmbWord as PtrV
        as *mut EmbQueue;
    (*p)
        .hostToGuestQueue = CreateQueue(
        100,
        ::std::mem::size_of::<EmbPtr>(),
    );
    let ref mut fresh12 = (*p).hostToGuestQ;
    *fresh12 = &mut *(EmbCommAreaPtr as *mut EmbWord)
        .offset((*p).hostToGuestQueue ) as *mut EmbWord as PtrV as *mut EmbQueue;
    pInterface = interface;
    firstInterface = true;
    while !pInterface.is_null() {
        if firstInterface != 0 {
            addressAsString[0
                ] = 0  ;
        } else {
            sprintf(
                addressAsString.as_mut_ptr(),
                b"%s,\0"  ,
                addressAsString.as_mut_ptr(),
            );
        }
        if (*pInterface).device[0  ] != 0 {
            sprintf(
                addressAsString.as_mut_ptr(),
                b"%s%s:\0"  ,
                addressAsString.as_mut_ptr(),
                ((*pInterface).device).as_mut_ptr(),
            );
        }
        match (*pInterface).myProtocol  {
            2048 => {
                guestAddress.s_addr = htonl((*pInterface).myAddress.s_addr);
                sprintf(
                    addressAsString.as_mut_ptr(),
                    b"%sINTERNET|%s\0"  ,
                    addressAsString.as_mut_ptr(),
                    inet_ntoa(guestAddress),
                );
            }
            2052 => {
                sprintf(
                    addressAsString.as_mut_ptr(),
                    b"%sCHAOS|%o\0"  ,
                    addressAsString.as_mut_ptr(),
                    htonl((*pInterface).myAddress.s_addr),
                );
            }
            _ => {}
        }
        if (*pInterface).myOptions[0  ] != 0 {
            sprintf(
                addressAsString.as_mut_ptr(),
                b"%s;%s\0"  ,
                addressAsString.as_mut_ptr(),
                ((*pInterface).myOptions).as_mut_ptr(),
            );
        }
        pInterface = (*pInterface).anotherAddress;
        firstInterface = false;
    }
    (*p).addressString = MakeEmbString(addressAsString.as_mut_ptr());
    if pthread_create(
        &mut (*p).receiverThread,
        &mut (*EmbCommAreaPtr).inputThreadAttrs,
        ::std::mem::transmute::<
            Option::<fn(pthread_addr_t) -> ()>,
            pthread_startroutine_t,
        >(Some(NetworkChannelReceiver as fn(pthread_addr_t) -> ())),
        p as pthread_addr_t,
    ) != 0
    {
        vpunt(

            b"Unable to create thread to receive packets for VLM network interface #%d\0"
                  as&str,
            unitNumber,
        );
    }
    (*p).receiverThreadSetup = true;
    let ref mut fresh13 = (*p).status;
    *fresh13 |= 1;
}

pub  fn ResetNetworkChannel(mut channel: *mut EmbChannel) {
    let mut netChannel: *mut EmbNetChannel = channel as *mut EmbNetChannel;
    ResetIncomingQueue((*netChannel).guestToHostQ);
    ResetOutgoingQueue((*netChannel).guestToHostReturnQ);
    ResetIncomingQueue((*netChannel).hostToGuestSupplyQ);
    ResetOutgoingQueue((*netChannel).hostToGuestQ);
}
static mut last_packet: [libc::c_char; 1560] = [0; 1560];
 fn new_packet(
    mut packet:&str,
    mut size: u32,
) -> u32 {
    if memcmp(
        last_packet.as_mut_ptr() ,
        packet ,
        size,
    ) == 0
    {
        return 0;
    }
    memcpy(
        last_packet.as_mut_ptr() ,
        packet ,
        size,
    );
    return 1;
}
 fn recv_packet(mut packet:&str, mut size: u32) {
    let mut netChannel: *mut EmbNetChannel = pInputChannel;
    let mut supplyQueue: *mut EmbQueue = (*netChannel).hostToGuestSupplyQ;
    let mut receiveQueue: *mut EmbQueue = (*netChannel).hostToGuestQ;
    let mut netPacketPtr: EmbPtr = 0;
    let mut netPacket: *mut EmbNetPacket = 0 as *mut EmbNetPacket;
    netPacketPtr = EmbQueueTakeWord(supplyQueue);
    netPacket = &mut *(EmbCommAreaPtr as *mut EmbWord).offset(netPacketPtr )
        as *mut EmbWord as PtrV as *mut EmbNetPacket;
    (*netPacket).nBytes = size;
    memcpy(
        &mut *((*netPacket).data).as_mut_ptr().offset(0 )
            as *mut EmbWord ,
        packet ,
        size,
    );
    EmbQueuePutWord(receiveQueue, netPacketPtr);
}

pub  fn answer_arp(mut pkt:&str, mut size: u32) {
    let mut tmp: [libc::c_char; 10] = [0; 10];
    let mut i: u32 = 0;
    *pkt.offset(21 ) = 2  ;
    memcpy(
        tmp.as_mut_ptr() ,
        &mut *pkt.offset(22 ) as&str
            ,
        10,
    );
    memcpy(
        &mut *pkt.offset(22 ) as&str
            ,
        &mut *pkt.offset(32 ) as&str
            ,
        10,
    );
    i = 0;
    while i < 6  {
        tmp[i ] = i ;
        i += 1;
    }
    memcpy(
        &mut *pkt.offset(32 ) as&str
            ,
        tmp.as_mut_ptr() ,
        10,
    );
    printf(b"answering arp\n\0"  );
    recv_packet(pkt, size);
}

pub  fn dump_packet(
    mut who:&str,
    mut pkt: *mut libc::c_uchar,
    mut size: u32,
) {
    let mut i: u32 = 0;
    let mut offset: u32 = 0;
    let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut pp: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut ptype: libc::c_ushort = 0;
    let mut op: u32 = 0;
    let mut prot: u32 = 0;
    p = pkt;
    ptype = ((*p.offset(12 )) << 8
        | *p.offset(13 )) as libc::c_ushort;
    match ptype  {
        2054 => {}
        2048 => {
            printf(b"%s ip: \0"  , who);
            p = p.offset(14 );
            prot = *p.offset(9 );
            printf(
                b"%u.%u.%u.%u \0"  ,
                *p.offset(12 ),
                *p.offset(13 ),
                *p.offset(14 ),
                *p.offset(15 ),
            );
            printf(
                b"%u.%u.%u.%u \0"  ,
                *p.offset(16 ),
                *p.offset(17 ),
                *p.offset(18 ),
                *p.offset(19 ),
            );
            p = p.offset(20 );
            match prot {
                17 => {
                    printf(
                        b"udp; %u %u\0"  ,
                        (*p.offset(0 ))
                            << 8
                            | *p.offset(1 ),
                        (*p.offset(2 ))
                            << 8
                            | *p.offset(3 ),
                    );
                }
                _ => {}
            }
            printf(b"\n\0"  );
        }
        _ => {
            printf(b"%s \0"  , who);
            i = 0;
            while i < 8  {
                printf(
                    b"%04x: %02x %02x %02x %02x %02x %02x %02x %02x\n\0"
                        ,
                    offset,
                    *p.offset(0 ),
                    *p.offset(1 ),
                    *p.offset(2 ),
                    *p.offset(3 ),
                    *p.offset(4 ),
                    *p.offset(5 ),
                    *p.offset(6 ),
                    *p.offset(7 ),
                );
                offset += 8;
                p = p.offset(8 );
                i += 1;
            }
        }
    };
}
 fn NetworkChannelTransmitter(mut pNetChannel: *mut EmbNetChannel) {
    let mut netChannel: *mut EmbNetChannel = pNetChannel;
    let mut transmitQueue: *mut EmbQueue = (*netChannel).guestToHostQ;
    let mut returnQueue: *mut EmbQueue = (*netChannel).guestToHostReturnQ;
    let mut netPacketPtr: EmbPtr = 0;
    let mut netPacket: *mut EmbNetPacket = 0 as *mut EmbNetPacket;
    let mut nBytes: ssize_t = 0;
    let mut actualBytes: ssize_t = 0;
    while EmbQueueFilled(transmitQueue) != 0 {
        if 0  == EmbQueueSpace(returnQueue) {
            SignalLater((*transmitQueue).signal);
            return;
        }
        netPacketPtr = EmbQueueTakeWord(transmitQueue);
        if (netPacketPtr as u64 ).is_null() {
            netPacketPtr = -(1);
        }
        if netPacketPtr != -(1) {
            netPacket = &mut *(EmbCommAreaPtr as *mut EmbWord)
                .offset(netPacketPtr ) as *mut EmbWord as PtrV
                as *mut EmbNetPacket;
            nBytes = (*netPacket).nBytes as ssize_t;
            memcpy(
                ((*netChannel).sll.sll_addr).as_mut_ptr() ,
                ((*(((*netPacket).data).as_mut_ptr() as *mut ethhdr)).h_dest)
                    .as_mut_ptr() ,
                6,
            );
            actualBytes = sendto(
                (*netChannel).fd,
                &mut *((*netPacket).data).as_mut_ptr().offset(0 )
                    as *mut EmbWord ,
                nBytes,
                0,
                0 as *const sockaddr,
                ::std::mem::size_of::<sockaddr_ll>() as socklen_t,
            );
            if actualBytes != nBytes {
                printf(b"tx error\n\0"  );
                let ref mut fresh14 = (*netChannel).nTransmitFailures;
                *fresh14 += 1;
            }
            if new_packet(
                ::std::mem::transmute::<
                    Option::<
                        fn(
                           &str,
                            u32,
                        ) -> u32,
                    >,
                   &str,
                >(
                    Some(
                        new_packet
                            as fn(
                               &str,
                                u32,
                            ) -> u32,
                    ),
                ),
                nBytes,
            ) != 0 || 1  != 0
            {
                dump_packet(
                    b"tx\0"   as&str,
                    &mut *((*netPacket).data)
                        .as_mut_ptr()
                        .offset(0 ) as *mut EmbWord
                        as *mut libc::c_uchar,
                    nBytes,
                );
            }
            EmbQueuePutWord(returnQueue, netPacketPtr);
        }
    }
}
 fn NetworkChannelReceiver(mut argument: pthread_addr_t) {
    let mut self_0: u64 = pthread_self();
    let mut netChannel: *mut EmbNetChannel = argument as *mut EmbNetChannel;
    let mut supplyQueue: *mut EmbQueue = (*netChannel).hostToGuestSupplyQ;
    let mut receiveQueue: *mut EmbQueue = (*netChannel).hostToGuestQ;
    let mut pollReceiver: pollfd = pollfd {
        fd: 0,
        events: 0,
        revents: 0,
    };
    let mut receiverPause: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
    let mut sll: sockaddr_ll = sockaddr_ll {
        sll_family: 0,
        sll_protocol: 0,
        sll_ifindex: 0,
        sll_hatype: 0,
        sll_pkttype: 0,
        sll_halen: 0,
        sll_addr: [0; 8],
    };
    let mut netPacketPtr: EmbPtr = 0;
    let mut netPacket: *mut EmbNetPacket = 0 as *mut EmbNetPacket;
    let mut actualBytes: ssize_t = 0;
    let mut sllLen: socklen_t = 0;
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
    pollReceiver.fd = (*netChannel).fd;
    pollReceiver.events = 0x1  as libc::c_short;
    loop {
        u64estcancel();
        pollReceiver.revents = 0  as libc::c_short;
        poll(&mut pollReceiver, 1  as nfds_t, 1000);
        if 0  == pollReceiver.revents  & 0x1  {
            continue;
        }
        sllLen = ::std::mem::size_of::<sockaddr_ll>() as socklen_t;
        actualBytes = recvfrom(
            (*netChannel).fd,
            &mut (*netChannel).receiveBuffer as *mut [Byte; 1516] ,
            1516,
            MSG_TRUNC,
            &mut sll as *mut sockaddr_ll as *mut sockaddr,
            &mut sllLen,
        );
        dump_packet(
            b"rx\0"   as&str,
            &mut (*netChannel).receiveBuffer as *mut [Byte; 1516] as *mut libc::c_uchar,
            actualBytes,
        );
        if actualBytes < 0   {
            let ref mut fresh15 = (*netChannel).nReceiveFailures;
            *fresh15 += 1;
        } else if 0   == actualBytes {
            let ref mut fresh16 = (*netChannel).nFalseReceiverWakeups;
            *fresh16 += 1;
        } else if 0  == EmbQueueSpace(supplyQueue)
            || 0  == EmbQueueSpace(receiveQueue)
        {
            let ref mut fresh17 = (*netChannel).nReceivedPacketsLost;
            *fresh17 += 1;
        } else {
            loop {
                netPacketPtr = EmbQueueTakeWord(supplyQueue);
                if !(0  == netPacketPtr) {
                    break;
                }
                receiverPause.tv_sec = 0  as __time_t;
                receiverPause.tv_nsec = 1000000 ;
                if pthread_delay_np(&mut receiverPause) != 0 {
                    vpunt(

                        b"Unable to sleep in thread %lx\0"
                             as&str,
                        self_0,
                    );
                }
            }
            netPacket = &mut *(EmbCommAreaPtr as *mut EmbWord)
                .offset(netPacketPtr ) as *mut EmbWord as PtrV
                as *mut EmbNetPacket;
            (*netPacket).nBytes = actualBytes as EmbWord;
            memcpy(
                &mut *((*netPacket).data).as_mut_ptr().offset(0 )
                    as *mut EmbWord ,
                &mut *((*netChannel).receiveBuffer)
                    .as_mut_ptr()
                    .offset(0 ) as *mut Byte
                    ,
                actualBytes,
            );
            EmbQueuePutWord(receiveQueue, netPacketPtr);
        }
    };
}
 fn TerminateNetChannel(
    mut netChannel: *mut EmbNetChannel,
    mut ipSocket: u32,
) {
    let mut embARPReq: *mut EmbNetARPReq = 0 as *mut EmbNetARPReq;
    let mut exit_value: *mut libc::c_void = 0 ;
    if (*netChannel).receiverThreadSetup != 0 {
        pthread_cancel((*netChannel).receiverThread);
        pthread_join((*netChannel).receiverThread, &mut exit_value);
        (*netChannel).receiverThreadSetup = false;
    }
    embARPReq = (*netChannel).arpReq;
    while !embARPReq.is_null() {
        ioctl(
            ipSocket,
            0x8953,
            &mut (*embARPReq).arp as *mut arpreq,
        );
    }
    if (*netChannel).fd != -(1) {
        close((*netChannel).fd);
        (*netChannel).fd = -(1);
    }
}

pub  fn TerminateNetworkChannels() {
    let mut netChannel: *mut EmbNetChannel = 0 as *mut EmbNetChannel;
    let mut channel: EmbPtr = 0;
    let mut ipSocket: u32 = 0;
    ipSocket = socket(2, SOCK_STREAM, 0);
    channel = (*EmbCommAreaPtr).channel_table;
    while channel != -(1) {
        netChannel = &mut *(EmbCommAreaPtr as *mut EmbWord).offset(channel )
            as *mut EmbWord as PtrV as *mut EmbNetChannel;
        if EmbNetworkChannelType  == (*netChannel).type_0 {
            TerminateNetChannel(netChannel, ipSocket);
        }
        channel = (*netChannel).next;
    }
    if ipSocket > -(1) {
        close(ipSocket);
    }
}
