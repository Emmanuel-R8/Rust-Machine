#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![register_tool(c2rust)]
#![feature(extern_types, register_tool)]


extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn memset(_: *mut libc::c_void, _: u32, _: libc::c_ulong) -> *mut libc::c_void;
    fn strncat(_:&str, _: *const libc::c_char, _: libc::c_ulong)
        ->&str;
    fn strncmp(_: *const libc::c_char, _: *const libc::c_char, _: libc::c_ulong) -> u32;
    fn strdup(_: *const libc::c_char) ->&str;
    fn strndup(_: *const libc::c_char, _: libc::c_ulong) ->&str;
    fn strchr(_: *const libc::c_char, _: u32) ->&str;
    fn strrchr(_: *const libc::c_char, _: u32) ->&str;
    fn scandir(
        __dir: *const libc::c_char,
        __namelist: *mut *mut *mut dirent,
        __selector: Option<fn(*const dirent) -> u32>,
        __cmp: Option<fn(*mut *const dirent, *mut *const dirent) -> u32>,
    ) -> u32;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    static mut stdout: *mut FILE;
    fn rename(__old: *const libc::c_char, __new: *const libc::c_char) -> u32;
    fn fflush(__stream: *mut FILE) -> u32;
    fn printf(_: *const libc::c_char, _: ...) -> u32;
    fn sprintf(_:&str, _: *const libc::c_char, _: ...) -> u32;
    fn __errno_location() -> *mut u32;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn lseek(__fd: u32, __offset: __off_t, __whence: u32) -> __off_t;
    fn close(__fd: u32) -> u32;
    fn read(__fd: u32, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn write(__fd: u32, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
    fn ftruncate(__fd: u32, __length: __off_t) -> u32;
    fn LispObjTag(lo: LispObj) -> ui32;
    fn LispObjData(lo: LispObj) -> ui32;
    fn WriteLispObjData(lo: *mut LispObj, newdata: ui32);
    fn MakeLispObj(tag: ui32, data: ui32) -> *mut LispObj;
    fn VirtualMemoryWriteBlockConstant(
        vma: isize,
        object: *mut LispObj,
        count: u32,
        increment: u32,
    ) -> u32;
    fn VirtualMemoryWrite(vma: isize, object: *mut LispObj) -> u32;
    fn VirtualMemoryRead(vma: isize, object: *mut LispObj) -> u32;
    fn MapVirtualAddressTag(vma: isize) -> *mut Tag;
    fn MapVirtualAddressData(vma: isize) -> *mut isize;
    fn EnsureVirtualAddressRange(
        virtualaddress: isize,
        count: u32,
        faultp: Boole,
    ) -> isize;
    fn MapWorldLoad(
        vma: isize,
        length: u32,
        worldfile: u32,
        dataoffset: off_t,
        tagoffset: off_t,
    ) -> isize;
    fn open(__file: *const libc::c_char, __oflag: u32, _: ...) -> u32;
    fn fstat(__fd: u32, __buf: *mut stat) -> u32;
    fn ReadSystemCommSlot(slot: u32, objectPointer: *mut LispObj);
    fn vpunt(section:&str, format:&str, _: ...);
}
pub type u8 = libc::c_uchar;
pub type i32 = u32;
pub type u32 = libc::c_uint;
pub type u64 = libc::c_ulong;
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dirent {
    pub d_ino: __ino_t,
    pub d_off: __off_t,
    pub d_reclen: libc::c_ushort,
    pub d_type: libc::c_uchar,
    pub d_name: [libc::c_char; 256],
}
pub type size_t = libc::c_ulong;
pub type i32 = i32;
pub type u8 = u8;
pub type ui32 = u32;
pub type u64 = u64;
pub type ptrdiff_t = libc::c_long;
pub type off_t = __off_t;
pub type ssize_t = __ssize_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
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
pub type EmbWord = i32;
pub type PtrV = *mut libc::c_void;
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
    pub tag: ui32,
    pub data: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub u: ui32,
    pub s: i32,
    pub f: libc::c_float,
}
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
pub struct _LoadMapEntry {
    pub address: isize,
    pub op: C2RustUnnamed_0,
    pub data: LispObj,
    pub world: PtrV,
}
pub type LoadMapEntry = _LoadMapEntry;
pub type _LoadMapEntryOpcode = libc::c_uint;
pub const LoadMapCopy: _LoadMapEntryOpcode = 3;
pub const LoadMapConstantIncremented: _LoadMapEntryOpcode = 2;
pub const LoadMapConstant: _LoadMapEntryOpcode = 1;
pub const LoadMapDataPages: _LoadMapEntryOpcode = 0;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _World {
    pub pathname:&str,
    pub fd: u32,
    pub format: u32,
    pub byteSwapped: u32,
    pub vlm_data_page_base: u32,
    pub vlm_tags_page_base: u32,
    pub vlm_data_page: *mut Byte,
    pub vlm_tags_page: *mut Byte,
    pub ivory_data_page: *mut Byte,
    pub currentPageNumber: u32,
    pub currentQNumber: u32,
    pub parentWorld: *mut _World,
    pub sysoutGeneration: isize,
    pub sysoutTimestamp1: isize,
    pub sysoutTimestamp2: isize,
    pub sysoutParentTimestamp1: isize,
    pub sysoutParentTimestamp2: isize,
    pub nWiredMapEntries: u32,
    pub wiredMapEntries: *mut LoadMapEntry,
    pub nMergedWiredMapEntries: u32,
    pub mergedWiredMapEntries: *mut LoadMapEntry,
    pub nUnwiredMapEntries: u32,
    pub unwiredMapEntries: *mut LoadMapEntry,
    pub nMergedUnwiredMapEntries: u32,
    pub mergedUnwiredMapEntries: *mut LoadMapEntry,
}
pub type World = _World;
pub type _LoadFileFormat = libc::c_uint;
pub const IvoryWorldFormat: _LoadFileFormat = 1;
pub const VLMWorldFormat: _LoadFileFormat = 0;
pub type VLMPageBases = _VLMPageBases;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _SaveWorldEntry {
    pub address: isize,
    pub extent: isize,
}
pub type SaveWorldEntry = _SaveWorldEntry;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _SaveWorldData {
    pub pathname: isize,
    pub entryCount: isize,
    pub entries: [SaveWorldEntry; 1],
}
pub type SaveWorldData = _SaveWorldData;
pub const sysoutParentTimestamp2: SystemCommAreaSlot = 47;
pub const sysoutParentTimestamp1: SystemCommAreaSlot = 46;
pub const sysoutTimestamp2: SystemCommAreaSlot = 45;
pub const sysoutTimestamp1: SystemCommAreaSlot = 44;
pub const sysoutGenerationNumber: SystemCommAreaSlot = 43;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
    pub st_dev: __dev_t,
    pub st_ino: __ino_t,
    pub st_nlink: __nlink_t,
    pub st_mode: __mode_t,
    pub st_uid: __uid_t,
    pub st_gid: __gid_t,
    pub __pad0: u32,
    pub st_rdev: __dev_t,
    pub st_size: __off_t,
    pub st_blksize: __blksize_t,
    pub st_blocks: __blkcnt_t,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __glibc_reserved: [__syscall_slong_t; 3],
}
pub type SystemCommAreaSlot = u8;
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

#[inline]
fn __bswap_32(mut __bsx: u32) -> u32 {
    return (__bsx & 0xff000000 as libc::c_uint) >> 24
        | (__bsx & 0xff0000 as libc::c_uint) >> 8
        | (__bsx & 0xff00 as libc::c_uint) << 8
        | (__bsx & 0xff as libc::c_uint) << 24;
}

#[no_mangle]
pub fn LoadVLMDebugger(mut config: *mut VLMConfig) {
    let mut world: World = World {
        pathname:  "" ,
        fd: 0,
        format: 0,
        byteSwapped: 0,
        vlm_data_page_base: 0,
        vlm_tags_page_base: 0,
        vlm_data_page: 0 as *mut Byte,
        vlm_tags_page: 0 as *mut Byte,
        ivory_data_page: 0 as *mut Byte,
        currentPageNumber: 0,
        currentQNumber: 0,
        parentWorld: 0 as *mut _World,
        sysoutGeneration: 0,
        sysoutTimestamp1: 0,
        sysoutTimestamp2: 0,
        sysoutParentTimestamp1: 0,
        sysoutParentTimestamp2: 0,
        nWiredMapEntries: 0,
        wiredMapEntries: 0 as *mut LoadMapEntry,
        nMergedWiredMapEntries: 0,
        mergedWiredMapEntries: 0 as *mut LoadMapEntry,
        nUnwiredMapEntries: 0,
        unwiredMapEntries: 0 as *mut LoadMapEntry,
        nMergedUnwiredMapEntries: 0,
        mergedUnwiredMapEntries: 0 as *mut LoadMapEntry,
    };
    let mut i: usize = 0;
    world.pathname = ((*config).vlmDebuggerPath).as_mut_ptr();
    OpenWorldFile(&mut world, 1 as usize as Boole);
    if world.nUnwiredMapEntries > 0 as usize {
        CloseWorldFile(&mut world, 1 as usize as Boole);
        vpunt(
             "" ,
            b"World file %s contains unwired pages; it can't be a VLM debugger\0" as *const u8
                as *const libc::c_char as&str,
            world.pathname,
        );
    }
    i = 0;
    while i < world.nWiredMapEntries {
        LoadMapData(&mut world, &mut *(world.wiredMapEntries).offset(i as isize));
        i += 1;
    }
    CloseWorldFile(&mut world, 1 as usize as Boole);
}
#[no_mangle]
pub fn LoadWorld(mut config: *mut VLMConfig) -> isize {
    let mut world: World = World {
        pathname:  "" ,
        fd: 0,
        format: 0,
        byteSwapped: 0,
        vlm_data_page_base: 0,
        vlm_tags_page_base: 0,
        vlm_data_page: 0 as *mut Byte,
        vlm_tags_page: 0 as *mut Byte,
        ivory_data_page: 0 as *mut Byte,
        currentPageNumber: 0,
        currentQNumber: 0,
        parentWorld: 0 as *mut _World,
        sysoutGeneration: 0,
        sysoutTimestamp1: 0,
        sysoutTimestamp2: 0,
        sysoutParentTimestamp1: 0,
        sysoutParentTimestamp2: 0,
        nWiredMapEntries: 0,
        wiredMapEntries: 0 as *mut LoadMapEntry,
        nMergedWiredMapEntries: 0,
        mergedWiredMapEntries: 0 as *mut LoadMapEntry,
        nUnwiredMapEntries: 0,
        unwiredMapEntries: 0 as *mut LoadMapEntry,
        nMergedUnwiredMapEntries: 0,
        mergedUnwiredMapEntries: 0 as *mut LoadMapEntry,
    };
    let mut worldImageSize: isize = 0;
    let mut i: usize = 0;
    world.pathname = ((*config).worldPath).as_mut_ptr();
    OpenWorldFile(&mut world, 1 as usize as Boole);
    MergeLoadMaps(&mut world, (*config).worldSearchPath);
    worldImageSize = 0 as usize as isize;
    i = 0;
    while i < world.nMergedWiredMapEntries {
        worldImageSize = (worldImageSize as libc::c_ulong).wrapping_add(LoadMapData(
            &mut world,
            &mut *(world.mergedWiredMapEntries).offset(i as isize),
        )) as isize as isize;
        i += 1;
    }
    i = 0;
    while i < world.nMergedUnwiredMapEntries {
        worldImageSize = (worldImageSize as libc::c_ulong).wrapping_add(LoadMapData(
            &mut world,
            &mut *(world.mergedUnwiredMapEntries).offset(i as isize),
        )) as isize as isize;
        i += 1;
    }
    CloseWorldFile(&mut world, 1 as usize as Boole);
    *__errno_location() = 0;
    return worldImageSize;
}
#[no_mangle]
pub fn SaveWorld(mut saveWorldDataVMA: isize) {
    let mut world: World = World {
        pathname:  "" ,
        fd: 0,
        format: 0,
        byteSwapped: 0,
        vlm_data_page_base: 0,
        vlm_tags_page_base: 0,
        vlm_data_page: 0 as *mut Byte,
        vlm_tags_page: 0 as *mut Byte,
        ivory_data_page: 0 as *mut Byte,
        currentPageNumber: 0,
        currentQNumber: 0,
        parentWorld: 0 as *mut _World,
        sysoutGeneration: 0,
        sysoutTimestamp1: 0,
        sysoutTimestamp2: 0,
        sysoutParentTimestamp1: 0,
        sysoutParentTimestamp2: 0,
        nWiredMapEntries: 0,
        wiredMapEntries: 0 as *mut LoadMapEntry,
        nMergedWiredMapEntries: 0,
        mergedWiredMapEntries: 0 as *mut LoadMapEntry,
        nUnwiredMapEntries: 0,
        unwiredMapEntries: 0 as *mut LoadMapEntry,
        nMergedUnwiredMapEntries: 0,
        mergedUnwiredMapEntries: 0 as *mut LoadMapEntry,
    };
    let mut saveWorldData: *mut SaveWorldData = 0 as *mut SaveWorldData;
    let mut saveWorldEntry: *mut SaveWorldEntry = 0 as *mut SaveWorldEntry;
    let mut loadMapEntry: *mut LoadMapEntry = 0 as *mut LoadMapEntry;
    let mut pathnameHeader: LispObj = LispObj {
        parts: _LispObj {
            tag: 0,
            data: C2RustUnnamed { u: 0 },
        },
    };
    let mut pathnameSize: size_t = 0;
    let mut i: usize = 0;
    saveWorldData = MapVirtualAddressData(saveWorldDataVMA) as *mut SaveWorldData;
    world.format = VLMWorldFormat;
    if 23
        != *MapVirtualAddressTag(
            (&mut (*saveWorldData).pathname as *mut isize)
                .offset_from(MapVirtualAddressData(0 as usize as isize))
                as libc::c_long as isize,
        )
    {
        vpunt(
             "" ,
            b"Destination pathname for SaveWorld is not a simple string\0" as *const u8
                as *const libc::c_char as&str,
        );
    }
    VirtualMemoryRead((*saveWorldData).pathname, &mut pathnameHeader);
    if 3 as usize as libc::c_uint
        != LispObjTag(pathnameHeader) & 0x3f as usize as libc::c_uint
    {
        vpunt(
             "" ,
            b"Destination pathname for SaveWord is not a simple string\0" as *const u8
                as *const libc::c_char as&str,
        );
    }
    if (LispObjData(pathnameHeader) & !(32767) as libc::c_uint) as libc::c_long
        != 0x50000000 as libc::c_long
    {
        vpunt(
             "" ,
            b"Destination pathname for SaveWorld is not a simple string\0" as *const u8
                as *const libc::c_char as&str,
        );
    }
    pathnameSize = (LispObjData(pathnameHeader) & 32767 as usize as libc::c_uint) as size_t;
    world.pathname =
        malloc(pathnameSize.wrapping_add(1 as usize as libc::c_ulong)) as&str;
    if (world.pathname).is_null() {
        vpunt(
             "" ,
            b"Unable to allocate space for local copy of destination pathname\0" as *const u8
                as *const libc::c_char as&str,
        );
    }
    memcpy(
        world.pathname as *mut libc::c_void,
        MapVirtualAddressData(
            ((*saveWorldData).pathname).wrapping_add(1 as usize as libc::c_ulong),
        ) as *const libc::c_void,
        pathnameSize,
    );
    *(world.pathname).offset(pathnameSize as isize) = 0 as usize as libc::c_char;
    i = 0;
    while (i as libc::c_ulong) < pathnameSize {
        if '>' as i32 == *(world.pathname).offset(i as isize) as usize {
            *(world.pathname).offset(i as isize) = '/' as i32 as libc::c_char;
        }
        i += 1;
    }
    world.nWiredMapEntries = (*saveWorldData).entryCount;
    world.nUnwiredMapEntries = 0;
    CreateWorldFile(&mut world);
    saveWorldEntry = &mut *((*saveWorldData).entries)
        .as_mut_ptr()
        .offset(0 as usize as isize) as *mut SaveWorldEntry;
    loadMapEntry = world.wiredMapEntries;
    i = 0;
    while i < world.nWiredMapEntries {
        (*loadMapEntry).address = (*saveWorldEntry).address;
        let ref mut fresh0 = (*loadMapEntry).op;
        (*fresh0).set_opcode(LoadMapDataPages as usize as isize);
        let ref mut fresh1 = (*loadMapEntry).op;
        (*fresh1).set_count((*saveWorldEntry).extent);
        i += 1;
        saveWorldEntry = saveWorldEntry.offset(1);
        loadMapEntry = loadMapEntry.offset(1);
    }
    CanonicalizeVLMLoadMapEntries(&mut world);
    WriteVLMWorldFileHeader(&mut world);
    WriteVLMWorldFilePages(&mut world);
    CloseWorldFile(&mut world, 1 as usize as Boole);
}
fn OpenWorldFile(mut world: *mut World, mut puntOnErrors: Boole) -> Boole {
    let mut q: LispObj = LispObj {
        parts: _LispObj {
            tag: 0,
            data: C2RustUnnamed { u: 0 },
        },
    };
    let mut cookie: libc::c_uint = 0;
    let mut pageBases: libc::c_uint = 0;
    let mut wiredCountQ: usize = 0;
    let mut unwiredCountQ: usize = 0;
    let mut pagesBaseQ: usize = 0;
    let mut firstSysoutQ: usize = 0;
    let mut firstMapQ: usize = 0;
    let ref mut fresh2 = (*world).vlm_data_page;
    *fresh2 = 0 as *mut Byte;
    let ref mut fresh3 = (*world).vlm_tags_page;
    *fresh3 = 0 as *mut Byte;
    let ref mut fresh4 = (*world).ivory_data_page;
    *fresh4 = 0 as *mut Byte;
    let ref mut fresh5 = (*world).wiredMapEntries;
    *fresh5 = 0 as *mut LoadMapEntry;
    let ref mut fresh6 = (*world).unwiredMapEntries;
    *fresh6 = 0 as *mut LoadMapEntry;
    let ref mut fresh7 = (*world).mergedWiredMapEntries;
    *fresh7 = 0 as *mut LoadMapEntry;
    let ref mut fresh8 = (*world).mergedUnwiredMapEntries;
    *fresh8 = 0 as *mut LoadMapEntry;
    let ref mut fresh9 = (*world).parentWorld;
    *fresh9 = 0 as *mut _World;
    let ref mut fresh10 = (*world).fd;
    *fresh10 = open((*world).pathname, 0);
    if *fresh10 < 0 as usize {
        if puntOnErrors != 0 {
            vpunt(
                 "" ,
                b"Unable to open world file %s\0" as *const u8 as *const libc::c_char
                    as&str,
                (*world).pathname,
            );
        } else {
            return 0 as usize as Boole;
        }
    }
    if read(
        (*world).fd,
        &mut cookie as *mut libc::c_uint as &str as *mut libc::c_void,
        ::std::mem::size_of::<u32>() as libc::c_ulong,
    ) as libc::c_ulong
        != ::std::mem::size_of::<u32>() as libc::c_ulong
    {
        if puntOnErrors != 0 {
            CloseWorldFile(world, 1 as usize as Boole);
            vpunt(
                 "" ,
                b"Reading world file %s cookie\0" as *const u8 as *const libc::c_char
                    as&str,
                (*world).pathname,
            );
        } else {
            return 0 as usize as Boole;
        }
    }
    if 0o24342504610 as libc::c_long == cookie as libc::c_long {
        (*world).format = VLMWorldFormat;
        (*world).byteSwapped = 0;
    } else if 0o21042305243 as libc::c_long == cookie as libc::c_long {
        (*world).format = VLMWorldFormat;
        (*world).byteSwapped = 1;
    } else if 0o14322444510 as libc::c_long == cookie as libc::c_long {
        (*world).format = IvoryWorldFormat;
        wiredCountQ = 1;
        unwiredCountQ = 2;
        firstSysoutQ = 0;
        firstMapQ = 8;
    } else if puntOnErrors != 0 {
        CloseWorldFile(world, 1 as usize as Boole);
        vpunt(
             "" ,
            b"Format of world file %s is unrecognized\0" as *const u8 as *const libc::c_char
                as&str,
            (*world).pathname,
        );
    } else {
        return 0 as usize as Boole;
    }
    let ref mut fresh11 = (*world).ivory_data_page;
    *fresh11 = malloc(1280 as usize as libc::c_ulong) as *mut Byte;
    if ((*world).ivory_data_page).is_null() {
        if puntOnErrors != 0 {
            CloseWorldFile(world, 1 as usize as Boole);
            vpunt(
                 "" ,
                b"Unable to allocate space for data buffer for world file %s\0" as *const u8
                    as *const libc::c_char as&str,
                (*world).pathname,
            );
        } else {
            return 0 as usize as Boole;
        }
    }
    (*world).currentPageNumber = -(1);
    ReadIvoryWorldFilePage(world, 0);
    if VLMWorldFormat as usize == (*world).format {
        ReadIvoryWorldFileQ(world, 0, &mut q);
        if 0o40000200 as usize as libc::c_uint == LispObjData(q) {
            wiredCountQ = 1;
            unwiredCountQ = 0;
            pagesBaseQ = 3;
            firstSysoutQ = 0;
            firstMapQ = 8;
        } else if 0o40000201 as usize as libc::c_uint == LispObjData(q) {
            wiredCountQ = 1;
            unwiredCountQ = 0;
            pagesBaseQ = 2;
            firstSysoutQ = 3;
            firstMapQ = 8;
        }
    }
    ReadIvoryWorldFileQ(world, wiredCountQ, &mut q);
    (*world).nWiredMapEntries = LispObjData(q);
    if (*world).nWiredMapEntries != 0 {
        let ref mut fresh12 = (*world).wiredMapEntries;
        *fresh12 = malloc(
            ((*world).nWiredMapEntries as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<LoadMapEntry>() as libc::c_ulong),
        ) as *mut LoadMapEntry;
        if ((*world).wiredMapEntries).is_null() {
            if puntOnErrors != 0 {
                CloseWorldFile(world, 1 as usize as Boole);
                vpunt(
                     "" ,
                    b"Unable to allocate space for wired load map for world file %s\0" as *const u8
                        as *const libc::c_char as&str,
                    (*world).pathname,
                );
            } else {
                return 0 as usize as Boole;
            }
        }
    }
    if unwiredCountQ != 0 {
        ReadIvoryWorldFileQ(world, unwiredCountQ, &mut q);
        (*world).nUnwiredMapEntries = LispObjData(q);
    } else {
        (*world).nUnwiredMapEntries = 0;
    }
    if (*world).nUnwiredMapEntries != 0 {
        let ref mut fresh13 = (*world).unwiredMapEntries;
        *fresh13 = malloc(
            ((*world).nUnwiredMapEntries as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<LoadMapEntry>() as libc::c_ulong),
        ) as *mut LoadMapEntry;
        if ((*world).unwiredMapEntries).is_null() {
            if puntOnErrors != 0 {
                CloseWorldFile(world, 1 as usize as Boole);
                vpunt(
                     "" ,
                    b"Unable to allocate space for unwired load map for world file %s\0"
                        as *const u8 as *const libc::c_char
                        as&str,
                    (*world).pathname,
                );
            } else {
                return 0 as usize as Boole;
            }
        }
    }
    if VLMWorldFormat as usize == (*world).format {
        ReadIvoryWorldFileQ(world, pagesBaseQ, &mut q);
        pageBases = LispObjData(q);
        (*world).vlm_data_page_base = (*(&mut pageBases as *mut libc::c_uint as *mut VLMPageBases))
            .dataPageBase();
        (*world).vlm_tags_page_base = (*(&mut pageBases as *mut libc::c_uint as *mut VLMPageBases))
            .tagsPageBase();
    }
    if firstSysoutQ != 0 {
        (*world).currentQNumber = firstSysoutQ;
        ReadIvoryWorldFileNextQ(world, &mut q);
        (*world).sysoutGeneration = LispObjData(q) as isize;
        ReadIvoryWorldFileNextQ(world, &mut q);
        (*world).sysoutTimestamp1 = LispObjData(q) as isize;
        ReadIvoryWorldFileNextQ(world, &mut q);
        (*world).sysoutTimestamp2 = LispObjData(q) as isize;
        ReadIvoryWorldFileNextQ(world, &mut q);
        (*world).sysoutParentTimestamp1 = LispObjData(q) as isize;
        ReadIvoryWorldFileNextQ(world, &mut q);
        (*world).sysoutParentTimestamp2 = LispObjData(q) as isize;
    } else {
        (*world).sysoutGeneration = 0 as usize as isize;
        let ref mut fresh14 = (*world).sysoutTimestamp2;
        *fresh14 = 0 as usize as isize;
        (*world).sysoutTimestamp1 = *fresh14;
        let ref mut fresh15 = (*world).sysoutParentTimestamp2;
        *fresh15 = 0 as usize as isize;
        (*world).sysoutParentTimestamp1 = *fresh15;
    }
    (*world).currentQNumber = firstMapQ;
    ReadLoadMap(world, (*world).nWiredMapEntries, (*world).wiredMapEntries);
    ReadLoadMap(
        world,
        (*world).nUnwiredMapEntries,
        (*world).unwiredMapEntries,
    );
    return 1 as usize as Boole;
}
fn CreateWorldFile(mut world: *mut World) {
    let ref mut fresh16 = (*world).vlm_data_page;
    *fresh16 = 0 as *mut Byte;
    let ref mut fresh17 = (*world).vlm_tags_page;
    *fresh17 = 0 as *mut Byte;
    let ref mut fresh18 = (*world).ivory_data_page;
    *fresh18 = 0 as *mut Byte;
    let ref mut fresh19 = (*world).wiredMapEntries;
    *fresh19 = 0 as *mut LoadMapEntry;
    let ref mut fresh20 = (*world).unwiredMapEntries;
    *fresh20 = 0 as *mut LoadMapEntry;
    let ref mut fresh21 = (*world).mergedWiredMapEntries;
    *fresh21 = 0 as *mut LoadMapEntry;
    let ref mut fresh22 = (*world).mergedUnwiredMapEntries;
    *fresh22 = 0 as *mut LoadMapEntry;
    let ref mut fresh23 = (*world).parentWorld;
    *fresh23 = 0 as *mut _World;
    if VLMWorldFormat as usize != (*world).format {
        vpunt(
             "" ,
            b"Cannot create world files in other than VLM format\0" as *const u8
                as *const libc::c_char as&str,
        );
    }
    let ref mut fresh24 = (*world).fd;
    *fresh24 = open(
        (*world).pathname,
        0o1 as usize | 0o100 as usize | 0o1000,
        0o400
            | 0o200
            | 0o400 as usize >> 3
            | 0o400 as usize >> 3 as usize >> 3,
    );
    if *fresh24 < 0 as usize {
        vpunt(
             "" ,
            b"Unable to create world file %s\0" as *const u8 as *const libc::c_char
                as&str,
            (*world).pathname,
        );
    }
    let ref mut fresh25 = (*world).ivory_data_page;
    *fresh25 = malloc(1280 as usize as libc::c_ulong) as *mut Byte;
    if ((*world).ivory_data_page).is_null() {
        CloseWorldFile(world, 1 as usize as Boole);
        vpunt(
             "" ,
            b"Unable to allocate space for data buffer for world file %s\0" as *const u8
                as *const libc::c_char as&str,
            (*world).pathname,
        );
    }
    (*world).currentPageNumber = -(1);
    if (*world).nWiredMapEntries != 0 {
        let ref mut fresh26 = (*world).wiredMapEntries;
        *fresh26 = malloc(
            ((*world).nWiredMapEntries as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<LoadMapEntry>() as libc::c_ulong),
        ) as *mut LoadMapEntry;
        if ((*world).wiredMapEntries).is_null() {
            CloseWorldFile(world, 1 as usize as Boole);
            vpunt(
                 "" ,
                b"Unable to allocate space for wired load map for world file %s\0" as *const u8
                    as *const libc::c_char as&str,
                (*world).pathname,
            );
        }
    }
    if (*world).nUnwiredMapEntries != 0 {
        let ref mut fresh27 = (*world).unwiredMapEntries;
        *fresh27 = malloc(
            ((*world).nUnwiredMapEntries as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<LoadMapEntry>() as libc::c_ulong),
        ) as *mut LoadMapEntry;
        if ((*world).unwiredMapEntries).is_null() {
            CloseWorldFile(world, 1 as usize as Boole);
            vpunt(
                 "" ,
                b"Unable to allocate space for unwired load map for world file %s\0" as *const u8
                    as *const libc::c_char as&str,
                (*world).pathname,
            );
        }
    }
}


fn ReadLoadMap(
    mut world: *mut World,
    mut nMapEntries: u32,
    mut mapEntries: *mut LoadMapEntry,
) {
    let mut q: LispObj = LispObj {
        parts: _LispObj {
            tag: 0,
            data: C2RustUnnamed { u: 0 },
        },
    };
    let mut i: usize = 0;
    i = 0;
    while i < nMapEntries {
        ReadIvoryWorldFileNextQ(world, &mut q);
        (*mapEntries).address = LispObjData(q) as isize;
        ReadIvoryWorldFileNextQ(world, &mut q);
        *(&mut (*mapEntries).op as *mut C2RustUnnamed_0 as *mut isize) =
            LispObjData(q) as isize;
        ReadIvoryWorldFileNextQ(world, &mut q);
        (*mapEntries).data = q;
        let ref mut fresh36 = (*mapEntries).world;
        *fresh36 = world as PtrV;
        i += 1;
        mapEntries = mapEntries.offset(1);
    }
}

fn LoadMapData(
    mut world: *mut World,
    mut mapEntry: *mut LoadMapEntry,
) -> isize {
    match (*world).format {
        0 => return VLMLoadMapData(world, mapEntry),
        1 => return IvoryLoadMapData(world, mapEntry),
        _ => {
            CloseWorldFile(world, 1 as usize as Boole);
            vpunt(
                 "" ,
                b"Format of world file %s is unrecognized\0" as *const u8 as *const libc::c_char
                    as&str,
                (*world).pathname,
            );
        }
    }
    panic!("Reached end of non-void function without returning");
}

fn VLMLoadMapData(
    mut world: *mut World,
    mut mapEntry: *mut LoadMapEntry,
) -> isize {
    let mut q: LispObj = LispObj {
        parts: _LispObj {
            tag: 0,
            data: C2RustUnnamed { u: 0 },
        },
    };
    let mut mapWorld: *mut World = 0 as *mut World;
    let mut pageNumber: isize = 0;
    let mut theAddress: isize = 0;
    let mut theSourceAddress: isize = 0;
    let mut dataOffset: off_t = 0;
    let mut tagOffset: off_t = 0;
    let mut increment: usize = 0;
    let mut i: usize = 0;
    let mut current_block_32: u64;
    match ((*mapEntry).op).opcode() as usize {
        0 => {
            mapWorld = (*mapEntry).world as *mut World;
            pageNumber = LispObjData((*mapEntry).data) as isize;
            if (*mapWorld).byteSwapped != 0 {
                EnsureVirtualAddressRange(
                    (*mapEntry).address,
                    ((*mapEntry).op).count(),
                    0 as usize as Boole,
                );
                ReadSwappedVLMWorldFilePage(mapWorld, pageNumber);
                (*mapWorld).currentQNumber = 0;
                printf(
                    b"LoadMapDataPages @ %ld, count %d\n\0" as *const u8 as *const libc::c_char,
                    theAddress,
                    ((*mapEntry).op).count(),
                );
                theAddress = (*mapEntry).address;
                i = 0;
                while i < ((*mapEntry).op).count() as usize {
                    ReadSwappedVLMWorldFileNextQ(mapWorld, &mut q);
                    VirtualMemoryWrite(theAddress, &mut q);
                    i += 1;
                    theAddress = theAddress.wrapping_add(1);
                }
            } else {
                dataOffset = (8192 as usize as libc::c_ulong).wrapping_mul(
                    ((*mapWorld).vlm_data_page_base as libc::c_ulong)
                        .wrapping_add(pageNumber.wrapping_mul(4 as usize as libc::c_ulong)),
                ) as off_t;
                tagOffset = (8192 as usize as libc::c_ulong).wrapping_mul(
                    ((*mapWorld).vlm_tags_page_base as libc::c_ulong)
                        .wrapping_add(pageNumber.wrapping_mul(1 as usize as libc::c_ulong)),
                ) as off_t;
                MapWorldLoad(
                    (*mapEntry).address,
                    ((*mapEntry).op).count(),
                    (*mapWorld).fd,
                    dataOffset,
                    tagOffset,
                );
            }
            current_block_32 = 10692455896603418738;
        }
        2 => {
            increment = 1;
            current_block_32 = 1068983259865828954;
        }
        1 => {
            current_block_32 = 1068983259865828954;
        }
        3 => {
            EnsureVirtualAddressRange(
                (*mapEntry).address,
                ((*mapEntry).op).count(),
                0 as usize as Boole,
            );
            theAddress = (*mapEntry).address;
            theSourceAddress = LispObjData((*mapEntry).data) as isize;
            i = 0;
            while i < ((*mapEntry).op).count() as usize {
                VirtualMemoryRead(theSourceAddress, &mut q);
                VirtualMemoryWrite(theAddress, &mut q);
                i += 1;
                theAddress = theAddress.wrapping_add(1);
                theSourceAddress = theSourceAddress.wrapping_add(1);
            }
            current_block_32 = 10692455896603418738;
        }
        _ => {
            CloseWorldFile(world, 1 as usize as Boole);
            vpunt(
                 "" ,
                b"Unknown load map opcode %d in world file %s\0" as *const u8 as *const libc::c_char
                    as&str,
                ((*mapEntry).op).opcode(),
                (*((*mapEntry).world as *mut World)).pathname,
            );
            current_block_32 = 10692455896603418738;
        }
    }
    match current_block_32 {
        1068983259865828954 => {
            EnsureVirtualAddressRange(
                (*mapEntry).address,
                ((*mapEntry).op).count(),
                0 as usize as Boole,
            );
            VirtualMemoryWriteBlockConstant(
                (*mapEntry).address,
                &mut (*mapEntry).data,
                ((*mapEntry).op).count(),
                increment,
            );
        }
        _ => {}
    }
    return ((*mapEntry).op).count();
}

fn IvoryLoadMapData(
    mut world: *mut World,
    mut mapEntry: *mut LoadMapEntry,
) -> isize {
    let mut q: LispObj = LispObj {
        parts: _LispObj {
            tag: 0,
            data: C2RustUnnamed { u: 0 },
        },
    };
    let mut theAddress: isize = 0;
    let mut theSourceAddress: isize = 0;
    let mut increment: usize = 0;
    let mut i: usize = 0;
    EnsureVirtualAddressRange(
        (*mapEntry).address,
        ((*mapEntry).op).count(),
        0 as usize as Boole,
    );
    let mut current_block_21: u64;
    match ((*mapEntry).op).opcode() as usize {
        0 => {
            ReadIvoryWorldFilePage(world, LispObjData((*mapEntry).data));
            (*world).currentQNumber = 0;
            theAddress = (*mapEntry).address;
            i = 0;
            while i < ((*mapEntry).op).count() as usize {
                ReadIvoryWorldFileNextQ(world, &mut q);
                VirtualMemoryWrite(theAddress, &mut q);
                i += 1;
                theAddress = theAddress.wrapping_add(1);
            }
            current_block_21 = 7172762164747879670;
        }
        2 => {
            increment = 1;
            current_block_21 = 9729758764470221092;
        }
        1 => {
            current_block_21 = 9729758764470221092;
        }
        3 => {
            theAddress = (*mapEntry).address;
            theSourceAddress = LispObjData((*mapEntry).data) as isize;
            i = 0;
            while i < ((*mapEntry).op).count() as usize {
                VirtualMemoryRead(theSourceAddress, &mut q);
                VirtualMemoryWrite(theAddress, &mut q);
                i += 1;
                theAddress = theAddress.wrapping_add(1);
                theSourceAddress = theSourceAddress.wrapping_add(1);
            }
            current_block_21 = 7172762164747879670;
        }
        _ => {
            CloseWorldFile(world, 1 as usize as Boole);
            vpunt(
                 "" ,
                b"Unknown load map opcode %d in world file %s\0" as *const u8 as *const libc::c_char
                    as&str,
                ((*mapEntry).op).opcode(),
                (*world).pathname,
            );
            current_block_21 = 7172762164747879670;
        }
    }
    match current_block_21 {
        9729758764470221092 => {
            VirtualMemoryWriteBlockConstant(
                (*mapEntry).address,
                &mut (*mapEntry).data,
                ((*mapEntry).op).count(),
                increment,
            );
        }
        _ => {}
    }
    return ((*mapEntry).op).count();
}
