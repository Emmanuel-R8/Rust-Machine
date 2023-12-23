use std::cmp::min;
use std::collections::HashMap;
use std::fs::{ read_dir, DirEntry };
use std::mem::size_of;
use std::ops::Div;
use std::path::{ Path, PathBuf };

use memmap::Mmap;
use num::Integer;
use sets::Set;
use uuid::Uuid;

use crate::common::constants::{
    QTag,
    VMAttribute,
    CDR,
    IVORY_PAGE_SIZE_BYTES,
    IVORY_PAGE_SIZE_QS,
    MEMORYWAD_SIZE,
    MEMORY_ADDRESS_PAGE_SHIFT,
    MEMORY_PAGE_SIZE,
    VLMMAXIMUM_HEADER_BLOCKS,
    VLMPAGE_SIZE_QS,
    VLMWORLD_FILE_V2_FIRST_MAP_Q,
    VLMWORLD_SUFFIX,
    VMATTRIBUTE_CREATED_DEFAULT,
    VMATTRIBUTE_EMPTY,
    VMATTRIBUTE_EXISTS,
};
use crate::common::memory_cell::MemoryCell;
use crate::hardware::cpu::{
    read_control_argument_size,
    read_control_caller_frame_size,
    write_control_argument_size,
    write_control_caller_frame_size,
    CPU,
};
use crate::hardware::memory::{
    compute_protection,
    default_attributes,
    memory_page_offset,
    memory_wad_offset,
};
use crate::utils::byte_swap_32;
use crate::world::world::{
    merge_a_map,
    panic_exit,
    virtual_memory_read,
    LoadMapEntry,
    LoadMapEntryOpcode,
    World,
};

#[derive()]
pub struct GlobalContext<'a> {
    pub cpu: CPU,
    pub mem: [MemoryCell; 1 << 31] /* 2^32 bytes of tags + data */,
    pub attribute_table: [VMAttribute; 1 << (32 - MEMORY_ADDRESS_PAGE_SHIFT)],

    pub world: Uuid,
    pub worlds: HashMap<Uuid, &'a World>,

    pub scanning_dir: PathBuf,

    pub unmapped_world_words: u32,
    pub mapped_world_words: u32,
    pub file_map_entries: u32,
    pub swap_map_entries: u32,
}

impl<'a> Default for GlobalContext<'a> {
    fn default() -> Self {
        return GlobalContext {
            cpu: CPU::default(),
            mem: [MemoryCell::default(); 1 << 31],
            attribute_table: [VMATTRIBUTE_EMPTY; 1 << (32 - MEMORY_ADDRESS_PAGE_SHIFT)],

            world: Uuid::nil(),
            worlds: HashMap::new(),

            scanning_dir: PathBuf::from(""),

            unmapped_world_words: 0,
            mapped_world_words: 0,
            file_map_entries: 0,
            swap_map_entries: 0,
        };
    }
}

impl<'a> GlobalContext<'a> {
    pub fn new() -> Self {
        return GlobalContext {
            cpu: CPU::default(),
            mem: [MemoryCell::default(); 1 << 31],
            attribute_table: [VMATTRIBUTE_EMPTY; 1 << (32 - MEMORY_ADDRESS_PAGE_SHIFT)],

            world: Uuid::nil(),
            worlds: HashMap::new(),

            scanning_dir: PathBuf::from(""),

            unmapped_world_words: 0,
            mapped_world_words: 0,
            file_map_entries: 0,
            swap_map_entries: 0,
        };
    }

    pub fn write_at(&mut self, addr: MemoryCell, val: MemoryCell) {
        self.mem[addr.as_address() as usize] = val;
    }

    pub fn inc_and_write_at(&mut self, addr: MemoryCell, val: MemoryCell) -> MemoryCell {
        let a = addr.inc();
        self.write_at(a, val);

        return a;
    }

    pub fn write_at_and_inc(&mut self, addr: MemoryCell, val: MemoryCell) -> MemoryCell {
        self.write_at(addr, val);
        return addr.inc();
    }

    pub fn read_at(&self, addr: MemoryCell) -> MemoryCell {
        return self.mem[addr.as_address()];
    }

    pub fn inc_and_read_at(&mut self, addr: MemoryCell) -> (MemoryCell, MemoryCell) {
        let a = addr.inc();
        return (self.read_at(a), a);
    }

    pub fn read_at_and_inc(&mut self, addr: MemoryCell) -> (MemoryCell, MemoryCell) {
        let a = addr.inc();
        return (self.read_at(addr), a);
    }

    /// Push one empty frame
    /// See IMAS p 242 for frame format
    pub fn push_one_fake_frame(&mut self) {
        // Push continuation
        let mut q = self.cpu.continuation.clone();
        q.set_cdr(CDR::Jump);

        self.cpu.sp = self.inc_and_write_at(self.cpu.sp, q);

        // The new stack pointer is stored as the new frame pointer (beginning of the frame)
        self.cpu.fp = self.cpu.sp;

        // Push the control register
        let q = MemoryCell::new_cdr_tag_u(CDR::Jump, QTag::Fixnum, self.cpu.control.as_raw());
        self.cpu.sp = self.inc_and_write_at(self.cpu.sp, q);

        // Create a new control register
        self.cpu.control = MemoryCell::new_cdr_tag_i(CDR::Jump, QTag::Fixnum, 0);
        write_control_argument_size(&mut self.cpu.control, 2);
        write_control_caller_frame_size(
            &mut self.cpu.control,
            self.cpu.sp.as_address() - self.cpu.fp.as_address()
        );

        self.cpu.continuation = self.cpu.pc;
    }

    /// Pop one empty frame
    pub fn pop_one_fake_frame(&mut self) {
        // Set the stack pointer at the address of the current frame
        self.cpu.sp = self.cpu.fp;

        // Determine the next frame pointer by decreasing by the frame size
        self.cpu.fp_inc(read_control_caller_frame_size(self.cpu.control));

        // Restore the PC using the stored continuation
        self.cpu.pc = self.cpu.continuation;

        // Temporary copy of FP
        (self.cpu.continuation, self.cpu.fp) = self.read_at_and_inc(self.cpu.fp);
        self.cpu.set_control(self.read_at(self.cpu.fp).as_raw());

        self.cpu.lp = self.cpu.fp.clone();
        self.cpu.lp_inc(read_control_argument_size(self.cpu.control));
    }

    #[inline]
    pub fn vma_created_p(&self, vma: u32) -> bool {
        return (self.attribute_table[vma as usize] & VMATTRIBUTE_EXISTS) == 0;
    }

    #[inline]
    pub fn vma_set_created(&mut self, vma: u32) {
        self.attribute_table[vma as usize] = VMATTRIBUTE_CREATED_DEFAULT;
    }

    #[inline]
    pub fn vma_set_attr(&mut self, vma: u32, attr: VMAttribute) {
        self.attribute_table[vma as usize] = attr;
    }

    #[inline]
    pub fn vma_clear(&mut self, vma: u32) {
        self.attribute_table[vma as usize] = VMATTRIBUTE_EMPTY;
    }

    // Close the world file
    pub fn close(&mut self, close_parent: bool) {
        let mut pw = Uuid::nil();
        // Get the parent world

        match self.worlds.get(&self.world) {
            Some(w) => {
                pw = (*w).parent_world;
                self.worlds.remove(&self.world);
            }
            _ => {}
        }

        if close_parent {
            self.worlds.remove(&pw);
        }
    }

    pub fn merge_parent_load_map(&self, world: Uuid) -> (Set<LoadMapEntry>, Set<LoadMapEntry>) {
        if world.is_nil() {
            return (
                Set::<LoadMapEntry>::new_ordered(&[], true),
                Set::<LoadMapEntry>::new_ordered(&[], true),
            );
        }

        // If at the top of the topmost parent (no generation above)
        let w = self.worlds.get(&world).unwrap();

        if w.generation == 0 || w.parent_world.is_nil() {
            return (
                Set::<LoadMapEntry>::new_ordered(&[], true),
                Set::<LoadMapEntry>::new_ordered(&[], true),
            );
        }

        // let new_merged_wired_entries = Set::<LoadMapEntry>::new_ordered(&[], true);
        // let new_unmerged_wired_entries = Set::<LoadMapEntry>::new_ordered(&[], true);
        let (mut new_merged_wired_entries, mut new_unmerged_wired_entries) =
            self.merge_parent_load_map(w.parent_world);
        new_merged_wired_entries = merge_a_map(
            w,
            &new_merged_wired_entries,
            &w.merged_wired_map_entries
        );
        new_unmerged_wired_entries = merge_a_map(
            &w,
            &new_unmerged_wired_entries,
            &w.unwired_map_entries
        );

        return (new_merged_wired_entries, new_unmerged_wired_entries);
    }

    pub fn merge_load_maps(
        &mut self,
        world_search_path: String
    ) -> (Set<LoadMapEntry>, Set<LoadMapEntry>) {
        let w = self.worlds.get(&self.world).unwrap();
        if w.generation == 0 {
            return (
                Set::<LoadMapEntry>::new_ordered(&[], true),
                Set::<LoadMapEntry>::new_ordered(&[], true),
            );
        }

        let new_merged_wired_entries = Set::<LoadMapEntry>::new_ordered(&[], true);
        let new_unmerged_wired_entries = Set::<LoadMapEntry>::new_ordered(&[], true);

        self.find_parent_worlds(world_search_path);
        let w = self.worlds.get(&self.world);
        if w.is_none() {
            return (
                Set::<LoadMapEntry>::new_ordered(&[], true),
                Set::<LoadMapEntry>::new_ordered(&[], true),
            );
        }

        let pw = w.unwrap().parent_world;
        if pw.is_nil() {
            return (new_merged_wired_entries, new_unmerged_wired_entries);
        }

        return self.merge_parent_load_map(pw);
    }

    /// Find the ancestors of the user's world
    ///
    /// Searches the directory containing
    /// said world and then the world file search path for the ancestors.  If
    /// successful, the world.parentWorld slot will form a chain from the user's
    /// world to the base world
    /// TODO: REIMPLEMENT. THE CURRENT VERSION DOESN'T WORK AT ALL
    fn find_parent_worlds(&mut self, world_search_path: String) {
        // Get the list of all world files in the directory where that world is located
        let list_world_files = self.scan_one_directory(&self.scanning_dir);

        // Remove all worlds apart from current one
        let current_world = self.world;
        self.worlds.retain(|&u, _| u == current_world);

        let w = *self.worlds.get(&current_world).unwrap();
        let mut dir_components = w.pathname.ancestors();
        let full_path = dir_components.next();
        let base_directory = dir_components.next();

        let mut top_uuid = Uuid::nil();
        while w.generation > 0 {
            for (w_uuid, w_world) in self.worlds.iter_mut() {
                if
                    w_world.generation == w_world.generation - 1 &&
                    w_world.timestamp_1 == w_world.parent_timestamp_1 &&
                    w_world.timestamp_2 == w_world.parent_timestamp_2
                {
                    top_uuid = *w_uuid;
                    // w_world.parent_world = tmp_uuid;
                    break;
                }
            }

            if w.parent_world.is_nil() {
                panic_exit(format!("Unable to find parent of world file {}", w.pathname.display()));
            } else {
                self.world = w.parent_world.clone();
            }
        }
        self.close_extra_worlds();
    }

    /// Scan a directory looking for world files
    ///
    /// Adds all acceptable world files
    /// that are found to the worlds array defined above
    pub fn scan_one_directory(&self, dir: &Path) -> Vec<DirEntry> {
        // scan the directory specified in the global context and only keeps the world files.
        return read_dir(dir)
            .expect("Error reading the world directory")
            .filter_map(Result::ok)
            .filter(|f| f.path().ends_with(VLMWORLD_SUFFIX))
            .collect();
    }

    pub fn close_extra_worlds(&mut self) {
        // No use of clear() to avoid maintaining memory allocation
        self.worlds = HashMap::new();
    }

    pub fn initialise(&mut self) {
        let gc = &mut GlobalContext::default();

        // Push initial frames:  These are a lie, they will be popped when you
        // start, so that the "continuation" at 4 becomes the PC.  The
        // continuation and control for the running frame are NIL and 0,
        // respectively, thus returning from that frame will not adjust the FP
        // and the sequencer will know to halt on seeing NIL as a PC.
        gc.push_one_fake_frame();
        gc.push_one_fake_frame();

        // EnsureVirtualAddressRange(0xf8000100, 0xf00, false);
        // EnsureVirtualAddressRange(0xf8062000, 0x9e000, false);
    }

    ///
    /// Load a map in to the GlobalContext world structure
    pub fn map_world_load(&mut self, start: u32, length: u32, offset: u32) -> u32 {
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

        let w = self.worlds.get(&self.world).unwrap();
        let world_file = w.fd.as_ref().unwrap();
        let mmap_buf = unsafe { Mmap::map(&world_file) };

        // sigh, have to copy partial pages and pages that already exist (e.g., shared FEP page)
        while remaining > 0 {
            while memory_wad_offset(vma) != 0 || self.vma_created_p(vma) {
                words = min(MEMORY_PAGE_SIZE - memory_page_offset(vma), remaining);

                // ensure_virtual_address(vma);
                data_count = words * (size_of::<u32>() as u32);
                tag_count = words + (size_of::<QTag>() as u32);

                // Adjust the protection to catch modifications to world pages
                self.vma_set_created(vma);

                vma += words;
                // offset += data_count;
                remaining -= words;
                unmapped_world_words += words;
            }

            swap_map_entries += 1;

            // Set the attributes for mapped in pages
            if remaining > 0 {
                let limit: u32 = remaining - memory_page_offset(remaining);
                words = 0;
                while words < limit && !self.wad_created(vma) {
                    let wad_limit: u32 = words + MEMORYWAD_SIZE;
                    // TODO: Check should not be sweeping through all addresses.
                    while words < wad_limit {
                        self.vma_set_attr(vma + words, default_attributes(false, true));
                        words += MEMORY_PAGE_SIZE;
                    }
                }

                data_count = words * (size_of::<usize>() as u32);
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

    // pub fn vlm_load_map_data(&mut self, map_selector: MapEntrySelector, index: usize) -> u32 {
    //     let mut w = self.worlds.get(&self.world).unwrap();
    //     let mut entry = (*w).select_entries(map_selector).data[index];

    //     match entry.map_code {
    //         LoadMapEntryOpcode::DataPages => {
    //             // let map_world = map_entry.world;
    //             let page_number = entry.data.u().unwrap();
    //             if w.byte_swapped {
    //                 // ensure_virtual_address_range(entry.address, entry.count, false);
    //                 self.read_swapped_vlm_world_file_page(page_number);

    //                 let mut the_address = entry.addr;
    //                 w.current_q_number = 0;
    //                 println!("LoadMapDataPages @ {}, count {}", the_address, entry.count);

    //                 for _ in 0..entry.count {
    //                     virtual_memory_write(
    //                         the_address,
    //                         self.read_swapped_vlm_world_file_next_q()
    //                     );
    //                     the_address += 1;
    //                 }
    //             } else {
    //                 let file_offset = 8192 * (w.data_page_base + page_number * 4);
    //                 // let tag_offset = 8192 * (&w.vlm_data_page_base + page_number * 1);

    //                 self.map_world_load(entry.addr, entry.count, file_offset);
    //             }
    //         }
    //         LoadMapEntryOpcode::ConstantIncremented => {
    //             // ensure_virtual_address_range(entry.address, entry.count, false);
    //             virtual_memory_write_block_constant(
    //                 entry.addr,
    //                 &mut entry.data,
    //                 entry.count,
    //                 true
    //             );
    //         }
    //         LoadMapEntryOpcode::Constant => {
    //             // ensure_virtual_address_range(entry.address, entry.count, false);
    //             virtual_memory_write_block_constant(
    //                 entry.addr,
    //                 &mut entry.data,
    //                 entry.count,
    //                 false
    //             );
    //         }
    //         LoadMapEntryOpcode::Copy => {
    //             // ensure_virtual_address_range(entry.address, entry.count, false);
    //             let mut the_address = entry.addr;
    //             let mut the_source_address = unsafe { entry.data.u().unwrap() };

    //             for i in 0..entry.count {
    //                 virtual_memory_write(the_address, virtual_memory_read(the_source_address));
    //                 the_address += 1;
    //                 the_source_address += 1;
    //             }
    //         }
    //         _ => {
    //             self.close(true);
    //             panic_exit(
    //                 format!(
    //                     "Unknown load map opcode {} in world file {}",
    //                     entry.map_code,
    //                     w.pathname.display().to_string()
    //                 )
    //             );
    //         }
    //     }

    //     return entry.count;
    // }

    fn read_swapped_vlm_world_file_page(&self, page_number: u32) {
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

    fn read_swapped_vlm_world_file_q(&self, q_address: u32) -> MemoryCell {
        let w = self.worlds.get(&self.world).unwrap();

        // Comparing q_address < 0 is not necessary with type constraints
        if q_address >= VLMPAGE_SIZE_QS {
            // self.close(true);
            panic_exit(
                format!(
                    "Invalid word number {} for world file {}",
                    q_address,
                    w.pathname.display().to_string()
                )
            );
        }

        let datum: u32 = byte_swap_32(w.data_page[q_address as usize]);
        let tag: QTag = unsafe { ::std::mem::transmute(w.tags_page[q_address as usize]) };
        return MemoryCell::new_cdr_tag_u(CDR::Jump, tag, datum);
    }

    fn read_swapped_vlm_world_file_next_q(&self) -> MemoryCell {
        let w = self.worlds.get(&self.world).unwrap();

        // while w.current_q_number >= VLMPAGE_SIZE_QS {
        //     self.read_swapped_vlm_world_file_page(w.current_page_number + 1);
        //     w.current_q_number -= VLMPAGE_SIZE_QS;
        // }
        // let q = self.read_swapped_vlm_world_file_q(w.current_q_number);
        // w.current_q_number += 1;

        // return q;
        return MemoryCell::default();
    }

    //  align map entries to page boundaries.
    //  Canonicalize the load map entries for a VLM world:  Look for load map entries
    //  that don't start on a page boundary and convert them into a series of
    //  LoadMapConstant entries to load the data.  Thus, all data in the world file
    //  will be page-aligned to allow for direct mapping of the world load file into
    //  memory.  (Eventually, we may also merge adjacent load map rentries.)
    fn canonicalize_vlm_load_map_entries(&mut self) -> Set<LoadMapEntry> {
        let new_wired_map_entries: Set<LoadMapEntry> = Set::<LoadMapEntry>::new_ordered(&[], true);

        let world_id = self.world;

        let &mut world: &mut &World = match self.worlds.get_mut(&world_id) {
            Some(w) => w,

            None => {
                return new_wired_map_entries;
            }
        };

        let n_wired_entries = world.wired_map_entries.data.len() as u32;

        let mut page_number: u32 = 0;
        let mut i: u32 = 0;

        // Iterate through every single map entries
        while i < n_wired_entries {
            let current_map_entry = &world.wired_map_entries.data[i as usize];

            let (page_count, r) = current_map_entry.addr.div_rem(&VLMPAGE_SIZE_QS);

            if r == 0 {
                // If the address of the page is a multiple of VLMPAGE_SIZE_QS, i.e. Page Aligned,
                // assign the page number within the file
                let mut new_wired_map_entry = world.wired_map_entries.data[i as usize];
                new_wired_map_entry.data = MemoryCell::new_cdr_tag_u(
                    CDR::Jump,
                    QTag::Fixnum,
                    page_number
                ); // Tag 8
                new_wired_map_entries.insert(new_wired_map_entry);
                page_number = page_number + page_count;
                i += 1;
            } else {
                // Not Page Aligned:  Convert into a series of LoadMapConstant entries
                for j in 0..current_map_entry.count {
                    let mut new_wired_map_entry = world.wired_map_entries.data[(i + j) as usize];

                    new_wired_map_entry.addr = world.wired_map_entries.data[i as usize].addr + j;
                    new_wired_map_entry.map_code = LoadMapEntryOpcode::Constant;
                    new_wired_map_entry.count = 1;
                    new_wired_map_entry.data = virtual_memory_read(new_wired_map_entry.addr);
                    new_wired_map_entries.insert(new_wired_map_entry);
                }

                i += current_map_entry.count;
            }
        }

        // Compute size of header in VLM blocks to determine where the tags and data pages will start within the world file
        let n_qs = (new_wired_map_entries.data.len() as u32) * 3 + VLMWORLD_FILE_V2_FIRST_MAP_Q;
        let page_count = (n_qs - 1).div(IVORY_PAGE_SIZE_QS);
        let block_count = (page_count * IVORY_PAGE_SIZE_BYTES - 1).div(VLMPAGE_SIZE_QS);

        if block_count > VLMMAXIMUM_HEADER_BLOCKS {
            // FIXME: world.close(true);
            panic_exit(
                format!(
                    "Unable to store data map in space reserved for same in world file {}",
                    world.pathname.display().to_string()
                )
            );
        }

        // world.tags_page_base = block_count;
        // world.data_page_base = (world.tags_page_base + 1) * page_number;

        return new_wired_map_entries;
    }
}
