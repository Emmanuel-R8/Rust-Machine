// #![feature(use_extern_macros, prox_macro_non_items)]
use log::warn;
use num::Integer;

use memmap::Mmap;
use sets::Set;
use std::borrow::BorrowMut;
use std::cmp::{min, Ordering};
use std::fs::{DirEntry, File};
use std::io::Read;
use std::mem::size_of;
use std::ops::Div;
use std::path::{Path, PathBuf};
use std::{fmt, process};
use uuid::Uuid;

use crate::common::constants::{
    LoadFileFormat, QTag, CDR, IVORY_PAGE_SIZE_BYTES, IVORY_PAGE_SIZE_QS, IVORY_WORLD_FILE_COOKIE,
    MEMORYWAD_SIZE, MEMORY_PAGE_SIZE, VLMMAXIMUM_HEADER_BLOCKS, VLMPAGE_SIZE_QS,
    VLMVERSION1_AND_ARCHITECTURE, VLMVERSION2_AND_ARCHITECTURE, VLMWORLD_FILE_COOKIE,
    VLMWORLD_FILE_V2_FIRST_MAP_Q, VLMWORLD_SUFFIX,
};
use crate::common::types::QWord;
use crate::_GC;

use crate::emulator::emulator::GlobalContext;
use crate::hardware::machine::VirtualMachine;
use crate::hardware::memory::{
    compute_protection, default_attributes, lisp_obj_data, make_lisp_obj_u, memory_page_offset,
    memory_wad_offset,
};
use crate::utils::{byte_swap_32, pack_8_to_32};

/// A single load map entry -- See SYS:NETBOOT;WORLD-SUBSTRATE.LISP for details
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct LoadMapEntry {
    pub address: u32, // VMA to be filled in by this load map entry
    // NOTE: opcount and opcode are field of a struct op{} in the C code
    pub count: u32,                   // Number of words to be filled in by this entry
    pub map_code: LoadMapEntryOpcode, // An LoadMapEntryOpcode specifying how to do so
    pub data: QWord,                  // Interpretation is based on the opcode
                                      // pub world: Rc<RefCell<World<'a>>>, // Ref to World from which this entry was obtained  // !!!!!!!!! Should not be needed to link back
}

pub fn clone(map_entries: &Set<LoadMapEntry>) -> Set<LoadMapEntry> {
    let mut res: Set<LoadMapEntry> = Set::new_ordered(&[], true);

    for m in map_entries.data {
        res.insert(m.clone());
    }

    return res;
}

// impl Default for Set<LoadMapEntry> {
//     fn default() -> Self {
//         return Set::<LoadMapEntry>::new_ordered(&[], true)
//     }
// }

impl Default for LoadMapEntry {
    fn default() -> Self {
        Self {
            address: 0,
            count: 0,
            map_code: LoadMapEntryOpcode::Constant,
            data: QWord::default(),
        }
    }
}

/// Load map operation codes
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoadMapEntryOpcode {
    DataPages, // Load data pages from the file
    #[default] //
    Constant, // Store a constant data page into memory
    ConstantIncremented, // Store an auto-incrementing constant into memory
    Copy,      // Copy an existing piece of memory
}

impl fmt::Display for LoadMapEntryOpcode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

// Ord and PartialOrd are implemented to have ordered sets
// Ordered by address then count
impl Ord for LoadMapEntry {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.address.cmp(&other.address) {
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
            Ordering::Equal => self.count.cmp(&other.count),
        }
    }
}

impl PartialOrd for LoadMapEntry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// List of possible map entries
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum MapEntrySelector {
    #[default] //
    Wired,
    MergedWired,
    Unwired,
    MergedUnwired,
}

// Description of an open world file
#[derive()]
pub struct World {
    pub id: Uuid,
    pub pathname: PathBuf,      // -> Pathname of the world file
    pub fd: Option<File>,       // Unix filedes # if the world file is open
    pub format: LoadFileFormat, // A LoadFileFormat indicating the type of file
    pub byte_swapped: bool,     // World is byte swapped on this machine (VLM only)

    pub page_base: u32,
    pub page: Vec<QWord>,            // -> The current VLM format page
    pub data_page_base: u32,         // Block number of first page of data (VLM only)
    pub data_page: Vec<u32>,         // -> The data of the current VLM format page
    pub tags_page_base: u32,         // Block number of first page of tags (VLM only)
    pub tags_page: Vec<QTag>,        // -> The tags of the current VLM format page
    pub ivory_data_page: Vec<QWord>, // [QWord; IVORY_PAGE_SIZE_BYTES] -> The data of the current Ivory format page. TODO: rename to current_data-page
    // Size is 0x500 = 0x100 for tags + 0x400 for data
    pub current_page_number: u32, // Page number of the page in the buffer, if any. -1 means not pointing yet
    pub current_Q_number: u32,    // Q number within the page to be read

    pub timestamp_1: u32, // Unique ID of this world, part 1 ...
    pub timestamp_2: u32, // ... part 2

    pub generation: u32, // Generation number of this world (> 0 if IDS)
    // pub parent_world: Option<Rc<RefCell<&'a World<'a>>>>, // -> Parent of this world if it's an IDS
    pub parent_world: Uuid,      // -> Parent of this world if it's an IDS
    pub parent_timestamp_1: u32, // Unique ID of this world's parent, part 1 ...
    pub parent_timestamp_2: u32, // ... part 2

    pub wired_map_entries: Set<LoadMapEntry>, // -> The wired load map entries
    pub merged_wired_map_entries: Set<LoadMapEntry>, // ..
    pub unwired_map_entries: Set<LoadMapEntry>, // -> The unwired load map entries (Ivory only)
    pub merged_unwired_map_entries: Set<LoadMapEntry>, // ..
}

impl World {
    pub fn new() -> World {
        let w = Self {
            id: Uuid::new_v4(),
            pathname: PathBuf::default(),
            fd: None,
            format: LoadFileFormat::IvoryWorldFormat,
            byte_swapped: false,

            page_base: 0,
            page: vec![],
            data_page_base: 0,
            data_page: vec![],
            tags_page_base: 0,
            tags_page: vec![],
            ivory_data_page: vec![QWord::default(); IVORY_PAGE_SIZE_BYTES as usize],

            current_page_number: 0,
            current_Q_number: 0,

            timestamp_1: 0,
            timestamp_2: 0,

            generation: 0,
            parent_world: Uuid::nil(),
            parent_timestamp_1: 0,
            parent_timestamp_2: 0,

            wired_map_entries: Set::<LoadMapEntry>::new_ordered(&[], true),
            merged_wired_map_entries: Set::<LoadMapEntry>::new_ordered(&[], true),
            unwired_map_entries: Set::<LoadMapEntry>::new_ordered(&[], true),
            merged_unwired_map_entries: Set::<LoadMapEntry>::new_ordered(&[], true),
        };
        return w;
    }

    // Select the specified Set<LoadMapEntry>
    fn select_entries(&mut self, selector: MapEntrySelector) -> &Set<LoadMapEntry> {
        return match selector {
            MapEntrySelector::Wired => &(self.wired_map_entries),
            MapEntrySelector::MergedWired => &(self.merged_wired_map_entries),
            MapEntrySelector::Unwired => &(self.unwired_map_entries),
            MapEntrySelector::MergedUnwired => &(self.merged_unwired_map_entries),
        };
    }

    // Close a world file
    pub fn close(&mut self, close_parent: bool) {
        self.fd = None; // Drop the file descriptor and close it automatically.
        self.data_page = vec![];
        self.tags_page = vec![];
        self.ivory_data_page = vec![];
        self.merged_wired_map_entries = Set::<LoadMapEntry>::new_ordered(&[], true);
        self.wired_map_entries = Set::<LoadMapEntry>::new_ordered(&[], true);
        self.merged_unwired_map_entries = Set::<LoadMapEntry>::new_ordered(&[], true);
        self.unwired_map_entries = Set::<LoadMapEntry>::new_ordered(&[], true);
    }
}

impl Default for World {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            pathname: PathBuf::default(),
            fd: None,
            format: LoadFileFormat::IvoryWorldFormat,
            byte_swapped: false,
            data_page_base: 0,
            tags_page_base: 0,
            page_base: 0,
            data_page: vec![],
            tags_page: vec![],
            page: vec![],
            ivory_data_page: vec![QWord::default(); IVORY_PAGE_SIZE_BYTES as usize],
            current_page_number: 0,
            current_Q_number: 0,
            parent_world: Uuid::nil(),
            generation: 0,
            timestamp_1: 0,
            timestamp_2: 0,
            parent_timestamp_1: 0,
            parent_timestamp_2: 0,
            wired_map_entries: Set::<LoadMapEntry>::new_ordered(&[], true),
            merged_wired_map_entries: Set::<LoadMapEntry>::new_ordered(&[], true),
            unwired_map_entries: Set::<LoadMapEntry>::new_ordered(&[], true),
            merged_unwired_map_entries: Set::<LoadMapEntry>::new_ordered(&[], true),
        }
    }
}

impl Clone for World {
    fn clone(&self) -> Self {
        let w = World {
            id: self.id.clone(),
            pathname: self.pathname.clone(),
            fd: None,
            format: self.format.clone(),
            byte_swapped: self.byte_swapped.clone(),
            page_base: self.page_base.clone(),
            page: self.page.clone(),
            data_page_base: self.data_page_base.clone(),
            data_page: self.data_page.clone(),
            tags_page_base: self.tags_page_base.clone(),
            tags_page: self.tags_page.clone(),
            ivory_data_page: self.ivory_data_page.clone(),
            current_page_number: self.current_page_number.clone(),
            current_Q_number: self.current_Q_number.clone(),
            timestamp_1: self.timestamp_1.clone(),
            timestamp_2: self.timestamp_2.clone(),
            generation: self.generation.clone(),
            parent_world: self.parent_world.clone(),
            parent_timestamp_1: self.parent_timestamp_1.clone(),
            parent_timestamp_2: self.parent_timestamp_2.clone(),
            wired_map_entries: self.wired_map_entries.clone(),
            merged_wired_map_entries: self.merged_wired_map_entries.clone(),
            unwired_map_entries: self.unwired_map_entries.clone(),
            merged_unwired_map_entries: self.merged_unwired_map_entries.clone(),
        };
        return w;
    }
}

// CHECK UNUSED
// impl Default for &World {
//     fn default() -> Self {
//         return &World::default();
//     }
// }

// CHECK UNUSED
// impl<'a> Clone for &mut World<'a> {
//     fn clone(self) -> Self {
//         let mut gc = *self.clone();
//         return gc.borrow_mut();
//     }
// }

// Block numbers of the first page of data and tags for a VLM world as stored in its header
#[derive(Default, Debug)]
pub struct VLMPageBases {
    // #if BYTE_ORDER == LITTLE_ENDIAN
    pub data_page_base: u32,
    pub tags_page_base: u32, // Limits header and load maps to 112K bytes
                             // #else
                             //   isize tagsPageBase : 4; // Limits header and load maps to 112K bytes
                             //   isize dataPageBase : 28;
                             // #endif
}

pub const VLMPAGE_BASES: VLMPageBases = VLMPageBases {
    data_page_base: 28,
    tags_page_base: 4,
};

// // Data structures passed by Lisp via the SaveWorld coprocessor register
#[derive(Default, Debug)]
pub struct SaveWorldEntry {
    pub address: u32, // VMA of data (usually a region) to be saved
    pub extent: u32,  // Number of words starting at this address to save
}

#[derive(Default, Debug)]
pub struct SaveWorldData {
    pub patname: u32,                    // Pathname of the world file (a DTP-STRING)
    pub entry_count: u32,                // Number of address/extent pairs to follow
    pub entries: Option<SaveWorldEntry>, //
}

// Print an error message and terminate the VLM
pub fn vpunt(msg: String) {
    warn!("Emulator final log");
    warn!("{}", msg);
    warn!("----------------------------------------------------------");
    warn!("TERMINATING EMULATOR\n");

    process::exit(1);
}

pub fn punt_world(ctx: &mut GlobalContext, msg: String) {
    ctx.close(true);
    vpunt(msg);
}

pub fn read_ivory_world_file_page(w: &mut World, page_number: u32) {
    todo!()
}

// Merges a foreground load map and a background load map together into a single load map
// background are the entries in the parent world
// foreground are the child world shadowing the parent (background)
pub fn merge_a_map<'a>(
    world: &World,
    fore_map: &Set<LoadMapEntry>,
    back_map: &Set<LoadMapEntry>,
) -> Set<LoadMapEntry> {
    // Load the relevant maps
    // We need mutable copies to adjust ends and starts if need at each iteration.
    let mut new_back = clone(&back_map);
    let mut new_fore = clone(fore_map);

    let n_back = new_back.data.len() as u32;
    let n_fore = new_fore.data.len() as u32;

    // Resulting new map
    let mut new_map_entries: Set<LoadMapEntry> = Set::<LoadMapEntry>::new_ordered(&[], true);

    // See SYS:IFEP;WORLD-SUBSTRATE.LISP for an explanation of the maximum number of entries
    let max = n_back + n_fore + n_fore;
    if max == 0 {
        return new_map_entries;
    }

    let page_size_Qs = match world.format {
        LoadFileFormat::VLMWorldFormat => VLMPAGE_SIZE_QS,
        _ => IVORY_PAGE_SIZE_QS,
    };

    let mut fore_idx: u32 = 0;
    let mut back_idx: u32 = 0;
    loop {
        if new_back.data[back_idx as usize].map_code != LoadMapEntryOpcode::DataPages {
            back_idx += 1;
            continue;
        }

        let fore_start = new_fore.data[fore_idx as usize].address;
        let fore_final = fore_start + new_fore.data[fore_idx as usize].count - 1;

        let back_start = new_back.data[fore_idx as usize].address;
        let back_final = back_start + new_back.data[fore_idx as usize].count - 1;

        // Possible situations:
        //  1: |--- BACK ---|
        //     No more entries in FRONT
        //  Push all the BACK on the new map, final exit
        //
        //  2: |--- FRONT ---|
        //     No more entries in BACK
        //  Push all the FRONT on the new map, final exit
        //
        //  3: |--- BACK ---|
        //                         |--- FRONT ---|
        //  Easy: Push BACK on the new map, increase back_idx, iterate
        //
        //  4: |--- BACK --------|
        //                   |----- FRONT ---|
        //  Push BACK until beginning of front; adjust beginning of BACK, iterate. Note no index increases.
        //
        //  5:         |--- BACK ----|
        //        |----- FRONT -----------|
        //  This BACK is usealess. increase back_idx to drop it, iterate. No increase of fore_idx, we could next have another 5, 6 or 7.
        //
        //  6:           |--- BACK --------|
        //          |----- FRONT ---|
        //  Push FRONT and increase fore_idx, adjust the start of BACK, iterate.
        //
        //  7:                    |--- BACK --------|
        //      |----- FRONT ---|
        //  Push FRONT and increase fore_idx. Iterate.
        //

        // Situation 1:
        if fore_idx >= n_fore {
            while back_idx < n_back {
                new_map_entries.insert(new_back.data[back_idx as usize]);
                back_idx += 1;
            }
            break;
        }

        // Situation 2:
        if back_idx >= n_back {
            while fore_idx < n_fore {
                new_map_entries.insert(new_fore.data[fore_idx as usize]);
                fore_idx += 1;
            }
            break;
        }

        // Situation 3:
        if back_final < fore_start {
            new_map_entries.insert(new_back.data[back_idx as usize]);
            back_idx += 1;
            continue;
        }

        // Situation 4:
        if back_start < fore_start && back_final >= fore_start {
            new_map_entries.insert(LoadMapEntry {
                address: new_back.data[back_idx as usize].address,
                count: new_fore.data[fore_idx as usize].address
                    - (new_back.data[back_idx as usize].address
                        + new_back.data[back_idx as usize].count
                        - 1),
                map_code: new_back.data[back_idx as usize].map_code,
                data: new_back.data[back_idx as usize].data,
            });
            new_back.data[back_idx as usize].address = new_fore.data[fore_idx as usize].address;
            new_back.data[back_idx as usize].count =
                back_final - new_back.data[back_idx as usize].address + 1;
            continue;
        }

        // Situation 5:
        if back_start >= fore_start && back_final <= fore_final {
            back_idx += 1;
            continue;
        }

        // Situation 6:
        if back_start >= fore_start && back_final > fore_final {
            new_map_entries.insert(new_fore.data[fore_idx as usize]);
            fore_idx += 1;
            new_back.data[back_idx as usize].address = fore_final + 1;
            new_back.data[back_idx as usize].count =
                back_final - new_back.data[back_idx as usize].address + 1;
            continue;
        }

        // Situation 7:
        if back_start > fore_final {
            new_map_entries.insert(new_fore.data[fore_idx as usize]);
            fore_idx += 1;
            continue;
        }
    }

    return new_map_entries;
}

// fn read_ivory_world_file_page(mut world: *mut World, mut page_number: u32) {
//     let mut offset: u32 = 0;
//     if world.current_page_number == page_number {
//         return;
//     }

//     offset = (page_number * IVORY_PAGE_SIZE_BYTES) ;
//     if offset != lseek(world.fd, offset, 0) {
//         world.close(true);
//         vpunt(format!("Unable to seek to offset %d in world file {}", world.pathname.display()));
//     }
//     if read(
//             world.fd?,
//             world.ivory_data_page,
//             IVORY_PAGE_SIZE_BYTES
//         ) != IVORY_PAGE_SIZE_BYTES
//     {
//         world.close(true);
//         vpunt(format!("Unable to read page {} from world file {}", page_number, world.pathname.display());
//     }
//     world.current_page_number = page_number;
// }

pub fn read_load_map(w: &mut World, map_selector: MapEntrySelector) {
    let map: &mut Vec<LoadMapEntry> = match map_selector {
        MapEntrySelector::Wired => w.wired_map_entries.data.borrow_mut(),
        MapEntrySelector::MergedWired => w.merged_wired_map_entries.data.borrow_mut(),
        MapEntrySelector::Unwired => w.unwired_map_entries.data.borrow_mut(),
        MapEntrySelector::MergedUnwired => w.merged_unwired_map_entries.data.borrow_mut(),
    };

    for e in map {
        let q = read_ivory_world_file_next_Q(w);
        e.address = q.a();

        let q = read_ivory_world_file_next_Q(w);
        let op = q.u();
        e.count = op & 0x00FF_FFFF;
        e.map_code = match (op & 0xFF00_0000) >> 24 {
            0 => LoadMapEntryOpcode::DataPages,
            1 => LoadMapEntryOpcode::Constant,
            2 => LoadMapEntryOpcode::ConstantIncremented,
            3 => LoadMapEntryOpcode::Copy,
            _ => LoadMapEntryOpcode::default(),
        };

        e.data = read_ivory_world_file_next_Q(w);
    }
}

// fn read_load_map(mut world: *mut World, mut nSet<LoadMapEntry>: u32, mut Set<LoadMapEntry>: *mut LoadMapEntry) {
//     let mut q: QWord = LispObj {
//         parts: _LispObj {
//             tag: 0,
//             data: QData { u: 0 },
//         },
//     };
//     let mut i: u32 = 0;
//     i = 0;
//     while i < nSet<LoadMapEntry> {
//         (*Set<LoadMapEntry>).address = ReadIvoryWorldFileNextQ(world, &mut q);
//         *(&mut (*Set<LoadMapEntry>).op as *mut isize) = ReadIvoryWorldFileNextQ(world, &mut q);
//         (*Set<LoadMapEntry>).data = ReadIvoryWorldFileNextQ(world, &mut q);
//         let ref mut fresh36 = (*Set<LoadMapEntry>).world;
//         *fresh36 = world as PtrV;
//         i += 1;
//         Set<LoadMapEntry> = Set<LoadMapEntry>.offset(1);
//     }
// }

fn open_world_file(puntOnErrors: bool) -> bool {
    let mut page_bases: QWord = QWord::default();
    let mut wired_count_Q: u32 = 0;
    let mut unwired_count_Q: u32 = 0;
    let mut pages_base_Q: u32 = 0;
    let mut first_sysout_Q: u32 = 0;
    let mut first_map_Q: u32 = 0;

    let mut w = unsafe { _GC.worlds.get(&_GC.world).unwrap() };

    w.data_page = vec![];
    w.tags_page = vec![];
    w.ivory_data_page = vec![];
    w.wired_map_entries = Set::<LoadMapEntry>::new_ordered(&[], true);
    w.unwired_map_entries = Set::<LoadMapEntry>::new_ordered(&[], true);
    w.merged_wired_map_entries = Set::<LoadMapEntry>::new_ordered(&[], true);
    w.merged_unwired_map_entries = Set::<LoadMapEntry>::new_ordered(&[], true);
    w.parent_world = Uuid::nil();

    let path = &w.pathname;
    let mut f = File::open(path).expect("Could not open file");
    w.fd = Some(f);

    let mut cookie = [0 as u8; size_of::<u32>()];
    if f.read_exact(&mut cookie).is_err() && puntOnErrors {
        w.close(true);
        vpunt(format!("Reading world file {} cookie.", path.display()));
    } else {
        return false;
    }

    match pack_8_to_32(cookie) {
        VLMWORLD_FILE_COOKIE => {
            w.format = LoadFileFormat::VLMWorldFormat;
            w.byte_swapped = false;
        }

        VLMWORLD_FILE_COOKIE_SWAPPED => {
            w.format = LoadFileFormat::VLMWorldFormat;
            w.byte_swapped = true;
        }

        IVORY_WORLD_FILE_COOKIE => {
            w.format = LoadFileFormat::IvoryWorldFormat;
            wired_count_Q = 1;
            unwired_count_Q = 2;
            first_sysout_Q = 0;
            first_map_Q = 8;
        }

        _ => {
            if puntOnErrors {
                w.close(true);
                vpunt(format!(
                    "Format of world file {} is unrecognized",
                    path.display()
                ));
            }
        }
    }

    w.ivory_data_page = vec![QWord::default(); (IVORY_PAGE_SIZE_BYTES / 4) as usize];

    w.current_page_number = 0;

    // The header and load maps for both VLM and Ivory world files are stored using Ivory file format settings (i.e., 256 Qs per 1280 byte page)
    if w.format == LoadFileFormat::VLMWorldFormat {
        match unsafe { lisp_obj_data(read_ivory_world_file_Q(&w, 0)).u } {
            VLMVERSION1_AND_ARCHITECTURE => {
                wired_count_Q = 1;
                unwired_count_Q = 0;
                pages_base_Q = 3;
                first_sysout_Q = 0;
                first_map_Q = 8;
            }
            VLMVERSION2_AND_ARCHITECTURE => {
                wired_count_Q = 1;
                unwired_count_Q = 0;
                pages_base_Q = 2;
                first_sysout_Q = 3;
                first_map_Q = 8;
            }
            _ => {
                vpunt(format!(
                    "Format magic code of world file {} is unrecognized",
                    path.display()
                ));
            }
        }
    }

    if w.format == LoadFileFormat::VLMWorldFormat {
        page_bases = read_ivory_world_file_Q(&w, pages_base_Q);
        w.data_page_base = unsafe { page_bases.parts.data.u };
        w.tags_page_base = unsafe { page_bases.parts.tag as u32 };
    }

    if first_sysout_Q != 0 {
        w.current_Q_number = first_sysout_Q;

        w.generation = unsafe { lisp_obj_data(read_ivory_world_file_next_Q(&mut w)).u };
        w.timestamp_1 = unsafe { lisp_obj_data(read_ivory_world_file_next_Q(&mut w)).u };
        w.timestamp_2 = unsafe { lisp_obj_data(read_ivory_world_file_next_Q(&mut w)).u };
        w.parent_timestamp_1 = unsafe { lisp_obj_data(read_ivory_world_file_next_Q(&mut w)).u };
        w.parent_timestamp_2 = unsafe { lisp_obj_data(read_ivory_world_file_next_Q(&mut w)).u };
    } else {
        w.generation = 0;
        w.timestamp_2 = 0;
        w.timestamp_1 = 0;
        w.parent_timestamp_2 = 0;
        w.parent_timestamp_1 = 0;
    }
    w.current_Q_number = first_map_Q;
    read_load_map(&mut w, MapEntrySelector::Wired);
    read_load_map(&mut w, MapEntrySelector::Unwired);

    return true;
}

fn read_ivory_world_file_Q(w: &World, address: u32) -> QWord {
    if address >= IVORY_PAGE_SIZE_BYTES {
        vpunt(format!(
            "Invalid word number {} for world file {}",
            address,
            &w.pathname.display().to_string()
        ));
    }

    return w.ivory_data_page[address as usize];
}

pub fn read_ivory_world_file_next_Q(w: &mut World) -> QWord {
    // If the the current address is too high, load the next page (several time if needed)
    while w.current_Q_number >= IVORY_PAGE_SIZE_BYTES {
        read_ivory_world_file_page(w, w.current_page_number + 1);
        w.current_Q_number -= IVORY_PAGE_SIZE_BYTES;
    }

    let q = read_ivory_world_file_Q(w, w.current_Q_number);
    w.current_Q_number += 1;

    return q;
}

pub fn world_p(candidate_world: DirEntry) -> bool {
    let mut a_world = World::default();
    let mut new_worlds: Vec<World>;
    let mut candidate_pathname: &Path;

    // if candidate_world.file_name().len() > VLMWORLD_SUFFIX.len() {
    //     a_world.pathname = PathBuf::from("/")
    //         .join(&ctx.scanning_dir)
    //         .join(candidate_world.file_name());
    // } else {
    //     return false;
    // }
    unimplemented!()
}

pub fn write_lisp_obj_data_u(q: &mut QWord, data: u32) {
    q.parts.data.u = data
}

impl GlobalContext {
    //  Canonicalize the load map entries for a VLM world:  Look for load map entries
    //  that don't start on a page boundary and convert them into a series of
    //  LoadMapConstant entries to load the data.  Thus, all data in the world file
    //  will be page-aligned to allow for direct mapping of the world load file into
    //  memory.  (Eventually, we may also merge adjacent load map rentries.)
    fn canonicalize_VLM_load_map_entries(&mut self) -> Set<LoadMapEntry> {
        let mut new_wired_map_entries: Set<LoadMapEntry> =
            Set::<LoadMapEntry>::new_ordered(&[], true);

        let mut w = unsafe { self.worlds.get(&self.world).unwrap() };
        let n_wired_entries = w.wired_map_entries.data.len() as u32;

        let mut page_number: u32 = 0;

        let mut i: u32 = 0;
        while i < n_wired_entries {
            let current_map_entry = &w.wired_map_entries.data[i as usize];

            let (page_count, r) = current_map_entry.address.div_rem(&VLMPAGE_SIZE_QS);

            if r == 0 {
                // If the address of the page is a multiple of VLMPAGE_SIZE_QS, i.e. Page Aligned,
                // assign the page number within the file
                let mut new_wired_map_entry = w.wired_map_entries.data[i as usize];
                new_wired_map_entry.data = make_lisp_obj_u(CDR::Jump, QTag::Fixnum, page_number); // Tag 8
                new_wired_map_entries.insert(new_wired_map_entry);
                page_number = page_number + page_count;
                i += 1;
            } else {
                // Not Page Aligned:  Convert into a series of LoadMapConstant entries
                for j in 0..current_map_entry.count {
                    let mut new_wired_map_entry = w.wired_map_entries.data[(i + j) as usize];

                    new_wired_map_entry.address = w.wired_map_entries.data[i as usize].address + j;
                    new_wired_map_entry.map_code = LoadMapEntryOpcode::Constant;
                    new_wired_map_entry.count = 1;
                    new_wired_map_entry.data = virtual_memory_read(new_wired_map_entry.address);
                    new_wired_map_entries.insert(new_wired_map_entry);
                }

                i += current_map_entry.count;
            }
        }

        // Compute size of header in VLM blocks to determine where the tags and data pages will start within the world file
        let n_Qs = new_wired_map_entries.data.len() as u32 * 3 + VLMWORLD_FILE_V2_FIRST_MAP_Q;
        let page_count = (n_Qs - 1).div(IVORY_PAGE_SIZE_QS);
        let block_count = (page_count * IVORY_PAGE_SIZE_BYTES - 1).div(VLMPAGE_SIZE_QS);

        if block_count > VLMMAXIMUM_HEADER_BLOCKS {
            w.close(true);
            vpunt(format!(
                "Unable to store data map in space reserved for same in world file {}",
                w.pathname.display().to_string()
            ));
        }
        w.tags_page_base = block_count;
        w.data_page_base = (w.tags_page_base + 1) * page_number;

        return new_wired_map_entries;
    }
}

pub fn write_ivory_world_file_next_Q(w: &mut World, q: QWord) {}
// fn write_ivory_world_file_next_Q(mut world: *mut World, mut q: QWord) {
//     let mut pointerOffset: u32 = 0;
//     let mut tagOffset: u32 = 0;
//     let mut datum: isize = 0;
//     if world.current_Q_number >= 256  {
//         write_ivory_world_file_page(world);
//     }
//     pointerOffset = 5  * (world.current_Q_number >> 2)
//         + (world.current_Q_number & 3)
//         + 1;
//     tagOffset = 4  * 5  * (world.current_Q_number >> 2)
//         + (world.current_Q_number & 3);
//     *(world.ivory_data_page).offset(tagOffset) = LispObjTag(q) as Byte;
//     *(world.ivory_data_page).offset(pointerOffset ) =
//         lisp_obj_data(q);
//     let ref mut fresh57 = world.current_Q_number;
//     *fresh57 += 1;
// }

pub fn virtual_memory_read(addr: u32) -> QWord {
    todo!();
}
pub fn copy_ivory_world_file_next_Q(world: &mut World, from: u32) {
    let q = virtual_memory_read(from);
    write_ivory_world_file_next_Q(world, q);
}

fn write_vlmworld_file_header(world: &mut World) {
    todo!()
    // let mut generation_Q: QWord = make_lisp_obj_u(QTag::Null, 0);

    // let mut q = QWord::new();
    // let mut page_bases: isize = 0;
    // let mut n_blocks: u32 = 0;
    // let mut i: usize = 0;

    // i = world.n_wired_map_entries;
    // let mut mapEntry  =LoadMapEntry::new();
    // while i > 0  {
    //     mapEntry  = world.wired_map_entries[i - 1 as usize];
    //     if mapEntry.opcode == LOAD_MAP_DATA_PAGES   {
    //         n_blocks = world.vlm_data_page_base + (lisp_obj_data(mapEntry.data) +(mapEntry.opcount  / VLMPAGE_SIZE_QS) + 1 ) * 4;
    //         if ftruncate(world.fd, n_blocks * VLMPAGE_SIZE_QS  ) < 0 {
    //             world.close(true);
    //             vpunt(format!("Unable to grow world file {} to {} bytes", world.pathname.display(), n_blocks * VLMPAGE_SIZE_QS));
    //         }
    //         break;
    //     } else {
    //         i -= 1;
    //     }
    // }

    // prepare_to_write_ivory_world_file_page(world, 0);
    // page_bases.set_dataPageBase(world.vlm_data_page_base );
    // page_bases.set_tagsPageBase(world.vlm_tags_page_base );

    // write_ivory_world_file_next_Q(world, make_lisp_obj_u((2 << 6) + 8, VLMVERSION2_AND_ARCHITECTURE));
    // write_ivory_world_file_next_Q(world, make_lisp_obj_u((2 << 6) + 9, world.n_wired_map_entries ));
    // write_ivory_world_file_next_Q(world, make_lisp_obj_u((2 << 6) + 10, page_bases ));

    // generation_Q = read_system_comm_slot(world.sysout_generation);
    // write_ivory_world_file_next_Q(world, make_lisp_obj_u(((2) << 6) + 35, lisp_obj_data(generation_Q)));

    // copy_ivory_world_file_next_Q(w, ADDRESS_NIL / SIZE_EMBWORD + world.sysout_timestamp1);
    // copy_ivory_world_file_next_Q(w, ADDRESS_NIL / SIZE_EMBWORD + world.sysout_timestamp2);
    // copy_ivory_world_file_next_Q(w, ADDRESS_NIL / SIZE_EMBWORD + world.sysout_parent_timestamp1);
    // copy_ivory_world_file_next_Q(w, ADDRESS_NIL / SIZE_EMBWORD + world.sysout_parent_timestamp2);

    // for i in 0..world.n_wired_map_entries {
    //     write_ivory_world_file_next_Q(world, make_lisp_obj_u(25, mapEntry.address));
    //     write_ivory_world_file_next_Q(world,  make_lisp_obj_u(8, mapEntry.opcode));
    //     write_ivory_world_file_next_Q(world,  world.wired_map_entries[i].data);
    // }

    // write_ivory_world_file_page(world);
}

pub fn map_virtual_address_data(addr: u32) -> u32 {
    todo!()
}
pub fn map_virtual_address_tag(addr: u32) -> u32 {
    todo!()
}

fn write_VLM_world_file_pages(vm: &mut VirtualMachine) {
    let mut w = unsafe { _GC.worlds.get(&_GC.world).unwrap() };

    let mut page_number: u32 = 0;
    let mut word_count: u32 = 0;
    let mut byte_count: u32 = 0;
    let mut offset: u64 = 0;
    let mut increment: u32 = 0;
    let mut i: usize = 0;

    // QWord = 1 byte tag / 4 bytes data
    // pages are stored as 1 block with all the tags / 3 blocks with all the data

    // for  m in world.wired_map_entries {
    //     if !(m.opcode != LOAD_MAP_DATA_PAGES) {
    //         page_number = lisp_obj_data(m.data).u ;
    //         offset = (VLMPAGE_SIZE_QS * (world.vlm_data_page_base + page_number * 4)) as u64;
    //         if world.fd.expect("")
    //             .seek(SeekFrom::Start(offset))
    //             .is_ok_and(|&x| x != offset) {
    //             vpunt(format!("Unable to seek to offset {} in world file {}", offset, world.pathname.display()));
    //         }

    //         world.fd.expect("").write(vm.mem.map_virtual_address_data(m.address, m.opcount));

    //         offset = (VLMPAGE_SIZE_QS * world.vlm_tags_page_base + page_number) as u64;
    //         if world.fd.expect("")
    //             .seek(SeekFrom::Start(offset))
    //             .is_ok_and(|&x| x != offset) {
    //             vpunt(format!("Unable to seek to offset {} in world file {}", offset, world.pathname.display()));
    //         }

    //         byte_count = (word_count * size_of::<QTag>() ) as usize;
    //         world.fd
    //             .expect(format!("Unable to write tags page {} into world file {}", page_number,world.pathname.display())
    //             .write(map_virtual_address_data(m.address)[0..byte_count]);

    //     }
    // }
}

fn prepare_to_write_ivory_world_file_page(w: &mut World, page_number: u32) {
    w.current_page_number = page_number;
    w.current_Q_number = 0;
    w.ivory_data_page = vec![QWord::default(); IVORY_PAGE_SIZE_BYTES as usize];
}

fn write_ivory_world_file_page(world: &mut World) {
    if 0 == world.current_Q_number {
        return;
    }

    todo!()
    // let mut offset: u32 = world.current_page_number * IVORY_PAGE_SIZE_BYTES ;
    // if offset != lseek(world.fd, offset, 0) {
    //     world.close(true);
    //     vpunt(format!("Unable to seek to offset {} in world file {}",
    //         offset,
    //         world.pathname.display())
    //     );
    // }
    // if IVORY_PAGE_SIZE_BYTES
    //     != write(
    //         world.fd,
    //         world.ivory_data_page ,
    //         IVORY_PAGE_SIZE_BYTES,
    //     )
    // {
    //     world.close(true);
    //     vpunt(format!("Unable to write page {} into world file {}" ,
    //         world.current_page_number,
    //         world.pathname.display())
    //     );
    // }
    // let ref mut fresh56 = world.current_page_number;
    // *fresh56 += 1;
    // prepare_to_write_ivory_world_file_page(world, world.current_page_number);
}

impl GlobalContext {
    fn read_swapped_VLM_world_file_page(&self, mut page_number: u32) {
        unimplemented!()

        // // If the page current loaded in the world is the page we are looking for, then nothing to do
        // if world.current_page_number == page_number {
        //     return;
        // }
        // world.current_page_number = 0;

        // // If no space allocated...
        // if world.vlm_data_page.len() == 0 {
        //     world.vlm_data_page  = vec![0; VLMPAGE_SIZE_QS];
        //     world.vlm_tags_page = vec![0; VLMPAGE_SIZE_QS / 4];
        // }

        // let mut dataOffset = (VLMPAGE_SIZE_QS  * (world.vlm_data_page_base + page_number * 4)) as u64;
        // if !world.fd.expect("")
        //         .seek(SeekFrom::Start(dataOffset as u64))
        //         .is_ok_and(|&x| x == dataOffset) {
        //     vpunt(format!("Unable to seek to offset {} for data pages in world file {}", dataOffset, world.pathname.display()));
        // }

        // if (4  * VLMPAGE_SIZE_QS) != read(world.fd, world.vlm_data_page, 4  * VLMPAGE_SIZE_QS) {
        //     world.close(true);
        //     vpunt(format!("Unable to read page {} from world file {}", page_number, world.pathname.display()));
        // }

        // let mut tagOffset= (VLMPAGE_SIZE_QS  * (world.vlm_tags_page_base + page_number * 1)) as u64;
        // if !world.fd.expect("")
        //         .seek(SeekFrom::Start(tagOffset as u64))
        //         .is_ok_and(|&x| x == tagOffset) {
        //     vpunt(format!("Unable to seek to offset {} for tag page in world file {}", dataOffset, world.pathname.display()));
        // }

        // if VLMPAGE_SIZE_QS != read(world.fd, world.vlm_tags_page, VLMPAGE_SIZE_QS) {
        //     world.close(true);
        //     vpunt(format!("Unable to read page {} from world file {}", page_number, world.pathname.display()));
        // }

        // world.current_page_number = page_number;
    }

    fn read_swapped_VLM_world_file_Q(&self, mut q_number: u32) -> QWord {
        let mut w = unsafe { self.worlds.get(&self.world).unwrap() };
        let mut datum: u32 = 0;

        if q_number < 0 || q_number >= VLMPAGE_SIZE_QS {
            self.close(true);
            vpunt(format!(
                "Invalid word number {} for world file {}",
                q_number,
                w.pathname.display().to_string()
            ));
        }

        datum = byte_swap_32(w.data_page[q_number as usize]);
        let tag: QTag = unsafe { ::std::mem::transmute(w.tags_page[q_number as usize]) };
        return make_lisp_obj_u(CDR::Jump, tag, datum);
    }

    fn read_swapped_VLM_world_file_next_Q(&self) -> QWord {
        let mut w = unsafe { self.worlds.get(&self.world).unwrap() };

        while w.current_Q_number >= VLMPAGE_SIZE_QS {
            self.read_swapped_VLM_world_file_page(w.current_page_number + 1);
            w.current_Q_number -= VLMPAGE_SIZE_QS;
        }
        let q = self.read_swapped_VLM_world_file_Q(w.current_Q_number);
        w.current_Q_number += 1;

        return q;
    }
}

pub fn byte_swap_world(mut world_pathname: &str, mut search_path: &str) {
    // let mut world = World::new();
    // let mut a_world= World::new();

    // world.pathname = world_pathname;
    // open_world_file(&mut world, true);
    // let originalWorld = &mut world;
    // find_parent_worlds(&mut world, search_path);
    // a_world = &mut world;

    // while !a_world.is_none() {
    //     if a_world.format == LoadFileFormat::VLMWorldFormat && a_world.byte_swapped ==true {
    //         ByteSwapOneWorld(a_world);
    //     }
    //     a_world = a_world.parent_world;
    // }
}

fn ByteSwapOneWorld(world: &mut World) {
    // let mut bakPathname =  Path::new("") ;
    // let mut block: Vec<u32>  = vec![0; VLMPAGE_SIZE_QS / 4];
    // let mut dataStart: usize = 0;
    // let mut dataEnd: usize = 0;
    // let mut offset: usize = 0;
    // let mut wordBlockStart:  u32 = 0;
    // let mut newFD: u32 = 0;

    // let mut newPathname = Path::new(&format!("{}.swap", world.pathname.display()));
    // print!("Swapping bytes in {} ...", world.pathname.display());

    // if fstat(world.fd, &mut worldStat) < 0  {
    //     world.close(true);
    //     vpunt(format!("Unable to determine attributes of world file {}", world.pathname.display()));
    // }

    // newFD = open(
    //     newPathname, 0o1  | 0o100  | 0o1000, 0o400 | 0o200 | 0o400  >> 3 | 0o400  >> 3  >> 3);
    // if newFD < 0  {
    //     world.close(true);
    //     vpunt(format!("Unable to create world file {}", newPathname));
    // }

    // offset = 0;
    // dataStart = VLMPAGE_SIZE_QS  * world.vlm_data_page_base;
    // dataEnd = if world.vlm_data_page_base > world.vlm_tags_page_base {
    //     worldStat.st_size
    // } else {
    //     (VLMPAGE_SIZE_QS  * world.vlm_tags_page_base)
    // };

    // wordBlockStart = &mut block as *mut [libc::c_char; VLMPAGE_SIZE_QS] as *mut u32;
    // if lseek(world.fd, 0  as __off_t, 0) != 0 {
    //     world.close(true);
    //     vpunt(format!("Unable to seek to start of world file {}", world.pathname.display()));
    // }

    // while offset < worldStat.st_size {
    //     if VLMPAGE_SIZE_QS != read(world.fd, block.as_mut_ptr(), VLMPAGE_SIZE_QS) {
    //      world.close(true);
    //         vpunt(format!("Unable to read data from world file {}", world.pathname.display()));
    //     }

    //     if 0 == offset {
    //         wordBlockStart = VLMWORLD_FILE_COOKIE;
    //     }

    //     if offset >= dataStart && offset.wrapping_add(VLMPAGE_SIZE_QS) <= dataEnd {
    //         let mut nWords: u32 = ((VLMPAGE_SIZE_QS  + 3) / 4) ;
    //         let mut i: u32 = 0;
    //         let mut wordP: u32 = wordBlockStart;
    //         i = 0;
    //         while i < nWords {
    //             wordP = __bswap_32(wordP);
    //             i += 1;
    //             wordP = wordP[1];
    //         }
    //     }

    //     if VLMPAGE_SIZE_QS != write(newFD, block.as_mut_ptr() , VLMPAGE_SIZE_QS) {
    //         world.close(true);
    //         vpunt(format!("Unable to write data to world file {}", newPathname.to_string()));
    //     }

    //     offset = offset.wrapping_add(VLMPAGE_SIZE_QS);
    // }

    // world.close(true);
    // close(newFD);

    // bakPathname = format!("{}.bak", world.pathname.display());
    // if rename(world.pathname, bakPathname) < 0  {
    //     world.close(true);
    //     vpunt(format!("Unable to rename world file {} to {}", world.pathname.display(), bakPathname));
    // }
    // if rename(newPathname, world.pathname) < 0  {
    //     world.close(true);
    //     vpunt(format!("Unable to rename world file {} to {}", newPathname.display(), world.pathname.display()));
    // }

    // printf("done.\n");
}

pub fn virtual_memory_write_block_constant(
    mut vma: u32,
    mut object: *mut QWord,
    mut count: u32,
    increment: bool,
) -> u32 {
    // let mut data: *mut isize = &mut *DataSpace.offset(vma ) as *mut isize;
    // let mut tag: *mut Tag = &mut *TagSpace.offset(vma ) as *mut Tag;
    // let mut ctag: Tag = (*object).parts.tag as Tag;
    // let mut cdata: isize = (*object).parts.data.u ;
    // let mut edata: *mut isize =
    //     &mut *DataSpace.offset(vma.wrapping_add(count) ) as *mut isize;
    // memory_vma = vma;
    // memset(
    //     tag as *mut libc::c_uchar ,
    //     ctag,
    //     (count).wrapping_mul(::std::mem::size_of::<Tag>()),
    // );
    // match increment {
    //     0 => {
    //         if cdata == 0 {
    //             memset(
    //                 data as *mut libc::c_uchar ,
    //                 0  as libc::c_uchar,
    //                 (count)
    //                     .wrapping_mul(::std::mem::size_of::<isize>()),
    //             );
    //         } else {
    //             while data < edata {
    //                 let fresh5 = data.offset(1);
    //                 *fresh5 = cdata;
    //                 memory_vma = memory_vma.wrapping_add(1);
    //             }
    //         }
    //     }
    //     1 => {
    //         while data < edata {
    //             *data.offset(1) = cdata.wrapping_add(1);
    //             memory_vma = memory_vma.wrapping_add(1);
    //         }
    //     }
    //     _ => {
    //         while data < edata {
    //             let fresh8 = data;
    //             data = data.offset(1);
    //             *fresh8 = cdata;
    //             cdata = (cdata).wrapping_add(increment)
    //                 ;
    //             memory_vma = memory_vma.wrapping_add(1);
    //         }
    //     }
    // }
    return 0;
}

pub fn virtual_memory_write(mut vma: u32, object: QWord) -> u32 {
    // memory_vma = vma;
    // *DataSpace.offset(vma ) = (*object).parts.data.u ;
    // *TagSpace.offset(vma ) = (*object).parts.tag as Tag;
    return 0;
}

///
/// Load a map in to the GlobalContext world structure
impl GlobalContext {
    pub fn map_world_load(&self, start: u32, length: u32, offset: u32) -> u32 {
        // According to the doc, by mapping PRIVATE, writes to the address
        //  will not go to the file, so we get copy-on-write for free.  The
        //  only reason we map read-only, is to catch modified for IDS */
        //  --- for now, we don't try to discover modified: it seems to run us out of map entries
        let mut vma = start;
        let mut remaining = length;

        let attr = default_attributes(false, true);
        let prot = compute_protection(attr);

        let mut data_count: u32 = 0;
        let mut tag_count: u32 = 0;
        let mut words: u32 = 0;
        let mut mapped_world_words: u32 = 0;
        let mut unmapped_world_words: u32 = 0;
        let mut swap_map_entries: u32 = 0;
        let mut file_map_entries: u32 = 0;

        let mut w = self.worlds.get(&self.world).unwrap();
        let world_file = w.fd.as_ref().unwrap();
        let mmap_buf = unsafe { Mmap::map(&world_file) };

        // sigh, have to copy partial pages and pages that already exist (e.g., shared FEP page)
        while remaining > 0 {
            while memory_wad_offset(vma) != 0 || unsafe { _GC.vma_created_p(vma) } {
                words = min(MEMORY_PAGE_SIZE - memory_page_offset(vma), remaining);

                // ensure_virtual_address(vma);
                data_count = words * size_of::<u32>() as u32;
                tag_count = words + size_of::<QTag>() as u32;

                // Adjust the protection to catch modifications to world pages
                unsafe { _GC.vma_set_created(vma) };

                vma += words;
                // offset += data_count;
                remaining -= words;
                unmapped_world_words += words;
            }

            swap_map_entries += 1;

            // Set the attributes for mapped in pages
            if remaining > 0 {
                let mut limit: u32 = remaining - memory_page_offset(remaining);
                words = 0;
                while words < limit && unsafe { !_GC.wad_created(vma) } {
                    let wad_limit: u32 = words + MEMORYWAD_SIZE;
                    // TODO: Check should not be sweeping through all addresses.
                    while words < wad_limit {
                        unsafe { _GC.vma_set_attr(vma + words, default_attributes(false, true)) };
                        words += MEMORY_PAGE_SIZE;
                    }
                }

                data_count = words * size_of::<usize>() as u32;
                vma += words;
                // offset += data_count;
                remaining -= words;
                mapped_world_words += words;
                file_map_entries += 2;
            }
        }
        return vma;

        // C VERSION

        // for (; length > 0;)
        // {
        //     /* sigh, have to copy partial pages and pages that already exist
        //     /* (e.g., shared FEP page) */
        //     for (; (length > 0) && (MemoryWadOffset(vma) || Created(vma) || (length < MemoryWad_Size));)
        //     {
        //         words = MemoryPage_Size - MemoryPageOffset(vma);
        //         if (words > length)
        //         {
        //             words = length;
        //         }
        //         EnsureVirtualAddress(vma);

        //         dataCount = sizeof(Integer) * words;
        //         if (dataoffset != lseek(worldfile, dataoffset, SEEK_SET))
        //         {
        //             vpunt(NULL, "Unable to seek to data offset %d in world file", dataoffset);
        //         }
        //         if (dataCount != read(worldfile, MapVirtualAddressData(vma), dataCount))
        //         {
        //             vpunt(NULL, "Unable to read data page %d from world file", MemoryPageNumber(vma));
        //         }

        //         tagCount = sizeof(Tag) * words;
        //         if (tagoffset != lseek(worldfile, tagoffset, SEEK_SET))
        //         {
        //             vpunt(NULL, "Unable to seek to tag offset %d in world file", tagoffset);
        //         }
        //         if (tagCount != read(worldfile, MapVirtualAddressTag(vma), tagCount))
        //         {
        //             vpunt(NULL, "Unable to read tag page %d from world file", MemoryPageNumber(vma));
        //         }

        //         /* Adjust the protection to catch modifications to world pages */
        //         SetCreated(vma);

        //         vma += words;
        //         dataoffset += dataCount;
        //         tagoffset += tagCount;
        //         length -= words;
        //         unmapped_world_words += words;
        //     }
        //     swap_map_entries += 1;

        //     if (length > 0)
        //     {
        //         int limit = length - MemoryWadOffset(length);

        //         /* Set the attributes for mapped in pages */
        //         for (words = 0; (words < limit) && !WadCreated(vma + words);)
        //         {
        //             int wadlimit = words + MemoryWad_Size;
        //             VMAttribute *pattr = &VMAttributeTable[MemoryPageNumber(vma + words)];

        //             for (; words < wadlimit; words += MemoryPage_Size, pattr++)
        //             {
        //                 *pattr = attr;
        //             }
        //         }

        //         data = (caddr_t)&DataSpace[vma];
        //         tag = (caddr_t)&TagSpace[vma];
        //         if (data != mmap(data, dataCount = sizeof(Integer) * words, PROT_READ | PROT_WRITE | PROT_EXEC,
        //                          MAP_FILE | MAP_PRIVATE | MAP_FIXED, worldfile, dataoffset))
        //         {
        //             vpunt(NULL, "Couldn't map %d world data pages at %lx for VMA %x", MemoryPageNumber(words), data, vma);
        //         }
        //         if (tag != mmap(tag, tagCount = sizeof(Tag) * words, prot, MAP_FILE | MAP_PRIVATE | MAP_FIXED, worldfile,
        //                         tagoffset))
        //         {
        //             vpunt(NULL, "Couldn't map %d world tag pages at %lx for VMA %x", MemoryPageNumber(words), tag, vma);
        //         }

        //         vma += words;
        //         dataoffset += dataCount;
        //         tagoffset += tagCount;
        //         length -= words;
        //         mapped_world_words += words;
        //         file_map_entries += 2;
        //     }
        // }
        // return (vma);

        // let mut data: u64 = "";
        // let mut tag: u64 = "";
        // let mut dataCount: size_t = 0;
        // let mut tagCount: size_t = 0;
        // let mut words: u32 = 0;
    }

    pub fn VLM_load_map_data(self, map_selector: MapEntrySelector, index: usize) -> u32 {
        let mut w = unsafe { &_GC.worlds.get(&self.world).unwrap() };
        let mut entry = (*w).select_entries(map_selector).data[index];

        match entry.map_code {
            LoadMapEntryOpcode::DataPages => {
                // let map_world = map_entry.world;
                let page_number = unsafe { entry.data.parts.data.u };
                if w.byte_swapped {
                    // ensure_virtual_address_range(entry.address, entry.count, false);
                    self.read_swapped_VLM_world_file_page(page_number);

                    let mut the_address = entry.address;
                    w.current_Q_number = 0;
                    println!("LoadMapDataPages @ {}, count {}", the_address, entry.count,);

                    for _ in 0..entry.count {
                        virtual_memory_write(
                            the_address,
                            self.read_swapped_VLM_world_file_next_Q(),
                        );
                        the_address += 1;
                    }
                } else {
                    let file_offset = 8192 * (w.data_page_base + page_number * 4);
                    // let tag_offset = 8192 * (&w.vlm_data_page_base + page_number * 1);

                    self.map_world_load(entry.address, entry.count, file_offset);
                }
            }
            LoadMapEntryOpcode::ConstantIncremented => {
                // ensure_virtual_address_range(entry.address, entry.count, false);
                virtual_memory_write_block_constant(
                    entry.address,
                    &mut entry.data,
                    entry.count,
                    true,
                );
            }
            LoadMapEntryOpcode::Constant => {
                // ensure_virtual_address_range(entry.address, entry.count, false);
                virtual_memory_write_block_constant(
                    entry.address,
                    &mut entry.data,
                    entry.count,
                    false,
                );
            }
            LoadMapEntryOpcode::Copy => {
                // ensure_virtual_address_range(entry.address, entry.count, false);
                let mut the_address = entry.address;
                let mut the_source_address = unsafe { entry.data.parts.data.u };

                for i in 0..entry.count {
                    virtual_memory_write(the_address, virtual_memory_read(the_source_address));
                    the_address += 1;
                    the_source_address += 1;
                }
            }
            _ => {
                self.close(true);
                vpunt(format!(
                    "Unknown load map opcode {} in world file {}",
                    entry.map_code,
                    w.pathname.display().to_string()
                ))
            }
        }

        return entry.count;
    }
}
