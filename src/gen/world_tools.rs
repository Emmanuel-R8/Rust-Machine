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
    pub tag: u32,
    pub data: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union QData {
    pub u: u32,
    pub s: i32,
    pub f: libc::c_float,
}
pub type in_addr_t = u32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct in_addr {
    pub s_addr: in_addr_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XParams {
    pub xpHostName: &str,
    pub xpHostAddress: libc::c_long,
    pub xpDisplay: u32,
    pub xpScreen: u32,
    pub xpInitialState: u32,
    pub xpGeometry: &str,
    pub xpForegroundColor: &str,
    pub xpBackgroundColor: &str,
    pub xpBorderColor: &str,
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
    pub outputFile: &str,
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
    pub worldSearchPath: &str,
    pub enableIDS: bool,
    pub virtualMemory: size_t,
    pub coldLoadXParams: XParams,
    pub generaXParams: XParams,
    pub diagnosticIPAddress: in_addr,
    pub interfaces: [NetworkInterface; 8],
    pub testFunction: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _LoadMapEntry {
    pub address: isize,
    pub op: C2RustUnnamed_0,
    pub data: QWord,
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
    pub pathname: &str,
    pub fd: u32,
    pub format: u32,
    pub byte_swapped: u32,
    pub vlm_data_page_base: u32,
    pub vlm_tags_page_base: u32,
    pub vlm_data_page: *mut Byte,
    pub vlm_tags_page: *mut Byte,
    pub ivory_data_page: *mut Byte,
    pub current_page_number: u32,
    pub current_Q_number: u32,
    pub parentWorld: *mut _World,
    pub sysoutGeneration: isize,
    pub sysoutTimestamp1: isize,
    pub sysoutTimestamp2: isize,
    pub sysoutParentTimestamp1: isize,
    pub sysoutParentTimestamp2: isize,
    pub nwired_map_entries: u32,
    pub wired_map_entries: *mut LoadMapEntry,
    pub n_merged_wired_map_entries: u32,
    pub merged_wired_map_entries: *mut LoadMapEntry,
    pub n_unwired_map_entries: u32,
    pub unwired_map_entries: *mut LoadMapEntry,
    pub n_merged_unwired_map_entries: u32,
    pub merged_unwired_map_entries: *mut LoadMapEntry,
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

pub fn LoadVLMDebugger(mut config: *mut VLMConfig) {
    let mut world: World = World {
        pathname: "",
        fd: 0,
        format: 0,
        byte_swapped: 0,
        vlm_data_page_base: 0,
        vlm_tags_page_base: 0,
        vlm_data_page: 0 as *mut Byte,
        vlm_tags_page: 0 as *mut Byte,
        ivory_data_page: 0 as *mut Byte,
        current_page_number: 0,
        current_Q_number: 0,
        parentWorld: 0 as *mut _World,
        sysoutGeneration: 0,
        sysoutTimestamp1: 0,
        sysoutTimestamp2: 0,
        sysoutParentTimestamp1: 0,
        sysoutParentTimestamp2: 0,
        nwired_map_entries: 0,
        wired_map_entries: 0 ,
        n_merged_wired_map_entries: 0,
        merged_wired_map_entries: 0 ,
        n_unwired_map_entries: 0,
        unwired_map_entries: 0 ,
        n_merged_unwired_map_entries: 0,
        merged_unwired_map_entries: 0 ,
    };
    let mut i: u32 = 0;
    world.pathname = ((*config).vlmDebuggerPath).as_mut_ptr();
    open_world_file(&mut world, true);
    if world.n_unwired_map_entries > 0  {
        CloseWorldFile(&mut world, true);
        vpunt(
            "",
            b"World file %s contains unwired pages; it can't be a VLM debugger\0"
                 as &str,
            world.pathname,
        );
    }
    i = 0;
    while i < world.nwired_map_entries {
        LoadMapData(&mut world, &mut *(world.wired_map_entries).offset(i ));
        i += 1;
    }
    CloseWorldFile(&mut world, true);
}

pub fn LoadWorld(mut config: *mut VLMConfig) -> isize {
    let mut world: World = World {
        pathname: "",
        fd: 0,
        format: 0,
        byte_swapped: 0,
        vlm_data_page_base: 0,
        vlm_tags_page_base: 0,
        vlm_data_page: 0 as *mut Byte,
        vlm_tags_page: 0 as *mut Byte,
        ivory_data_page: 0 as *mut Byte,
        current_page_number: 0,
        current_Q_number: 0,
        parentWorld: 0 as *mut _World,
        sysoutGeneration: 0,
        sysoutTimestamp1: 0,
        sysoutTimestamp2: 0,
        sysoutParentTimestamp1: 0,
        sysoutParentTimestamp2: 0,
        nwired_map_entries: 0,
        wired_map_entries: 0 ,
        n_merged_wired_map_entries: 0,
        merged_wired_map_entries: 0 ,
        n_unwired_map_entries: 0,
        unwired_map_entries: 0 ,
        n_merged_unwired_map_entries: 0,
        merged_unwired_map_entries: 0 ,
    };
    let mut worldImageSize: isize = 0;
    let mut i: u32 = 0;
    world.pathname = ((*config).worldPath).as_mut_ptr();
    open_world_file(&mut world, true);
    merge_load_maps(&mut world, (*config).worldSearchPath);
    worldImageSize = 0 ;
    i = 0;
    while i < world.n_merged_wired_map_entries {
        worldImageSize = (worldImageSize).wrapping_add(LoadMapData(
            &mut world,
            &mut *(world.merged_wired_map_entries).offset(i ),
        ));
        i += 1;
    }
    i = 0;
    while i < world.n_merged_unwired_map_entries {
        worldImageSize = (worldImageSize).wrapping_add(LoadMapData(
            &mut world,
            &mut *(world.merged_unwired_map_entries).offset(i ),
        ));
        i += 1;
    }
    CloseWorldFile(&mut world, true);
    *__errno_location() = 0;
    return worldImageSize;
}

pub fn SaveWorld(mut saveWorldDataVMA: isize) {
    let mut world: World = World {
        pathname: "",
        fd: 0,
        format: 0,
        byte_swapped: 0,
        vlm_data_page_base: 0,
        vlm_tags_page_base: 0,
        vlm_data_page: 0 as *mut Byte,
        vlm_tags_page: 0 as *mut Byte,
        ivory_data_page: 0 as *mut Byte,
        current_page_number: 0,
        current_Q_number: 0,
        parentWorld: 0 as *mut _World,
        sysoutGeneration: 0,
        sysoutTimestamp1: 0,
        sysoutTimestamp2: 0,
        sysoutParentTimestamp1: 0,
        sysoutParentTimestamp2: 0,
        nwired_map_entries: 0,
        wired_map_entries: 0 ,
        n_merged_wired_map_entries: 0,
        merged_wired_map_entries: 0 ,
        n_unwired_map_entries: 0,
        unwired_map_entries: 0 ,
        n_merged_unwired_map_entries: 0,
        merged_unwired_map_entries: 0 ,
    };
    let mut saveWorldData: *mut SaveWorldData = 0 as *mut SaveWorldData;
    let mut saveWorldEntry: *mut SaveWorldEntry = 0 as *mut SaveWorldEntry;
    let mut loadMapEntry: *mut LoadMapEntry = 0 ;
    let mut pathnameHeader: QWord = LispObj {
        parts: _LispObj {
            tag: 0,
            data: QData { u: 0 },
        },
    };
    let mut pathnameSize: size_t = 0;
    let mut i: u32 = 0;
    saveWorldData = MapVirtualAddressData(saveWorldDataVMA) as *mut SaveWorldData;
    world.format = VLMWorldFormat;
    if 23
        != *MapVirtualAddressTag(
            (&mut (*saveWorldData).pathname as *mut isize)
                .offset_from(MapVirtualAddressData(0 ))
                ,
        )
    {
        vpunt(
            "",
            b"Destination pathname for SaveWorld is not a simple string\0"
                 as &str,
        );
    }
    VirtualMemoryRead((*saveWorldData).pathname, &mut pathnameHeader);
    if 3 != LispObjTag(pathnameHeader) & 0x3f {
        vpunt(
            "",
            b"Destination pathname for SaveWord is not a simple string\0"
                 as &str,
        );
    }
    if (LispObjData(pathnameHeader) & !(32767) )
        != 0x50000000
    {
        vpunt(
            "",
            b"Destination pathname for SaveWorld is not a simple string\0"
                 as &str,
        );
    }
    pathnameSize = (LispObjData(pathnameHeader) & 32767);
    world.pathname = malloc(pathnameSize.wrapping_add(1)) as &str;
    if (world.pathname).is_null() {
        vpunt(
            "",
            b"Unable to allocate space for local copy of destination pathname\0"
                 as &str,
        );
    }
    memcpy(
        world.pathname ,
        MapVirtualAddressData(((*saveWorldData).pathname).wrapping_add(1)) ,
        pathnameSize,
    );
    *(world.pathname).offset(pathnameSize ) = 0  ;
    i = 0;
    while (i) < pathnameSize {
        if '>' as i32 == *(world.pathname).offset(i )  {
            *(world.pathname).offset(i ) = '/' as i32 ;
        }
        i += 1;
    }
    world.nwired_map_entries = (*saveWorldData).entryCount;
    world.n_unwired_map_entries = 0;
    CreateWorldFile(&mut world);
    saveWorldEntry = &mut *((*saveWorldData).entries)
        .as_mut_ptr()
        .offset(0 ) as *mut SaveWorldEntry;
    loadMapEntry = world.wired_map_entries;
    i = 0;
    while i < world.nwired_map_entries {
        (*loadMapEntry).address = (*saveWorldEntry).address;
        let ref mut fresh0 = (*loadMapEntry).op;
        (*fresh0).set_opcode(LoadMapDataPages );
        let ref mut fresh1 = (*loadMapEntry).op;
        (*fresh1).set_count((*saveWorldEntry).extent);
        i += 1;
        saveWorldEntry = saveWorldEntry.offset(1);
        loadMapEntry = loadMapEntry.offset(1);
    }
    CanonicalizeVLMLoadMapEntries(&mut world);
    WriteVLMWorldFileHeader(&mut world);
    WriteVLMWorldFilePages(&mut world);
    CloseWorldFile(&mut world, true);
}

fn CreateWorldFile(mut world: *mut World) {
    let ref mut fresh16 = (*world).vlm_data_page;
    *fresh16 = 0 as *mut Byte;
    let ref mut fresh17 = (*world).vlm_tags_page;
    *fresh17 = 0 as *mut Byte;
    let ref mut fresh18 = (*world).ivory_data_page;
    *fresh18 = 0 as *mut Byte;
    let ref mut fresh19 = (*world).wired_map_entries;
    *fresh19 = 0 ;
    let ref mut fresh20 = (*world).unwired_map_entries;
    *fresh20 = 0 ;
    let ref mut fresh21 = (*world).merged_wired_map_entries;
    *fresh21 = 0 ;
    let ref mut fresh22 = (*world).merged_unwired_map_entries;
    *fresh22 = 0 ;
    let ref mut fresh23 = (*world).parentWorld;
    *fresh23 = 0 as *mut _World;
    if VLMWorldFormat  != (*world).format {
        vpunt(
            "",
            b"Cannot create world files in other than VLM format\0"
                 as &str,
        );
    }
    let ref mut fresh24 = (*world).fd;
    *fresh24 = open(
        (*world).pathname,
        0o1  | 0o100  | 0o1000,
        0o400 | 0o200 | 0o400  >> 3 | 0o400  >> 3  >> 3,
    );
    if *fresh24 < 0  {
        vpunt(
            "",
            b"Unable to create world file %s\0"   as &str,
            (*world).pathname,
        );
    }
    let ref mut fresh25 = (*world).ivory_data_page;
    *fresh25 = malloc(IVORY_PAGE_SIZE_BYTES) as *mut Byte;
    if ((*world).ivory_data_page).is_null() {
        CloseWorldFile(world, true);
        vpunt(
            "",
            b"Unable to allocate space for data buffer for world file %s\0"
                 as &str,
            (*world).pathname,
        );
    }
    (*world).current_page_number = -(1);
    if (*world).nwired_map_entries != 0 {
        let ref mut fresh26 = (*world).wired_map_entries;
        *fresh26 =
            malloc(((*world).nwired_map_entries).wrapping_mul(::std::mem::size_of::<LoadMapEntry>()))
                ;
        if ((*world).wired_map_entries).is_null() {
            CloseWorldFile(world, true);
            vpunt(
                "",
                b"Unable to allocate space for wired load map for world file %s\0"
                     as &str,
                (*world).pathname,
            );
        }
    }
    if (*world).n_unwired_map_entries != 0 {
        let ref mut fresh27 = (*world).unwired_map_entries;
        *fresh27 = malloc(
            ((*world).n_unwired_map_entries).wrapping_mul(::std::mem::size_of::<LoadMapEntry>()),
        ) ;
        if ((*world).unwired_map_entries).is_null() {
            CloseWorldFile(world, true);
            vpunt(
                "",
                b"Unable to allocate space for unwired load map for world file %s\0"
                     as &str,
                (*world).pathname,
            );
        }
    }
}

fn ReadLoadMap(mut world: *mut World, mut nMapEntries: u32, mut mapEntries: *mut LoadMapEntry) {
    let mut q: QWord = LispObj {
        parts: _LispObj {
            tag: 0,
            data: QData { u: 0 },
        },
    };
    let mut i: u32 = 0;
    i = 0;
    while i < nMapEntries {
        ReadIvoryWorldFileNextQ(world, &mut q);
        (*mapEntries).address = LispObjData(q) ;
        ReadIvoryWorldFileNextQ(world, &mut q);
        *(&mut (*mapEntries).op as *mut C2RustUnnamed_0 as *mut isize) = LispObjData(q) ;
        ReadIvoryWorldFileNextQ(world, &mut q);
        (*mapEntries).data = q;
        let ref mut fresh36 = (*mapEntries).world;
        *fresh36 = world as PtrV;
        i += 1;
        mapEntries = mapEntries.offset(1);
    }
}

fn LoadMapData(mut world: *mut World, mut mapEntry: *mut LoadMapEntry) -> isize {
    match (*world).format {
        0 => return VLMLoadMapData(world, mapEntry),
        1 => return IvoryLoadMapData(world, mapEntry),
        _ => {
            CloseWorldFile(world, true);
            vpunt(
                "",
                b"Format of world file %s is unrecognized\0"
                    as &str,
                (*world).pathname,
            );
        }
    }
    panic!("Reached end of non-void function without returning");
}

fn VLMLoadMapData(mut world: *mut World, mut mapEntry: *mut LoadMapEntry) -> isize {
    let mut q: QWord = LispObj {
        parts: _LispObj {
            tag: 0,
            data: QData { u: 0 },
        },
    };
    let mut mapWorld: *mut World = 0 as *mut World;
    let mut pageNumber: isize = 0;
    let mut theAddress: isize = 0;
    let mut theSourceAddress: isize = 0;
    let mut dataOffset: u32 = 0;
    let mut tagOffset: u32 = 0;
    let mut increment: u32 = 0;
    let mut i: u32 = 0;
    let mut current_block_32: u64;
    match ((*mapEntry).op).opcode()  {
        0 => {
            mapWorld = (*mapEntry).world as *mut World;
            pageNumber = LispObjData((*mapEntry).data) ;
            if (*mapWorld).byte_swapped != 0 {
                EnsureVirtualAddressRange(
                    (*mapEntry).address,
                    ((*mapEntry).op).count(),
                    false,
                );
                ReadSwappedVLMWorldFilePage(mapWorld, pageNumber);
                (*mapWorld).current_Q_number = 0;
                printf(
                    b"LoadMapDataPages @ %ld, count %d\n\0"  ,
                    theAddress,
                    ((*mapEntry).op).count(),
                );
                theAddress = (*mapEntry).address;
                i = 0;
                while i < ((*mapEntry).op).count()  {
                    ReadSwappedVLMWorldFileNextQ(mapWorld, &mut q);
                    VirtualMemoryWrite(theAddress, &mut q);
                    i += 1;
                    theAddress = theAddress.wrapping_add(1);
                }
            } else {
                dataOffset = (8192).wrapping_mul(
                    ((*mapWorld).vlm_data_page_base).wrapping_add(pageNumber.wrapping_mul(4)),
                ) ;
                tagOffset = (8192).wrapping_mul(
                    ((*mapWorld).vlm_tags_page_base).wrapping_add(pageNumber.wrapping_mul(1)),
                ) ;
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
                false,
            );
            theAddress = (*mapEntry).address;
            theSourceAddress = LispObjData((*mapEntry).data) ;
            i = 0;
            while i < ((*mapEntry).op).count()  {
                VirtualMemoryRead(theSourceAddress, &mut q);
                VirtualMemoryWrite(theAddress, &mut q);
                i += 1;
                theAddress = theAddress.wrapping_add(1);
                theSourceAddress = theSourceAddress.wrapping_add(1);
            }
            current_block_32 = 10692455896603418738;
        }
        _ => {
            CloseWorldFile(world, true);
            vpunt(
                "",
                b"Unknown load map opcode %d in world file %s\0"
                    as &str,
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
                false,
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

fn IvoryLoadMapData(mut world: *mut World, mut mapEntry: *mut LoadMapEntry) -> isize {
    let mut q: QWord = LispObj {
        parts: _LispObj {
            tag: 0,
            data: QData { u: 0 },
        },
    };
    let mut theAddress: isize = 0;
    let mut theSourceAddress: isize = 0;
    let mut increment: u32 = 0;
    let mut i: u32 = 0;
    EnsureVirtualAddressRange(
        (*mapEntry).address,
        ((*mapEntry).op).count(),
        false,
    );
    let mut current_block_21: u64;
    match ((*mapEntry).op).opcode()  {
        0 => {
            ReadIvoryWorldFilePage(world, LispObjData((*mapEntry).data));
            (*world).current_Q_number = 0;
            theAddress = (*mapEntry).address;
            i = 0;
            while i < ((*mapEntry).op).count()  {
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
            theSourceAddress = LispObjData((*mapEntry).data) ;
            i = 0;
            while i < ((*mapEntry).op).count()  {
                VirtualMemoryRead(theSourceAddress, &mut q);
                VirtualMemoryWrite(theAddress, &mut q);
                i += 1;
                theAddress = theAddress.wrapping_add(1);
                theSourceAddress = theSourceAddress.wrapping_add(1);
            }
            current_block_21 = 7172762164747879670;
        }
        _ => {
            CloseWorldFile(world, true);
            vpunt(
                "",
                b"Unknown load map opcode %d in world file %s\0"
                    as &str,
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
