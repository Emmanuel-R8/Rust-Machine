use std::borrow::{Borrow, BorrowMut};
use std::cell::RefCell;
use std::cmp::min;
use std::collections::HashMap;
use std::fs::{read_dir, DirEntry, File};
use std::io::Read;
use std::mem::size_of;
use std::path::{Path, PathBuf};
use std::rc::Rc;

use memmap::Mmap;
use sets::Set;
use uuid::Uuid;

use crate::common::constants::{
    LoadFileFormat, QTag, VMAttribute, CDR, IVORY_PAGE_SIZE_BYTES, MEMORY_ADDRESS_PAGE_SHIFT,
    VMATTRIBUTE_CREATED_DEFAULT, VMATTRIBUTE_EMPTY, VMATTRIBUTE_EXISTS, MEMORY_PAGE_SIZE, MEMORYWAD_SIZE, VLMPAGE_SIZE_QS,
};
use crate::common::types::{QCDRTagData, QImmediate, QWord};
use crate::hardware::cpu::{
    read_control_argument_size, read_control_caller_frame_size, write_control_argument_size,
    write_control_caller_frame_size, CPU,
};
use crate::hardware::memory::{lisp_obj_data, default_attributes, compute_protection, memory_wad_offset, memory_page_offset, make_lisp_obj_u};
use crate::utils::{pack_8_to_32, byte_swap_32};
use crate::world::world::{
    clone, merge_a_map, panic_exit, read_ivory_world_file_Q, read_ivory_world_file_next_Q,
    read_load_map, LoadMapEntry, MapEntrySelector, World, LoadMapEntryOpcode, virtual_memory_write, virtual_memory_write_block_constant, virtual_memory_read,
};

#[derive()]
pub struct GlobalContext {
    pub cpu: CPU,
    pub mem: [QWord; 1 << 31], /* 2^32 bytes of tags + data */
    pub attribute_table: [VMAttribute; 1 << (32 - MEMORY_ADDRESS_PAGE_SHIFT)],

    pub world: Uuid,
    pub worlds: HashMap<Uuid, World>,

    pub scanning_dir: PathBuf,

    pub unmapped_world_words: u32,
    pub mapped_world_words: u32,
    pub file_map_entries: u32,
    pub swap_map_entries: u32,
}

impl Default for GlobalContext {
    fn default() -> Self {
        return GlobalContext {
            cpu: CPU::default(),
            mem: [QWord::default(); 1 << 31],
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

impl GlobalContext {
    pub fn new() -> Self {
        return GlobalContext {
            cpu: CPU::default(),
            mem: [QWord::default(); 1 << 31],
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

    pub fn write_at(&mut self, addr: QWord, val: QWord) {
        self.mem[unsafe { addr.parts.data.a } as usize] = val;
    }

    pub fn inc_and_write_at(&mut self, addr: QWord, val: QWord) -> QWord {
        let a = addr.inc();
        self.write_at(a, val);

        return a;
    }

    pub fn write_at_and_inc(&mut self, addr: QWord, val: QWord) -> QWord {
        self.write_at(addr, val);
        return addr.inc();
    }

    pub fn read_at(&mut self, addr: QWord) -> QWord {
        return self.mem[unsafe { addr.parts.data.a } as usize];
    }

    pub fn inc_and_read_at(&mut self, addr: QWord) -> (QWord, QWord) {
        let a = addr.inc();
        return (self.read_at(a), a);
    }

    pub fn read_at_and_inc(&mut self, addr: QWord) -> (QWord, QWord) {
        let a = addr.inc();
        return (self.read_at(addr), a);
    }

    /// Push one empty frame
    /// See IMAS p 242 for frame format
    pub fn push_one_fake_frame(&mut self) {
        // Push continuation
        let mut q = self.cpu.continuation.clone();
        q.parts.cdr = CDR::Jump;

        self.cpu.sp = self.inc_and_write_at(self.cpu.sp, q);

        // The new stack pointer is stored as the new frame pointer (beginning of the frame)
        self.cpu.fp = self.cpu.sp;

        // Push the control register
        let mut q = QWord::default();
        q.parts.cdr = CDR::Jump;
        q.parts.tag = QTag::Fixnum;
        q.parts.data.u = unsafe { self.cpu.control.parts.data.u };
        self.cpu.sp = self.inc_and_write_at(self.cpu.sp, q);

        // Create a new control register
        self.cpu.control = QWord {
            parts: QCDRTagData {
                cdr: CDR::Jump,
                tag: QTag::Fixnum,
                data: QImmediate { u: 0 },
            },
        };
        write_control_argument_size(&mut self.cpu.control, 2);
        write_control_caller_frame_size(&mut self.cpu.control, unsafe {
            (self.cpu.sp - self.cpu.fp).parts.data.u
        });

        self.cpu.continuation = self.cpu.pc;
    }

    /// Pop one empty frame
    pub fn pop_one_fake_frame(&mut self) {
        // Set the stack pointer at the address of the current frame
        self.cpu.sp = self.cpu.fp;

        // Determine the next frame pointer by decreasing by the frmae size
        unsafe { self.cpu.fp.parts.data.a -= read_control_caller_frame_size(self.cpu.control) };

        // Restore the PC using the stored continuation
        self.cpu.pc = self.cpu.continuation;

        // Temporary copy of FP
        (self.cpu.continuation, self.cpu.fp) = self.read_at_and_inc(self.cpu.fp);
        self.cpu.control.parts.data.u = unsafe { self.read_at(self.cpu.fp).parts.data.u };

        self.cpu.lp = self.cpu.fp.clone();
        unsafe { self.cpu.lp.parts.data.a += read_control_argument_size(self.cpu.control) };
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
                pw = w.parent_world;
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
        new_merged_wired_entries =
            merge_a_map(w, &new_merged_wired_entries, &w.merged_wired_map_entries);
        new_unmerged_wired_entries =
            merge_a_map(&w, &new_unmerged_wired_entries, &w.unwired_map_entries);

        return (new_merged_wired_entries, new_unmerged_wired_entries);
    }

    pub fn merge_load_maps(
        &mut self,
        world_search_path: String,
    ) -> (Set<LoadMapEntry>, Set<LoadMapEntry>) {
        let w = self.worlds.get(&self.world).unwrap();
        if w.generation == 0 {
            return (
                Set::<LoadMapEntry>::new_ordered(&[], true),
                Set::<LoadMapEntry>::new_ordered(&[], true),
            );
        }

        let mut new_merged_wired_entries = Set::<LoadMapEntry>::new_ordered(&[], true);
        let mut new_unmerged_wired_entries = Set::<LoadMapEntry>::new_ordered(&[], true);

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

    fn find_parent_worlds(&mut self, world_search_path: String) {
        // Remove all worlds apart from current one
        let current_world = self.world;
        self.worlds.retain(|&u, _| u == current_world);

        let w = self.worlds.get(&current_world).unwrap();
        let mut dir_components = w.pathname.ancestors();
        let full_path = dir_components.next();
        let base_directory = dir_components.next();

        match base_directory {
            None => {
                panic_exit(format!(
                    "Unable to determine pathname of directory containing world file {}",
                    w.pathname.display()
                ));
            }
            Some(base_dir) => {
                self.scan_one_directory(base_dir);
            }
        }

        while w.generation > 0 {
            for (w_uuid, mut w_world) in &self.worlds {
                if w_world.generation == w_world.generation - 1
                    && w_world.timestamp_1 == w_world.parent_timestamp_1
                    && w_world.timestamp_2 == w_world.parent_timestamp_2
                {
                    w_world.parent_world = w_uuid.clone();
                    break;
                }
            }

            if w.parent_world.is_nil() {
                panic_exit(format!(
                    "Unable to find parent of world file {}",
                    w.pathname.display()
                ));
            } else {
                self.world = w.parent_world.clone();
            }
        }
        self.close_extra_worlds();
    }

    fn scan_one_directory(&self, dir: &Path) -> Vec<DirEntry> {
        // scan the directory specified in the global context and only keeps the world files.
        let dir_reader = read_dir(&self.scanning_dir);
        let mut results: Vec<DirEntry> = vec![];

        match dir_reader {
            Ok(dr) => {
                // dr.map(|elt| elt.into_ok()).filter(|d| world_p(dr, ctx));

                for r in dr {
                    match r {
                        Ok(r_) => {
                            results.push(r_);
                        }
                        Err(_) => {}
                    }
                }
            }
            Err(_) => {}
        }

        if results.len() == 0 {
            panic_exit(format!(
                "Unable to search directory {} to find parents of world file.",
                dir.display()
            ));
        }

        return results;
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

    fn open_world_file(&mut self, puntOnErrors: bool) -> bool {
        let mut page_bases: QWord = QWord::default();
        let mut wired_count_Q: u32 = 0;
        let mut unwired_count_Q: u32 = 0;
        let mut pages_base_Q: u32 = 0;
        let mut first_sysout_Q: u32 = 0;
        let mut first_map_Q: u32 = 0;

        let mut w = World::new();

        let path = &w.pathname;
        w.fd = Some(File::open(path).expect("Could not open file"));

        let mut cookie: [u8; 4] = [0 as u8; 4];
        if w.fd.unwrap().read_exact(&mut cookie).is_err() && puntOnErrors {
            panic_exit(format!("Reading world file {} cookie.", path.display()));
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
                    panic_exit(format!(
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
                    panic_exit(format!(
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
        w.wired_map_entries = read_load_map(&mut w, MapEntrySelector::Wired);
        w.unwired_map_entries = read_load_map(&mut w, MapEntrySelector::Unwired);

        let key = self.world;
        self.worlds.insert(key, w);

        return true;
    }

    ///
    /// Load a map in to the GlobalContext world structure
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
            while memory_wad_offset(vma) != 0 || self.vma_created_p(vma)  {
                words = min(MEMORY_PAGE_SIZE - memory_page_offset(vma), remaining);

                // ensure_virtual_address(vma);
                data_count = words * size_of::<u32>() as u32;
                tag_count = words + size_of::<QTag>() as u32;

                // Adjust the protection to catch modifications to world pages
                self.vma_set_created(vma) ;

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
                while words < limit &&  !self.wad_created(vma)  {
                    let wad_limit: u32 = words + MEMORYWAD_SIZE;
                    // TODO: Check should not be sweeping through all addresses.
                    while words < wad_limit {
                        self.vma_set_attr(vma + words, default_attributes(false, true)) ;
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

    pub fn VLM_load_map_data(&mut self, map_selector: MapEntrySelector, index: usize) -> u32 {
        let mut w =  self.worlds.get(&self.world).unwrap() ;
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
                panic_exit(format!(
                    "Unknown load map opcode {} in world file {}",
                    entry.map_code,
                    w.pathname.display().to_string()
                ))
            }
        }

        return entry.count;
    }
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
            panic_exit(format!(
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
