#![feature(use_extern_macros, prox_macro_non_items)]

extern crate interpolate;

use std::fs::{File, read_dir, DirEntry};
use std::path::Path;
use std::process;
use log::warn;

use interpolate::s;
use close_file::Closable;

use crate::common::types::LispQ;

// PtrV is like Ptr but with better error checking
// pub type PtrV = &mut u32;

// A single load map entry -- See SYS:NETBOOT;WORLD-SUBSTRATE.LISP for details
#[derive(Debug)]
pub struct LoadMapEntry {
    address: u32,              // VMA to be filled in by this load map entry
    opcount: u32,              // Number of words to be filled in by this entry
    opcode: u32,               // An LoadMapEntryOpcode specifying how to do so
    data: LispQ,             // Interpretation is based on the opcode
    world: Option<Box<World>>, // -> World from which this entry was obtained
}

// Load map operation codes
#[derive(Debug)]
pub enum LoadMapEntryOpcode {
    DataPages,           // Load data pages from the file
    Constant,            // Store a constant into memory
    ConstantIncremented, // Store an auto-incrementing constant into memory
    Copy,                // Copy an existing piece of memory
}

// Description of an open world file
#[derive(Debug)]
pub struct World {
    pathname: Option<Path>,                                 // -> Pathname of the world file
    fd: Option<File>,                                 // Unix filedes # if the world file is open
    format: u32,                      // A LoadFileFormat indicating the type of file
    byte_swapped: bool,               // World is byte swapped on this machine (VLM only)
    vlm_data_page_base: u32,          // Block number of first page of data (VLM only)
    vlm_tags_page_base: u32,          // Block number of first page of tags (VLM only)
    vlm_data_page: u8,                // -> The data of the current VLM format page
    vlm_tags_page: u8,                // -> The tags of the current VLM format page
    ivory_data_page: u8,              // -> The data of the current Ivory format page
    current_page_number: u32,         // Page number of the page in the buffer, if any
    current_q_number: u32,             // Q number within the page to be read
    parent_world: Option<Box<World>>, // -> Parent of this world if it's an IDS
    sysout_generation: u32,           // Generation number of this world (> 0 if IDS)
    sysout_timestamp1: u32,           // Unique ID of this world, part 1 ...
    sysout_timestamp2: u32,           // ... part 2
    sysout_parent_timestamp1: u32,    // Unique ID of this world's parent, part 1 ...
    sysout_parent_timestamp2: u32,    // ... part 2
    n_wired_map_entries: u32,         // Number of wired load map entries
    wired_map_entries: Option<LoadMapEntry>, // -> The wired load map entries
    n_merged_wired_map_entries: u32,  // As above but after merging with parent worlds
    merged_wired_map_entries: Option<LoadMapEntry>, // ..
    n_unwired_map_entries: u32,       // Number of unwired load map entries (Ivory only)
    unwired_map_entries: Option<LoadMapEntry>, // -> The unwired load map entries (Ivory only)
    n_merged_unwired_map_entries: u32, // As above but after merging with parent worlds
    merged_unwired_map_entries: Option<LoadMapEntry>, // ..
}

impl World {
    pub fn new(mut self){
        World {
            pathname: String::from(""),
            fd: None,
            format: 0,
            byte_swapped: false,
            vlm_data_page_base: 0,
            vlm_tags_page_base: 0,
            vlm_data_page: 0,
            vlm_tags_page: 0,
            ivory_data_page: 0,
            current_page_number: 0,
            current_q_number: 0,
            parent_world: None,
            sysout_generation: 0,
            sysout_timestamp1: 0,
            sysout_timestamp2: 0,
            sysout_parent_timestamp1: 0,
            sysout_parent_timestamp2: 0,
            n_wired_map_entries: 0,
            wired_map_entries: None,
            n_merged_wired_map_entries: 0,
            merged_wired_map_entries: None,
            n_unwired_map_entries: 0,
            unwired_map_entries: None,
            n_merged_unwired_map_entries: 0,
            merged_unwired_map_entries: None,
        }
    }
}


// Possible world file formats
#[derive(Debug)]
pub enum LoadFileFormat {
    VLMWorldFormat,   // VLM world file (.VLOD)
    IvoryWorldFormat, // Ivory world file (.ILOD)
}

// Block numbers of the first page of data and tags for a VLM world as stored in its header
#[derive(Debug)]
pub struct _VLMPageBases {
    // #if BYTE_ORDER == LITTLE_ENDIAN
    data_page_base: u32,
    tags_page_base: u32, // Limits header and load maps to 112K bytes
                         // #else
                         //   isize tagsPageBase : 4; // Limits header and load maps to 112K bytes
                         //   isize dataPageBase : 28;
                         // #endif
}

pub const VLMPAGE_BASES: _VLMPageBases = _VLMPageBases {
    data_page_base: 28,
    tags_page_base: 4,
};

// // Data structures passed by Lisp via the SaveWorld coprocessor register
#[derive(Debug)]
pub struct SaveWorldEntry {
    address: u32, // VMA of data (usually a region) to be saved
    extent: u32,  // Number of words starting at this address to save
}

#[derive(Debug)]
pub struct SaveWorldData {
    pathname: u32,                   // Pathname of the world file (a DTP-STRING)
    entry_count: u32,                // Number of address/extent pairs to follow
    entries: Option<SaveWorldEntry>, //
}

// Close a world file
impl World {
    pub fn close(&self, close_parent: bool) {
        match self.fd {
            Some(f) => {
                f.close().unwrap();
            }
            None => {}
        }

        self.vlm_data_page = 0;
        self.vlm_tags_page = 0;
        self.ivory_data_page = 0;
        self.merged_wired_map_entries = None;
        self.wired_map_entries = None;
        self.merged_unwired_map_entries = None;
        self.unwired_map_entries = None;

        if close_parent && self.parent_world.is_some() {
            self.parent_world.unwrap().close(true);
            self.parent_world = None;
        }
    }
}

// Print an error message and terminate the VLM
pub fn vpunt(msg: String) {
    warn!("Emulator final log");
    warn!("{}", msg);
    warn!("----------------------------------------------------------");
    warn!("TERMINATING EMULATOR\n");

    process::exit(1);
}

pub fn punt_world(world: World, msg: String ) {
    world.close(true);
    vpunt(msg);
}

pub fn MergeLoadMaps(mut world: World, world_search_path: String) {
    if world.sysoutGeneration > 0  {
        let originalWorld = world;
        FindParentWorlds(world, world_search_path);
        MergeParentLoadMap(world);
    } else {
        world.n_merged_wired_map_entries = world.n_wired_map_entries;
        let ref mut fresh37 = world.merged_wired_map_entries;
        *fresh37 = world.wiredMapEntries;
        world.nMergedUnwiredMapEntries = world.n_unwired_map_entries;
        let ref mut fresh38 = world.merged_unwired_map_entries;
        *fresh38 = world.unwired_map_entries;
    };
}

#[derive(Debug)]
pub struct GlobalContext{
     originalWorld:  Option<World>,
  worlds: Option<World> ,
  total_worlds: usize,
  n_worlds: usize ,
  scanning_dir: Option<Path>
}

impl GlobalContext {
    pub fn new(mut self) {
    GlobalContext{
        originalWorld:None,
         worlds: None,
  total_worlds: 0,
  n_worlds:  0,
  scanning_dir: None
}

    }
}

fn FindParentWorlds(
    mut world: &World,
    mut world_search_path:String, mut ctx: GlobalContext
) {
    let mut child_world:  World;
    let mut failing_world_pathname: String = String::from( "") ;
    let mut slash_position: String = String::from( "") ;
    let mut colon_position: String = String::from( "") ;
    let mut i: usize = 0;

    ctx.n_worlds = 0 ;
    ctx.total_worlds = 0;
    ctx.worlds = None;

    let scanning_dir = world.pathname;
    slash_position = String::strrchr(scanning_dir, "/")  ;
    if !slash_position.is_null() {
        *slash_position = 0 as usize as libc::c_char;
    } else {
        world.close(true);
        vpunt(interpolate::s!("Unable to determine pathname of directory containing world file {world.pathname}"));
    }
    ScanOneDirectory(world);
    colon_position = world_search_path.find(':');
    while !colon_position.is_null() {
        *colon_position = 0 as usize as libc::c_char;
        scanning_dir = world_search_path;
        ScanOneDirectory(world);
        world_search_path = colon_position.offset(1 as usize as isize);
        colon_position = world_search_path.find(':');
    }
    if world_search_path.len() > 0 {
        scanning_dir = world_search_path;
        ScanOneDirectory(world);
    }

    child_world = world;
    while child_world.sysoutGeneration != 0 {
        i = 0;
        while i < ctx.n_worlds {
            if !ctx.worlds.offset(i).is_null()
                && ctx.worlds.offset(i as isize).sysoutGeneration
                    == (ctx.child_world.sysoutGeneration)
                        .wrapping_sub(1 as usize as libc::c_ulong)
                && ctx.worlds.offset(i as isize).sysoutTimestamp1
                    == ctx.child_world.sysoutParentTimestamp1
                && ctx.worlds.offset(i as isize).sysoutTimestamp2
                    == ctx.child_world.sysoutParentTimestamp2
            {
                let ref mut fresh39 = (*child_world).parentWorld;
                *fresh39 = ctx.worlds.offset(i as isize);
                let ref mut fresh40 = ctx.worlds.offset(i as isize);
                *fresh40 = 0 as *mut World;
                break;
            } else {
                i += 1;
            }
        }
        if ((*child_world).parentWorld).is_null() {
            failing_world_pathname = child_world.pathname;
            CloseExtraWorlds();
            world.close(true);
            vpunt(interpolate::s!("Unable to find parent of world file {failingWorldPathname}"  ));
        }
        child_world = child_world.parentWorld;
    }
    CloseExtraWorlds();
}

fn  ScanOneDirectory(dir: Path, ctx:GlobalContext) {
    // scan the directory specified in the global context and only keeps the world files.
    let dir_reader = read_dir(ctx.scanning_dir).filter(world_p);
    let (_, n_entries) = dir_reader.unwrap().size_hint();

    if n_entries.is_none() | n_entries.unwrap()==0 {
            CloseExtraWorlds();
            vpunt(interpolate::s!("Unable to search directory {dir.as_os_str().as_string()} to find parents of world file."));
    }

    dir_reader
}


pub fn  world_p(candidate_world: DirEntry, ctx:GlobalContext) -> bool {
    let mut a_world = World.new();
    let mut new_worlds: [World];
    let mut candidate_pathname: Path;
    let mut name_len: usize = 0;
    let mut new_total_worlds: usize = 0;
    let mut i: usize = 0;

    name_len = candidate_world.as_os_str().len();
    if name_len > ".vlod".as_string().len() {
        sprintf(
            candidate_pathname.as_mut_ptr(),
            b"%s/%s\0" as *const u8 as *const libc::c_char,
            scanning_dir,
            ((*candidate_world).d_name).as_ptr(),
        );

        a_world.pathname = Path(candidate_pathname);

        strndup(
            candidate_pathname.as_mut_ptr(),
            (256 as usize + 1) as libc::c_ulong,
        );
        if OpenWorldFile(&mut a_world, 0 as usize as bool) != 0 {
            if ctx.n_worlds == ctx.total_worlds {
                new_total_worlds = ctx.total_worlds + 32;
                new_worlds = malloc(
                    (::std::mem::size_of::<*mut World>() as libc::c_ulong)
                        .wrapping_mul(new_total_worlds as libc::c_ulong),
                ) as *mut *mut World;
                if new_worlds.is_null() {
                    CloseExtraWorlds();
                    a_world.close(true);
                    ctx.originalWorld.close(true);
                    vpunt("Unable to allocate space to search for parents of world file {originalWorld.pathname}".as_string());
                }
                memcpy(
                    new_worlds as *mut libc::c_void,
                    worlds as *const libc::c_void,
                    (ctx.total_worlds as libc::c_ulong)
                        .wrapping_mul(::std::mem::size_of::<*mut World>() as libc::c_ulong),
                );
                free(worlds as *mut libc::c_void);
                worlds = new_worlds;
                ctx.total_worlds = new_total_worlds;
            }
            let ref mut fresh41 = *worlds.offset(nWorlds as isize);
            *fresh41 = malloc(::std::mem::size_of::<World>() as libc::c_ulong) as *mut World;
            if worlds.offset(nWorlds).is_null() {
                CloseExtraWorlds();
                a_world.close(true);
                ctx.original_world.close(true);
                vpunt(interpolate::s!("Unable to allocate space to search for parents of world file {originalWorld.pathname}".as_string()));
            }
            memcpy(
                worlds.offset(nWorlds as isize) as *mut libc::c_void,
                &mut a_world as *mut World as *const libc::c_void,
                ::std::mem::size_of::<World>() as libc::c_ulong,
            );
            i = 0;
            while i < worlds.offset(ctx.n_worlds ).nWiredMapEntries {
                let ref mut fresh42 = (*((**worlds.offset(nWorlds as isize)).wiredMapEntries)
                    .offset(i as isize))
                .world;
                fresh42 = worlds.offset(ctx.n_worlds ) ;
                i += 1;
            }
            i = 0;
            while i < worlds.offset(ctx.n_worlds).nUnwiredMapEntries {
                let ref mut fresh43 = (*((**worlds.offset(nWorlds as isize)).unwiredMapEntries)
                    .offset(i as isize))
                .world;
                *fresh43 = worlds.offset(crx.n_worlds);
                i += 1;
            }
            ctx.n_worlds += 1;
            return 1;
        } else {
            return 0;
        }
    } else {
        return 0;
    };
}


fn  CloseExtraWorlds() {
    let mut i: usize = 0;
    if worlds.is_null() {
        return;
    }
    i = 0;
    while i < nWorlds {
        if !worlds.offset(i as isize).is_null() {
            worlds.offset(i as isize).close(true);
            free(*worlds.offset(i as isize) as *mut libc::c_void);
        }
        i += 1;
    }
    free(worlds as *mut libc::c_void);
    worlds = 0 as *mut *mut World;
    totalWorlds = 0;
}


fn  MergeParentLoadMap(mut world: *mut World) {
    if world.sysoutGeneration == 0 as usize as libc::c_ulong {
        world.nMergedWiredMapEntries = world.nWiredMapEntries;
        let ref mut fresh44 = world.mergedWiredMapEntries;
        *fresh44 = world.wiredMapEntries;
        world.nMergedUnwiredMapEntries = world.nUnwiredMapEntries;
        let ref mut fresh45 = world.mergedUnwiredMapEntries;
        *fresh45 = world.unwiredMapEntries;
        return;
    }
    MergeParentLoadMap((*world).parentWorld);
    MergeAMap(
        (*world).nWiredMapEntries,
        (*world).wiredMapEntries,
        (*(*world).parentWorld).nMergedWiredMapEntries,
        (*(*world).parentWorld).mergedWiredMapEntries,
        &mut (*world).nMergedWiredMapEntries,
        &mut (*world).mergedWiredMapEntries,
    );
    MergeAMap(
        (*world).nUnwiredMapEntries,
        (*world).unwiredMapEntries,
        (*(*world).parentWorld).nMergedUnwiredMapEntries,
        (*(*world).parentWorld).mergedUnwiredMapEntries,
        &mut (*world).nMergedUnwiredMapEntries,
        &mut (*world).mergedUnwiredMapEntries,
    );
}


fn  MergeAMap(
    mut nForeground: u32,
    mut foreground: *mut LoadMapEntry,
    mut nBackground: u32,
    mut background: *mut LoadMapEntry,
    mut nMerged: *mut u32,
    mut merged: *mut *mut LoadMapEntry,
) {
    let mut new: *mut LoadMapEntry = 0 as *mut LoadMapEntry;
    let mut pageSizeQs: isize = (if VLMWorldFormat as usize == (*originalWorld).format {
        8192
    } else {
        256
    }) as isize;
    let mut oldAddress: isize = 0;
    let mut slop: isize = 0;
    let mut max: usize = 0;
    let mut actual: usize = 0;
    let mut fIndex: usize = 0;
    let mut bIndex: usize = 0;
    let mut copiedForeground: bool = false;
    max = nBackground + nForeground + nForeground;
    actual = 0;
    if 0 as usize == max {
        *nMerged = 0;
        *merged = 0 as *mut LoadMapEntry;
        return;
    }
    new = malloc(
        (max as libc::c_ulong).wrapping_mul(::std::mem::size_of::<LoadMapEntry>() as libc::c_ulong),
    ) as *mut LoadMapEntry;
    if new.is_null() {
        CloseExtraWorlds();
        world.close(true);
        vpunt(
             "" ,
            b"Unable to allocate space to compute merged load map for world file %s\0" as *const u8
                as *const libc::c_char as&str,
            (*originalWorld).pathname,
        );
    }
    fIndex = 0;
    bIndex = 0;
    copiedForeground = 0 as usize as bool;
    while fIndex < nForeground {
        while bIndex < nBackground
            && (((*background.offset(bIndex as isize)).op).opcode()
                != LoadMapDataPages
                || (*background.offset(bIndex as isize)).address
                    < (*foreground.offset(fIndex as isize)).address
                    && ((*background.offset(bIndex as isize)).address)
                        .wrapping_add(((*background.offset(bIndex as isize)).op).count())
                        < (*foreground.offset(fIndex as isize)).address)
        {
            memcpy(
                &mut *new.offset(actual as isize) as *mut LoadMapEntry as *mut libc::c_void,
                &mut *background.offset(bIndex as isize) as *mut LoadMapEntry
                    as *const libc::c_void,
                ::std::mem::size_of::<LoadMapEntry>() as libc::c_ulong,
            );
            bIndex += 1;
            actual += 1;
        }
        if ((*foreground.offset(fIndex as isize)).op).opcode()
            != LoadMapDataPages
            && copiedForeground == 0
        {
            memcpy(
                &mut *new.offset(actual as isize) as *mut LoadMapEntry as *mut libc::c_void,
                &mut *foreground.offset(fIndex as isize) as *mut LoadMapEntry
                    as *const libc::c_void,
                ::std::mem::size_of::<LoadMapEntry>() as libc::c_ulong,
            );
            actual += 1;
            copiedForeground = 1 as usize as bool;
        } else {
            if (*background.offset(bIndex as isize)).address
                < (*foreground.offset(fIndex as isize)).address
            {
                memcpy(
                    &mut *new.offset(actual as isize) as *mut LoadMapEntry as *mut libc::c_void,
                    &mut *background.offset(bIndex as isize) as *mut LoadMapEntry
                        as *const libc::c_void,
                    ::std::mem::size_of::<LoadMapEntry>() as libc::c_ulong,
                );
                let ref mut fresh46 = (*new.offset(actual as isize)).op;
                (*fresh46).set_count(
                    ((*foreground.offset(fIndex as isize)).address)
                        .wrapping_sub((*background.offset(bIndex as isize)).address),
                );
                actual += 1;
            }
            if copiedForeground == 0 {
                memcpy(
                    &mut *new.offset(actual as isize) as *mut LoadMapEntry as *mut libc::c_void,
                    &mut *foreground.offset(fIndex as isize) as *mut LoadMapEntry
                        as *const libc::c_void,
                    ::std::mem::size_of::<LoadMapEntry>() as libc::c_ulong,
                );
                actual += 1;
                copiedForeground = 1 as usize as bool;
            }
            if (*background.offset(bIndex as isize)).address
                < ((*foreground.offset(fIndex as isize)).address)
                    .wrapping_add(((*foreground.offset(fIndex as isize)).op).count())
            {
                if ((*background.offset(bIndex as isize)).address)
                    .wrapping_add(((*background.offset(bIndex as isize)).op).count())
                    > ((*foreground.offset(fIndex as isize)).address)
                        .wrapping_add(((*foreground.offset(fIndex as isize)).op).count())
                {
                    oldAddress = (*background.offset(bIndex as isize)).address;
                    (*background.offset(bIndex as isize)).address =
                        ((*foreground.offset(fIndex as isize)).address)
                            .wrapping_add(((*foreground.offset(fIndex as isize)).op).count());
                    let ref mut fresh47 = (*background.offset(bIndex as isize)).op;
                    (*fresh47).set_count(
                        (*fresh47).count()
                            - ((*foreground.offset(fIndex as isize)).address)
                                .wrapping_add(((*foreground.offset(fIndex as isize)).op).count())
                                .wrapping_sub(oldAddress) as isize,
                    );
                    slop = (*background.offset(bIndex as isize)).address
                        & pageSizeQs.wrapping_sub(1 as usize as libc::c_ulong);
                    if slop != 0 as usize as libc::c_ulong {
                        let ref mut fresh48 = (*background.offset(bIndex as isize)).address;
                        *fresh48 = (*fresh48 as libc::c_ulong)
                            .wrapping_add(pageSizeQs.wrapping_sub(slop))
                            as isize as isize;
                        let ref mut fresh49 = (*background.offset(bIndex as isize)).op;
                        (*fresh49).set_count((*fresh49).count() - slop as isize);
                        if ((*background.offset(bIndex as isize)).op).count()
                            <= 0
                        {
                            CloseExtraWorlds();
                            originalWorld.close(true);
                            vpunt(
                                 "" ,
                                b"A merged load map entry wouldn't start on a page boundary for world file %s\0"
                                    as *const u8 as *const libc::c_char as&str,
                                (*originalWorld).pathname,
                            );
                        }
                    }
                    WriteLispObjData(
                        &mut (*background.offset(bIndex as isize)).data,
                        (LispObjData((*background.offset(bIndex as isize)).data) as libc::c_ulong)
                            .wrapping_add(
                                ((*background.offset(bIndex as isize)).address)
                                    .wrapping_sub(oldAddress)
                                    .wrapping_div(pageSizeQs),
                            ) as ui32,
                    );
                } else {
                    bIndex += 1;
                }
            }
        }
        if bIndex >= nBackground
            || (*background.offset(bIndex as isize)).address
                >= ((*foreground.offset(fIndex as isize)).address)
                    .wrapping_add(((*foreground.offset(fIndex as isize)).op).count())
        {
            fIndex += 1;
            copiedForeground = 0 as usize as bool;
        }
    }
    while bIndex < nBackground {
        memcpy(
            &mut *new.offset(actual as isize) as *mut LoadMapEntry as *mut libc::c_void,
            &mut *background.offset(bIndex as isize) as *mut LoadMapEntry as *const libc::c_void,
            ::std::mem::size_of::<LoadMapEntry>() as libc::c_ulong,
        );
        bIndex += 1;
        actual += 1;
    }
    *nMerged = actual;
    *merged = new;
}


fn  CanonicalizeVLMLoadMapEntries(mut world: *mut World) {
    let mut mapEntry: *mut LoadMapEntry = 0 as *mut LoadMapEntry;
    let mut newWiredMapEntries: *mut LoadMapEntry = 0 as *mut LoadMapEntry;
    let mut newMapEntry: *mut LoadMapEntry = 0 as *mut LoadMapEntry;
    let mut pageNumber: isize = 0;
    let mut pageCount: isize = 0;
    let mut blockCount: isize = 0;
    let mut nQs: isize = 0;
    let mut newNWiredMapEntries: usize = 0;
    let mut i: usize = 0;
    let mut j: usize = 0;
    let mut q: LispQ = LispQ {
        parts: _LispObj {
            tag: 0,
            data: C2RustUnnamed { u: 0 },
        },
    };
    pageNumber = 0 as usize as isize;
    i = 0;
    while i < (*world).nWiredMapEntries {
        mapEntry = &mut *((*world).wiredMapEntries).offset(i as isize) as *mut LoadMapEntry;
        if 0 as usize as libc::c_ulong
            == (*mapEntry).address & (8192 as usize - 1) as libc::c_ulong
        {
            pageCount = ((((*mapEntry).op).count() as usize + 8192
                - 1)
                / 8192) as isize;
            (*mapEntry).data = *MakeLispObj(8 as usize as ui32, pageNumber as ui32);
            pageNumber =
                (pageNumber as libc::c_ulong).wrapping_add(pageCount) as isize as isize;
            i += 1;
        } else {
            newNWiredMapEntries = (*world).nWiredMapEntries
                + ((*mapEntry).op).count()
                - 1;
            newWiredMapEntries = malloc(
                (newNWiredMapEntries as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<LoadMapEntry>() as libc::c_ulong),
            ) as *mut LoadMapEntry;
            if newWiredMapEntries.is_null() {
                world.close(true);
                vpunt(
                     "" ,
                    b"Unable to allocate space for wired load map for world file %s\0" as *const u8
                        as *const libc::c_char as&str,
                    (*world).pathname,
                );
            }
            memcpy(
                newWiredMapEntries as *mut libc::c_void,
                (*world).wiredMapEntries as *const libc::c_void,
                (i as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<LoadMapEntry>() as libc::c_ulong),
            );
            memcpy(
                &mut *newWiredMapEntries
                    .offset((i + ((*mapEntry).op).count()) as isize)
                    as *mut LoadMapEntry as *mut libc::c_void,
                &mut *((*world).wiredMapEntries).offset((i + 1) as isize)
                    as *mut LoadMapEntry as *const libc::c_void,
                (((*world).nWiredMapEntries - i) as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<LoadMapEntry>() as libc::c_ulong),
            );
            j = 0;
            while j < ((*mapEntry).op).count() as usize {
                newMapEntry =
                    &mut *newWiredMapEntries.offset((i + j) as isize) as *mut LoadMapEntry;
                (*newMapEntry).address = ((*mapEntry).address).wrapping_add(j as libc::c_ulong);
                let ref mut fresh50 = (*newMapEntry).op;
                (*fresh50).set_opcode(LoadMapConstant as usize as isize);
                let ref mut fresh51 = (*newMapEntry).op;
                (*fresh51).set_count(1 as usize as isize);
                VirtualMemoryRead((*newMapEntry).address, &mut q);
                (*newMapEntry).data = q;
                j += 1;
            }
            i += ((*mapEntry).op).count();
            free((*world).wiredMapEntries as *mut libc::c_void);
            (*world).nWiredMapEntries = newNWiredMapEntries;
            let ref mut fresh52 = (*world).wiredMapEntries;
            *fresh52 = newWiredMapEntries;
        }
    }
    nQs = (8 as usize + 3 as usize * (*world).nWiredMapEntries) as isize;
    pageCount = nQs
        .wrapping_add(256 as usize as libc::c_ulong)
        .wrapping_sub(1 as usize as libc::c_ulong)
        .wrapping_div(256 as usize as libc::c_ulong);
    blockCount = pageCount
        .wrapping_mul(1280 as usize as libc::c_ulong)
        .wrapping_add(8192 as usize as libc::c_ulong)
        .wrapping_sub(1 as usize as libc::c_ulong)
        .wrapping_div(8192 as usize as libc::c_ulong);
    if blockCount > 14 as usize as libc::c_ulong {
        world.close(true);
        vpunt(
             "" ,
            b"Unable to store data map in space reserved for same in world file %s\0" as *const u8
                as *const libc::c_char as&str,
            (*world).pathname,
        );
    }
    (*world).vlm_tags_page_base = blockCount;
    (*world).vlm_data_page_base = ((*world).vlm_tags_page_base as libc::c_ulong)
        .wrapping_add((1 as usize as libc::c_ulong).wrapping_mul(pageNumber))
       ;
}


fn  WriteVLMWorldFileHeader(mut world: *mut World) {
    let mut mapEntry: *mut LoadMapEntry = 0 as *mut LoadMapEntry;
    let mut generationQ: LispQ = LispQ {
        parts: _LispObj {
            tag: 0,
            data: C2RustUnnamed { u: 0 },
        },
    };
    let mut q: *mut LispQ = 0 as *mut LispQ;
    let mut pageBases: isize = 0;
    let mut nBlocks: off_t = 0;
    let mut i: usize = 0;
    i = (*world).nWiredMapEntries;
    while i > 0 as usize {
        mapEntry = &mut *((*world).wiredMapEntries).offset((i - 1) as isize)
            as *mut LoadMapEntry;
        if LoadMapDataPages as usize == ((*mapEntry).op).opcode() as usize {
            nBlocks = ((*world).vlm_data_page_base as libc::c_uint).wrapping_add(
                (LispObjData((*mapEntry).data))
                    .wrapping_add(
                        (((*mapEntry).op).count() as usize / 8192)
                            as libc::c_uint,
                    )
                    .wrapping_add(1 as usize as libc::c_uint)
                    .wrapping_mul(4 as usize as libc::c_uint),
            ) as off_t;
            if ftruncate((*world).fd, nBlocks * 8192 as usize as libc::c_long)
                < 0
            {
                world.close(true);
                vpunt(
                     "" ,
                    b"Unable to grow world file %s to %d bytes\0" as *const u8
                        as *const libc::c_char as&str,
                    (*world).pathname,
                    nBlocks * 8192 as usize as libc::c_long,
                );
            }
            break;
        } else {
            i -= 1;
        }
    }
    PrepareToWriteIvoryWorldFilePage(world, 0);
    let ref mut fresh53 = *(&mut pageBases as *mut isize as *mut VLMPageBases);
    (*fresh53).set_dataPageBase((*world).vlm_data_page_base as isize);
    let ref mut fresh54 = *(&mut pageBases as *mut isize as *mut VLMPageBases);
    (*fresh54).set_tagsPageBase((*world).vlm_tags_page_base as isize);
    WriteIvoryWorldFileNextQ(
        world,
        *MakeLispObj(
            (((2) << 6) + 8) as ui32,
            0o40000201 as usize as ui32,
        ),
    );
    WriteIvoryWorldFileNextQ(
        world,
        *MakeLispObj(
            (((2) << 6) + 9) as ui32,
            (*world).nWiredMapEntries as ui32,
        ),
    );
    WriteIvoryWorldFileNextQ(
        world,
        *MakeLispObj(
            (((2) << 6) + 10) as ui32,
            pageBases as ui32,
        ),
    );
    ReadSystemCommSlot(sysoutGenerationNumber, &mut generationQ);
    WriteIvoryWorldFileNextQ(
        world,
        *MakeLispObj(
            (((2) << 6) + 35) as ui32,
            LispObjData(generationQ),
        ),
    );
    VirtualMemoryRead(
        (0xf8041100 as libc::c_long as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<EmbWord>() as libc::c_ulong)
            .wrapping_add(sysoutTimestamp1 as usize as ptrdiff_t as libc::c_ulong),
        q,
    );
    WriteIvoryWorldFileNextQ(world, *q);
    VirtualMemoryRead(
        (0xf8041100 as libc::c_long as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<EmbWord>() as libc::c_ulong)
            .wrapping_add(sysoutTimestamp2 as usize as ptrdiff_t as libc::c_ulong),
        q,
    );
    WriteIvoryWorldFileNextQ(world, *q);
    VirtualMemoryRead(
        (0xf8041100 as libc::c_long as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<EmbWord>() as libc::c_ulong)
            .wrapping_add(sysoutParentTimestamp1 as usize as ptrdiff_t as libc::c_ulong),
        q,
    );
    WriteIvoryWorldFileNextQ(world, *q);
    VirtualMemoryRead(
        (0xf8041100 as libc::c_long as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<EmbWord>() as libc::c_ulong)
            .wrapping_add(sysoutParentTimestamp2 as usize as ptrdiff_t as libc::c_ulong),
        q,
    );
    WriteIvoryWorldFileNextQ(world, *q);
    i = 0;
    while i < (*world).nWiredMapEntries {
        mapEntry = &mut *((*world).wiredMapEntries).offset(i as isize) as *mut LoadMapEntry;
        q = MakeLispObj(
            25 as usize as ui32,
            (*mapEntry).address as ui32,
        );
        WriteIvoryWorldFileNextQ(world, *q);
        q = MakeLispObj(
            8 as usize as ui32,
            *(&mut (*mapEntry).op as *mut C2RustUnnamed_0 as *mut isize) as ui32,
        );
        WriteIvoryWorldFileNextQ(world, *q);
        WriteIvoryWorldFileNextQ(world, (*mapEntry).data);
        i += 1;
    }
    WriteIvoryWorldFilePage(world);
}


fn  WriteVLMWorldFilePages(mut world: *mut World) {
    let mut mapEntry: *mut LoadMapEntry = 0 as *mut LoadMapEntry;
    let mut pageNumber: isize = 0;
    let mut wordCount: size_t = 0;
    let mut byteCount: size_t = 0;
    let mut offset: off_t = 0;
    let mut increment: usize = 0;
    let mut i: usize = 0;
    i = 0;
    while i < (*world).nWiredMapEntries {
        mapEntry = &mut *((*world).wiredMapEntries).offset(i as isize) as *mut LoadMapEntry;
        if !(LoadMapDataPages as usize != ((*mapEntry).op).opcode()) {
            pageNumber = LispObjData((*mapEntry).data) as isize;
            wordCount = ((*mapEntry).op).count();
            offset = (8192 as usize as libc::c_ulong).wrapping_mul(
                ((*world).vlm_data_page_base as libc::c_ulong)
                    .wrapping_add(pageNumber.wrapping_mul(4 as usize as libc::c_ulong)),
            ) as off_t;
            byteCount = wordCount.wrapping_mul(::std::mem::size_of::<isize>() as libc::c_ulong);
            if offset != lseek((*world).fd, offset, 0) {
                world.close(true);
                vpunt(
                     "" ,
                    b"Unable to seek to offset %d in world file %s\0" as *const u8
                        as *const libc::c_char as&str,
                    offset,
                    (*world).pathname,
                );
            }
            if byteCount
                != write(
                    (*world).fd,
                    MapVirtualAddressData((*mapEntry).address) as *const libc::c_void,
                    byteCount,
                ) as libc::c_ulong
            {
                world.close(true);
                vpunt(
                     "" ,
                    b"Unable to write data page %d into world file %s\0" as *const u8
                        as *const libc::c_char as&str,
                    pageNumber,
                    (*world).pathname,
                );
            }
            offset = (8192 as usize as libc::c_ulong).wrapping_mul(
                ((*world).vlm_tags_page_base as libc::c_ulong)
                    .wrapping_add(pageNumber.wrapping_mul(1 as usize as libc::c_ulong)),
            ) as off_t;
            byteCount = wordCount.wrapping_mul(::std::mem::size_of::<Tag>() as libc::c_ulong);
            if offset != lseek((*world).fd, offset, 0) {
                world.close(true);
                vpunt(
                     "" ,
                    b"Unable to seek to offset %d in world file %s\0" as *const u8
                        as *const libc::c_char as&str,
                    offset,
                    (*world).pathname,
                );
            }
            if byteCount
                != write(
                    (*world).fd,
                    MapVirtualAddressTag((*mapEntry).address) as *const libc::c_void,
                    byteCount,
                ) as libc::c_ulong
            {
                world.close(true);
                vpunt(
                     "" ,
                    b"Unable to write tags page %d into world file %s\0" as *const u8
                        as *const libc::c_char as&str,
                    pageNumber,
                    (*world).pathname,
                );
            }
        }
        i += 1;
    }
}


fn  ReadIvoryWorldFilePage(mut world: *mut World, mut pageNumber: u32) {
    let mut offset: off_t = 0;
    if (*world).currentPageNumber == pageNumber {
        return;
    }
    offset = (pageNumber * 1280) as off_t;
    if offset != lseek((*world).fd, offset, 0) {
        world.close(true);
        vpunt(
             "" ,
            b"Unable to seek to offset %d in world file %s\0" as *const u8 as *const libc::c_char
                as&str,
            offset,
            (*world).pathname,
        );
    }
    if 1280 as usize as libc::c_long
        != read(
            (*world).fd,
            (*world).ivory_data_page as *mut libc::c_void,
            1280 as usize as size_t,
        )
    {
        world.close(true);
        vpunt(
             "" ,
            b"Unable to read page %d from world file %s\0" as *const u8 as *const libc::c_char
                as&str,
            pageNumber,
            (*world).pathname,
        );
    }
    (*world).currentPageNumber = pageNumber;
}


fn  ReadIvoryWorldFileQ(
    mut world: *mut World,
    mut qNumber: u32,
    mut q: *mut LispQ,
) {
    let mut pointerOffset: usize = 0;
    let mut tagOffset: usize = 0;
    let mut datum: isize = 0;
    if qNumber < 0 as usize || qNumber >= 256 as usize {
        world.close(true);
        vpunt(
             "" ,
            b"Invalid word number %d for world file %s\0" as *const u8 as *const libc::c_char
                as&str,
            qNumber,
            (*world).pathname,
        );
    }
    pointerOffset = 5 as usize * (qNumber >> 2)
        + (qNumber & 3)
        + 1;
    tagOffset = 4 as usize * 5 as usize * (qNumber >> 2)
        + (qNumber & 3);
    q = MakeLispObj(
        *((*world).ivory_data_page).offset(tagOffset as isize) as ui32,
        *((*world).ivory_data_page as *mut isize).offset(pointerOffset as isize) as ui32,
    );
}


fn  ReadIvoryWorldFileNextQ(mut world: *mut World, mut q: *mut LispQ) {
    while (*world).currentQNumber >= 256 as usize {
        ReadIvoryWorldFilePage(world, (*world).currentPageNumber + 1);
        (*world).currentQNumber -= 256;
    }
    ReadIvoryWorldFileQ(world, (*world).currentQNumber, q);
    let ref mut fresh55 = (*world).currentQNumber;
    *fresh55 += 1;
}
 fn  PrepareToWriteIvoryWorldFilePage(
    mut world: *mut World,
    mut pageNumber: u32,
) {
    (*world).currentPageNumber = pageNumber;
    (*world).currentQNumber = 0;
    memset(
        (*world).ivory_data_page as *mut libc::c_void,
        0,
        1280 as usize as libc::c_ulong,
    );
}


fn  WriteIvoryWorldFilePage(mut world: *mut World) {
    let mut offset: off_t = 0;
    if 0 as usize == (*world).currentQNumber {
        return;
    }
    offset = ((*world).currentPageNumber * 1280) as off_t;
    if offset != lseek((*world).fd, offset, 0) {
        world.close(true);
        vpunt(
             "" ,
            b"Unable to seek to offset %d in world file %s\0" as *const u8 as *const libc::c_char
                as&str,
            offset,
            (*world).pathname,
        );
    }
    if 1280 as usize as libc::c_long
        != write(
            (*world).fd,
            (*world).ivory_data_page as *const libc::c_void,
            1280 as usize as size_t,
        )
    {
        world.close(true);
        vpunt(
             "" ,
            b"Unable to write page %d into world file %s\0" as *const u8 as *const libc::c_char
                as&str,
            (*world).currentPageNumber,
            (*world).pathname,
        );
    }
    let ref mut fresh56 = (*world).currentPageNumber;
    *fresh56 += 1;
    PrepareToWriteIvoryWorldFilePage(world, (*world).currentPageNumber);
}


fn  WriteIvoryWorldFileNextQ(mut world: *mut World, mut q: LispQ) {
    let mut pointerOffset: usize = 0;
    let mut tagOffset: usize = 0;
    let mut datum: isize = 0;
    if (*world).currentQNumber >= 256 as usize {
        WriteIvoryWorldFilePage(world);
    }
    pointerOffset = 5 as usize * ((*world).currentQNumber >> 2)
        + ((*world).currentQNumber & 3)
        + 1;
    tagOffset = 4 as usize * 5 as usize * ((*world).currentQNumber >> 2)
        + ((*world).currentQNumber & 3);
    *((*world).ivory_data_page).offset(tagOffset as isize) = LispObjTag(q) as Byte;
    *((*world).ivory_data_page as *mut isize).offset(pointerOffset as isize) =
        LispObjData(q) as isize;
    let ref mut fresh57 = (*world).currentQNumber;
    *fresh57 += 1;
}


fn  ReadSwappedVLMWorldFilePage(
    mut world: *mut World,
    mut pageNumber: u32,
) {
    let mut dataOffset: off_t = 0;
    let mut tagsOffset: off_t = 0;
    if ((*world).vlm_data_page).is_null() {
        let ref mut fresh58 = (*world).vlm_data_page;
        *fresh58 = malloc((4 as usize * 8192) as libc::c_ulong) as *mut Byte;
        if ((*world).vlm_data_page).is_null() {
            world.close(true);
            vpunt(
                 "" ,
                b"Unable to allocate space for data buffer for world file %s\0" as *const u8
                    as *const libc::c_char as&str,
                (*world).pathname,
            );
        }
        let ref mut fresh59 = (*world).vlm_tags_page;
        *fresh59 = malloc(8192 as usize as libc::c_ulong) as *mut Byte;
        if ((*world).vlm_tags_page).is_null() {
            world.close(true);
            vpunt(
                 "" ,
                b"Unable to allocate space for data buffer for world file %s\0" as *const u8
                    as *const libc::c_char as&str,
                (*world).pathname,
            );
        }
        (*world).currentPageNumber = -(1);
    }
    if (*world).currentPageNumber == pageNumber {
        return;
    }
    dataOffset =
        (8192 as usize * ((*world).vlm_data_page_base + pageNumber * 4)) as off_t;
    tagsOffset =
        (8192 as usize * ((*world).vlm_tags_page_base + pageNumber * 1)) as off_t;
    if dataOffset != lseek((*world).fd, dataOffset, 0) {
        world.close(true);
        vpunt(
             "" ,
            b"Unable to seek to offset %d in world file %s\0" as *const u8 as *const libc::c_char
                as&str,
            dataOffset,
            (*world).pathname,
        );
    }
    if (4 as usize * 8192) as libc::c_long
        != read(
            (*world).fd,
            (*world).vlm_data_page as *mut libc::c_void,
            (4 as usize * 8192) as size_t,
        )
    {
        world.close(true);
        vpunt(
             "" ,
            b"Unable to read page %d from world file %s\0" as *const u8 as *const libc::c_char
                as&str,
            pageNumber,
            (*world).pathname,
        );
    }
    if tagsOffset != lseek((*world).fd, tagsOffset, 0) {
        world.close(true);
        vpunt(
             "" ,
            b"Unable to seek to offset %d in world file %s\0" as *const u8 as *const libc::c_char
                as&str,
            tagsOffset,
            (*world).pathname,
        );
    }
    if 8192 as usize as libc::c_long
        != read(
            (*world).fd,
            (*world).vlm_tags_page as *mut libc::c_void,
            8192 as usize as size_t,
        )
    {
        world.close(true);
        vpunt(
             "" ,
            b"Unable to read page %d from world file %s\0" as *const u8 as *const libc::c_char
                as&str,
            pageNumber,
            (*world).pathname,
        );
    }
    (*world).currentPageNumber = pageNumber;
}


fn  ReadSwappedVLMWorldFileQ(
    mut world: *mut World,
    mut qNumber: u32,
    mut q: *mut LispQ,
) {
    let mut datum: isize = 0;
    if qNumber < 0 as usize || qNumber >= 8192 as usize {
        world.close(true);
        vpunt(
             "" ,
            b"Invalid word number %d for world file %s\0" as *const u8 as *const libc::c_char
                as&str,
            qNumber,
            (*world).pathname,
        );
    }
    datum =
        __bswap_32(*((*world).vlm_data_page as *mut isize).offset(qNumber as isize) as u32)
            as isize;
    q = MakeLispObj(
        *((*world).vlm_tags_page).offset(qNumber as isize) as ui32,
        datum as ui32,
    );
}


fn  ReadSwappedVLMWorldFileNextQ(mut world: *mut World, mut q: *mut LispQ) {
    while (*world).currentQNumber >= 8192 as usize {
        ReadSwappedVLMWorldFilePage(world, (*world).currentPageNumber + 1);
        (*world).currentQNumber -= 8192;
    }
    ReadSwappedVLMWorldFileQ(world, (*world).currentQNumber, q);
    let ref mut fresh60 = (*world).currentQNumber;
    *fresh60 += 1;
}

pub  fn  ByteSwapWorld(
    mut worldPathname:&str,
    mut searchPath:&str,
) {
    let mut world: World = World {
        pathname:  &"" ,
        fd: 0,
        format: 0,
        byte_swapped: 0,
        vlm_data_page_base: 0,
        vlm_tags_page_base: 0,
        vlm_data_page: 0 as *mut Byte,
        vlm_tags_page: 0 as *mut Byte,
        ivory_data_page: 0 as *mut Byte,
        current_page_number: 0,
        current_q_number: 0,
        parent_world: 0 as *mut _World,
        sysout_generation: 0,
        sysout_timestamp1: 0,
        sysout_timestamp2: 0,
        sysout_parent_timestamp1: 0,
        sysout_parent_timestamp2: 0,
        n_wired_map_entries: 0,
        wired_map_entries: 0 as *mut LoadMapEntry,
        n_merged_wired_map_entries: 0,
        merged_wired_map_entries: 0 as *mut LoadMapEntry,
        n_unwired_map_entries: 0,
        unwired_map_entries: 0 as *mut LoadMapEntry,
        n_merged_unwired_map_entries: 0,
        merged_unwired_map_entries: 0 as *mut LoadMapEntry,
    };
    let mut aWorld: *mut World = 0 as *mut World;
    world.pathname = worldPathname;
    OpenWorldFile(&mut world, 1 as usize as bool);
    originalWorld = &mut world;
    FindParentWorlds(&mut world, searchPath);
    aWorld = &mut world;
    while !aWorld.is_null() {
        if VLMWorldFormat as usize == (*aWorld).format && (*aWorld).byteSwapped != 0 {
            ByteSwapOneWorld(aWorld);
        }
        aWorld = (*aWorld).parentWorld;
    }
    *__errno_location() = 0;
}


fn  ByteSwapOneWorld(mut world: *mut World) {
    let mut newPathname: &str =  "" ;
    let mut bakPathname: &str =  "" ;
    let mut block: [libc::c_char; 8192] = [0; 8192];
    let mut worldStat: stat = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_mtim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_ctim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        __glibc_reserved: [0; 3],
    };
    let mut dataStart: size_t = 0;
    let mut dataEnd: size_t = 0;
    let mut offset: size_t = 0;
    let mut wordBlockStart: *mut ui32 = 0 as *mut ui32;
    let mut newFD: usize = 0;
    newPathname = strndup(
        (*world).pathname,
        (256 as usize + 1) as libc::c_ulong,
    );
    newPathname = strncat(
        newPathname,
        b".swap\0" as *const u8 as *const libc::c_char,
        (256 as usize + 1) as libc::c_ulong,
    );
    printf(
        b"Swapping bytes in %s ... \0" as *const u8 as *const libc::c_char,
        (*world).pathname,
    );
    fflush(stdout);
    if fstat((*world).fd, &mut worldStat) < 0 as usize {
        world.close(true);
        vpunt(
             "" ,
            b"Unable to determine attributes of world file %s\0" as *const u8 as *const libc::c_char
                as&str,
            (*world).pathname,
        );
    }
    newFD = open(
        newPathname,
        0o1 as usize | 0o100 as usize | 0o1000,
        0o400
            | 0o200
            | 0o400 as usize >> 3
            | 0o400 as usize >> 3 as usize >> 3,
    );
    if newFD < 0 as usize {
        world.close(true);
        vpunt(
             "" ,
            b"Unable to create world file %s\0" as *const u8 as *const libc::c_char
                as&str,
            newPathname,
        );
    }
    offset = 0 as usize as size_t;
    dataStart = (8192 as usize * (*world).vlm_data_page_base) as size_t;
    dataEnd = (if (*world).vlm_data_page_base > (*world).vlm_tags_page_base {
        worldStat.st_size
    } else {
        (8192 as usize * (*world).vlm_tags_page_base) as libc::c_long
    }) as size_t;
    wordBlockStart = &mut block as *mut [libc::c_char; 8192] as *mut ui32;
    if 0 as usize as libc::c_long
        != lseek((*world).fd, 0 as usize as __off_t, 0)
    {
        world.close(true);
        vpunt(
             "" ,
            b"Unable to seek to start of world file %s\0" as *const u8 as *const libc::c_char
                as&str,
            (*world).pathname,
        );
    }
    while offset < worldStat.st_size as libc::c_ulong {
        if 8192 as usize as libc::c_long
            != read(
                (*world).fd,
                block.as_mut_ptr() as *mut libc::c_void,
                8192 as usize as size_t,
            )
        {
         world.close(true);
            vpunt(
                 "" ,
                b"Unable to read data from world file %s\0" as *const u8 as *const libc::c_char
                    as&str,
                (*world).pathname,
            );
        }
        if 0 as usize as libc::c_ulong == offset {
            *wordBlockStart = 0o24342504610 as libc::c_long as ui32;
        }
        if offset >= dataStart
            && offset.wrapping_add(8192 as usize as libc::c_ulong) <= dataEnd
        {
            let mut nWords: size_t =
                ((8192 as usize + 3) / 4) as size_t;
            let mut i: size_t = 0;
            let mut wordP: *mut ui32 = wordBlockStart;
            i = 0 as usize as size_t;
            while i < nWords {
                *wordP = __bswap_32(*wordP);
                i = i.wrapping_add(1);
                wordP = wordP.offset(1);
            }
        }
        if 8192 as usize as libc::c_long
            != write(
                newFD,
                block.as_mut_ptr() as *const libc::c_void,
                8192 as usize as size_t,
            )
        {
            world.close(true);
            vpunt(
                 "" ,
                b"Unable to write data to world file %s\0" as *const u8 as *const libc::c_char
                    as&str,
                newPathname,
            );
        }
        offset = (offset as libc::c_ulong).wrapping_add(8192 as usize as libc::c_ulong)
            as size_t as size_t;
    }
    world.close(true);
    close(newFD);
    bakPathname = strndup(
        (*world).pathname,
        (256 as usize + 1) as libc::c_ulong,
    );
    bakPathname = strncat(
        bakPathname,
        b".bak\0" as *const u8 as *const libc::c_char,
        (256 as usize + 1) as libc::c_ulong,
    );
    if rename((*world).pathname, bakPathname) < 0 as usize {
        world.close(true);
        vpunt(             interpolate::s!("Unable to rename world file {world.pathname} to {bakPathname}" )        );
    }
    if rename(newPathname, (*world).pathname) < 0 as usize {
        world.close(true);
        vpunt("Unable to rename world file %s to %s\0" ,
            newPathname,
world.pathname,
        );
    }
    printf(b"done.\n\0" as *const u8 as *const libc::c_char);
}
