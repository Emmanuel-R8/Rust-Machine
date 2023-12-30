// #![feature(use_extern_macros, prox_macro_non_items)]
use log::warn;

use sets::Set;
use std::cmp::Ordering;
use std::fs::{ DirEntry, File };
use std::io::Read;
use std::path::{ Path, PathBuf };
use std::{ fmt, process };
use uuid::Uuid;

use crate::common::constants::{
    LoadFileFormat,
    QTag,
    IVORY_PAGE_SIZE_BYTES,
    IVORY_PAGE_SIZE_QS,
    IVORY_WORLD_FILE_COOKIE,
    VLMPAGE_SIZE_QS,
    VLMVERSION1_AND_ARCHITECTURE,
    VLMVERSION2_AND_ARCHITECTURE,
    VLMWORLD_FILE_COOKIE,
    VLMWORLD_FILE_COOKIE_SWAPPED,
};

use crate::common::memory_cell::MemoryCell;
use crate::common::types::Address;
use crate::emulator::emulator::GlobalContext;
use crate::utils::pack_8_to_32;

// A single load map entry -- See SYS:NETBOOT;WORLD-SUBSTRATE.LISP for details
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct LoadMapEntry {
    pub addr: Address, // VMA to be filled in by this load map entry
    // NOTE: opcount and opcode are field of a struct op{} in the C code
    pub count: u32, // Number of words to be filled in by this entry
    pub map_code: LoadMapEntryOpcode, // An LoadMapEntryOpcode specifying how to do so
    pub data: MemoryCell, // Interpretation is based on the opcode
    // pub world: Rc<RefCell<World<'a>>>, // Ref to World from which this entry was obtained
    // !!!!!!!!! Should not be needed to link back
}

pub fn clone(map_entries: &Set<LoadMapEntry>) -> Set<LoadMapEntry> {
    let res: Set<LoadMapEntry> = Set::new_ordered(&[], true);

    for m in &map_entries.data {
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
            addr: 0,
            count: 0,
            map_code: LoadMapEntryOpcode::Constant,
            data: MemoryCell::default(),
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
    Copy, // Copy an existing piece of memory
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
        match self.addr.cmp(&other.addr) {
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
    pub pathname: PathBuf, // -> Pathname of the world file
    pub fd: Option<File>, // Unix filedes # if the world file is open
    pub format: LoadFileFormat, // A LoadFileFormat indicating the type of file
    pub byte_swapped: bool, // World is byte swapped on this machine (VLM only)

    pub page_base: u32,
    pub page: Vec<MemoryCell>, // -> The current VLM format pagemut
    pub data_page_base: u32, // Block number of first page of data (VLM only)
    pub data_page: Vec<u32>, // -> The data of the current VLM format page
    pub tags_page_base: u32, // Block number of first page of tags (VLM only)
    pub tags_page: Vec<QTag>, // -> The tags of the current VLM format page
    // [MemoryCell; IVORY_PAGE_SIZE_BYTES] -> The data of the current Ivory format page.
    // TODO: rename to current_data-page
    pub ivory_data_page: Vec<MemoryCell>,
    // Size is 0x500 = 0x100 for tags + 0x400 for data
    // pub current_page_number: u32, // Page number of the page in the buffer, if any. -1 means not pointing yet
    // pub current_q_number: u32,    // Q number within the page to be read
    pub timestamp_1: u32, // Unique ID of this world, part 1 ...
    pub timestamp_2: u32, // ... part 2

    pub generation: u32, // Generation number of this world (> 0 if IDS)
    // pub parent_world: Option<Rc<RefCell<&'a World<'a>>>>, // -> Parent of this world if it's an IDS
    pub parent_world: Uuid, // -> Parent of this world if it's an IDS
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
            format: LoadFileFormat::Ivory,
            byte_swapped: false,

            page_base: 0,
            page: vec![],
            data_page_base: 0,
            data_page: vec![],
            tags_page_base: 0,
            tags_page: vec![],
            ivory_data_page: vec![MemoryCell::default(); IVORY_PAGE_SIZE_BYTES as usize],

            // current_page_number: 0,
            // current_q_number: 0,
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

    pub fn load_image(&mut self, pathname: &str, punt_on_errors: bool) {
        let mut page_bases: MemoryCell = MemoryCell::default();
        let mut wired_count_q: u32 = 0;
        let mut unwired_count_q: u32 = 0;
        let mut pages_base_q: u32 = 0;
        let mut first_sysout_q: u32 = 0;
        let mut first_map_q: u32 = 0;

        let mut f = File::open(pathname).unwrap();

        // Read 4 bytes for the file magic cookie
        let mut cookie = [0 as u8; 4];
        let _ = f.read(&mut cookie);

        match pack_8_to_32(cookie) {
            VLMWORLD_FILE_COOKIE => {
                self.format = LoadFileFormat::VLM;
                self.byte_swapped = false;
            }

            VLMWORLD_FILE_COOKIE_SWAPPED => {
                self.format = LoadFileFormat::VLM;
                self.byte_swapped = true;
            }

            IVORY_WORLD_FILE_COOKIE => {
                self.format = LoadFileFormat::Ivory;
                wired_count_q = 1;
                unwired_count_q = 2;
                first_sysout_q = 0;
                first_map_q = 8;
            }

            _ => {
                if punt_on_errors {
                    panic_exit(format!("Format of world file {} is unrecognized", pathname));
                }
            }
        }

        self.ivory_data_page = vec![MemoryCell::default(); (IVORY_PAGE_SIZE_BYTES / 4) as usize];
        // w.current_page_number = 0;

        // The header and load maps for both VLM and Ivory world files are stored using Ivory file format settings (i.e., 256 Qs per 1280 byte page)
        if self.format == LoadFileFormat::VLM {
            match read_ivory_world_file_q(self, 0).as_raw() {
                VLMVERSION1_AND_ARCHITECTURE => {
                    wired_count_q = 1;
                    unwired_count_q = 0;
                    pages_base_q = 3;
                    first_sysout_q = 0;
                    first_map_q = 8;
                }
                VLMVERSION2_AND_ARCHITECTURE => {
                    wired_count_q = 1;
                    unwired_count_q = 0;
                    pages_base_q = 2;
                    first_sysout_q = 3;
                    first_map_q = 8;
                }
                _ => {
                    panic_exit(
                        format!("Format magic code of world file {} is unrecognized", pathname)
                    );
                }
            }
        }

        if self.format == LoadFileFormat::VLM {
            page_bases = read_ivory_world_file_q(self, pages_base_q);
            self.data_page_base = page_bases.as_raw();
            self.tags_page_base = page_bases.tag() as u32;
        }

        if first_sysout_q != 0 {
            // w.current_q_number = first_sysout_q;

            // w.generation = unsafe {
            //     lisp_obj_data(read_ivory_world_file_next_q(&mut w)).unwrap().u().unwrap()
            // };
            // w.timestamp_1 = unsafe {
            //     lisp_obj_data(read_ivory_world_file_next_q(&mut w)).unwrap().u().unwrap()
            // };
            // w.timestamp_2 = unsafe {
            //     lisp_obj_data(read_ivory_world_file_next_q(&mut w)).unwrap().u().unwrap()
            // };
            // w.parent_timestamp_1 = unsafe {
            //     lisp_obj_data(read_ivory_world_file_next_q(&mut w)).unwrap().u().unwrap()
            // };
            // w.parent_timestamp_2 = unsafe {
            //     lisp_obj_data(read_ivory_world_file_next_q(&mut w)).unwrap().u().unwrap()
            // };
        } else {
            self.generation = 0;
            self.timestamp_2 = 0;
            self.timestamp_1 = 0;
            self.parent_timestamp_2 = 0;
            self.parent_timestamp_1 = 0;
        }
        // w.current_q_number = first_map_q;
        // w.wired_map_entries = read_load_map(&mut w, MapEntrySelector::Wired);
        // w.unwired_map_entries = read_load_map(&mut w, MapEntrySelector::Unwired);

        self.fd = Some(f);
    }

    // Select the specified Set<LoadMapEntry>
    pub fn select_entries(&mut self, selector: MapEntrySelector) -> &Set<LoadMapEntry> {
        return match selector {
            MapEntrySelector::Wired => &self.wired_map_entries,
            MapEntrySelector::MergedWired => &self.merged_wired_map_entries,
            MapEntrySelector::Unwired => &self.unwired_map_entries,
            MapEntrySelector::MergedUnwired => &self.merged_unwired_map_entries,
        };
    }

    // Close a world file
    pub fn close(&mut self, _close_parent: bool) {
        self.fd = None;
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
            format: LoadFileFormat::Ivory,
            byte_swapped: false,
            data_page_base: 0,
            tags_page_base: 0,
            page_base: 0,
            data_page: vec![],
            tags_page: vec![],
            page: vec![],
            ivory_data_page: vec![MemoryCell::default(); IVORY_PAGE_SIZE_BYTES as usize],
            // current_page_number: 0,
            // current_q_number: 0,
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
            // current_page_number: self.current_page_number.clone(),
            // current_q_number: self.current_q_number.clone(),
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
    pub extent: u32, // Number of words starting at this address to save
}

#[derive(Default, Debug)]
pub struct SaveWorldData {
    pub patname: u32, // Pathname of the world file (a DTP-STRING)
    pub entry_count: u32, // Number of address/extent pairs to follow
    pub entries: Option<SaveWorldEntry>, //
}

// Print an error message and terminate the VLM
pub fn panic_exit(msg: String) {
    warn!("Emulator final log");
    warn!("{}", msg);
    warn!("----------------------------------------------------------");
    warn!("TERMINATING EMULATOR\n");

    process::exit(1);
}

pub fn punt_world(ctx: &mut GlobalContext, msg: String) {
    ctx.close(true);
    panic_exit(msg);
}

pub fn read_ivory_world_file_page(_w: &mut World, _page_number: u32) {
    todo!()
}

// Merges a foreground load map and a background load map together into a single load map
// background are the entries in the parent world
// foreground are the child world shadowing the parent (background)
pub fn merge_a_map<'a>(
    world: &World,
    fore_map: &Set<LoadMapEntry>,
    back_map: &Set<LoadMapEntry>
) -> Set<LoadMapEntry> {
    // Load the relevant maps
    // We need mutable copies to adjust ends and starts if need at each iteration.
    let mut new_back = clone(&back_map);
    let new_fore = clone(fore_map);

    let n_back = new_back.data.len() as u32;
    let n_fore = new_fore.data.len() as u32;

    // Resulting new map
    let new_map_entries: Set<LoadMapEntry> = Set::<LoadMapEntry>::new_ordered(&[], true);

    // See SYS:IFEP;WORLD-SUBSTRATE.LISP for an explanation of the maximum number of entries
    let max = n_back + n_fore + n_fore;
    if max == 0 {
        return new_map_entries;
    }

    let _page_size_qs = match world.format {
        LoadFileFormat::VLM => VLMPAGE_SIZE_QS,
        _ => IVORY_PAGE_SIZE_QS,
    };

    let mut fore_idx: u32 = 0;
    let mut back_idx: u32 = 0;
    loop {
        if new_back.data[back_idx as usize].map_code != LoadMapEntryOpcode::DataPages {
            back_idx += 1;
            continue;
        }

        let fore_start: Address = new_fore.data[fore_idx as usize].addr;
        let fore_final: Address =
            fore_start + (new_fore.data[fore_idx as Address].count as Address) - 1;

        let back_start: Address = new_back.data[fore_idx as usize].addr;
        let back_final: Address =
            back_start + (new_back.data[fore_idx as Address].count as Address) - 1;

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
                addr: new_back.data[back_idx as usize].addr,
                count: (new_fore.data[fore_idx as usize].addr as u32) -
                ((new_back.data[back_idx as usize].addr as u32) +
                    new_back.data[back_idx as usize].count -
                    1),
                map_code: new_back.data[back_idx as usize].map_code,
                data: new_back.data[back_idx as usize].data,
            });
            new_back.data[back_idx as usize].addr = new_fore.data[fore_idx as usize].addr;
            new_back.data[back_idx as usize].count = (back_final -
                new_back.data[back_idx as usize].addr +
                1) as u32;
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
            new_back.data[back_idx as usize].addr = fore_final + 1;
            new_back.data[back_idx as usize].count = (back_final -
                new_back.data[back_idx as usize].addr +
                1) as u32;
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

// pub fn read_load_map(w: &mut World, map_selector: MapEntrySelector) -> Set<LoadMapEntry> {
//     let map = match map_selector {
//         MapEntrySelector::Wired => w.wired_map_entries.data.clone(),
//         MapEntrySelector::MergedWired => w.merged_wired_map_entries.data.clone(),
//         MapEntrySelector::Unwired => w.unwired_map_entries.data.clone(),
//         MapEntrySelector::MergedUnwired => w.merged_unwired_map_entries.data.clone(),
//     };

//     let mut res = Set::<LoadMapEntry>::new_ordered(&[], true);

//     for e in map {
//         let mut new_entry = e.clone();

//         let q = read_ivory_world_file_next_q(w);
//         new_entry.addr = q.a().unwrap();

//         let q = read_ivory_world_file_next_q(w);
//         let op = q.u().unwrap();
//         new_entry.count = op & 0x00FF_FFFF;
//         new_entry.map_code = match (op & 0xFF00_0000) >> 24 {
//             0 => LoadMapEntryOpcode::DataPages,
//             1 => LoadMapEntryOpcode::Constant,
//             2 => LoadMapEntryOpcode::ConstantIncremented,
//             3 => LoadMapEntryOpcode::Copy,
//             _ => LoadMapEntryOpcode::default(),
//         };

//         new_entry.data = read_ivory_world_file_next_q(w);

//         res.insert(new_entry);
//     }

//     return res;
// }

// fn read_load_map(mut world: *mut World, mut nSet<LoadMapEntry>: u32, mut Set<LoadMapEntry>: *mut LoadMapEntry) {
//     let mut q: MemoryCell = LispObj {
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

pub fn read_ivory_world_file_q(w: &World, address: u32) -> MemoryCell {
    if address >= IVORY_PAGE_SIZE_BYTES {
        panic_exit(
            format!(
                "Invalid word number {} for world file {}",
                address,
                &w.pathname.display().to_string()
            )
        );
    }

    return w.ivory_data_page[address as usize];
}

// pub fn read_ivory_world_file_next_q(w: &mut World) -> MemoryCell {
//     let current_q_number:u32 = 0;

//     // If the the current address is too high, load the next page (several time if needed)
//     while w.current_q_number >= IVORY_PAGE_SIZE_BYTES {
//         read_ivory_world_file_page(w, w.current_page_number + 1);
//         w.current_q_number -= IVORY_PAGE_SIZE_BYTES;
//     }

//     let q = read_ivory_world_file_q(w, w.current_q_number);
//     w.current_q_number += 1;

//     return q;
// }

pub fn world_p(_candidate_world: DirEntry) -> bool {
    let _a_world = World::default();
    let mut _new_worlds: Vec<World>;
    let mut _candidate_pathname: &Path;

    // if candidate_world.file_name().len() > VLMWORLD_SUFFIX.len() {
    //     a_world.pathname = PathBuf::from("/")
    //         .join(&ctx.scanning_dir)
    //         .join(candidate_world.file_name());
    // } else {
    //     return false;
    // }
    unimplemented!()
}

pub fn write_ivory_world_file_next_q(_w: &mut World, _q: MemoryCell) {}
// fn write_ivory_world_file_next_Q(mut world: *mut World, mut q: MemoryCell) {
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

pub fn virtual_memory_read(_addr: Address) -> MemoryCell {
    todo!();
}
pub fn copy_ivory_world_file_next_q(world: &mut World, from: Address) {
    let q = virtual_memory_read(from);
    write_ivory_world_file_next_q(world, q);
}

fn write_vlmworld_file_header(_world: &mut World) {
    todo!()
    // let mut generation_Q: MemoryCell = make_lisp_obj_u(QTag::Null, 0);

    // let mut q = MemoryCell::new();
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

pub fn map_virtual_address_data(_addr: Address) -> u32 {
    todo!()
}
pub fn map_virtual_address_tag(_addr: Address) -> u32 {
    todo!()
}

impl World {
    pub fn write_vlm_world_file_pages(&self) {
        let _page_number: u32 = 0;
        let _word_count: u32 = 0;
        let _byte_count: u32 = 0;
        let _offset: u64 = 0;
        let _increment: u32 = 0;
        let _i: usize = 0;

        // MemoryCell = 1 byte tag / 4 bytes data
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
}

// fn prepare_to_write_ivory_world_file_page(w: &mut World, page_number: u32) {
//     w.current_page_number = page_number;
//     w.current_q_number = 0;
//     w.ivory_data_page = vec![MemoryCell::default(); IVORY_PAGE_SIZE_BYTES as usize];
// }

// fn write_ivory_world_file_page(world: &mut World) {
//     if 0 == world.current_q_number {
//         return;
//     }

// todo!()
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
// }

// pub fn byte_swap_world(mut world_pathname: &str, mut search_path: &str) {
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
// }

fn byte_swap_one_world(_world: &mut World) {
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
    _vma: u32,
    _object: *mut MemoryCell,
    _count: u32,
    _increment: bool
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

pub fn virtual_memory_write(_vma: u32, _object: MemoryCell) -> u32 {
    // memory_vma = vma;
    // *DataSpace.offset(vma ) = (*object).parts.data.u ;
    // *TagSpace.offset(vma ) = (*object).parts.tag as Tag;
    return 0;
}
