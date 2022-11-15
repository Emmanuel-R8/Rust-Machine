use std::borrow::{Borrow, BorrowMut};
use std::cell::RefCell;
use std::collections::HashMap;
use std::fs::{read_dir, DirEntry};
use std::path::{Path, PathBuf};
use std::rc::Rc;

use sets::Set;
use uuid::Uuid;

use crate::common::constants::{
    QTag, VMAttribute, CDR, MEMORY_ADDRESS_PAGE_SHIFT, VMATTRIBUTE_CREATED_DEFAULT,
    VMATTRIBUTE_EMPTY, VMATTRIBUTE_EXISTS,
};
use crate::common::types::{QCDRTagData, QImmediate, QWord};
use crate::hardware::cpu::{
    read_control_argument_size, read_control_caller_frame_size, write_control_argument_size,
    write_control_caller_frame_size, CPU,
};
use crate::world::world::{clone_map_entries, merge_a_map, vpunt, MapEntrySelector, World, LoadMapEntry};

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
            _ => {},
        }

        if close_parent {
            self.worlds.remove(&pw);
        }
    }

    pub fn merge_parent_load_map(&mut self) {
        let mut w = unsafe { self.worlds.get(&self.world).unwrap() };

        // If at the top of the topmost parent (no generation above)
        if w.generation == 0 {
            w.merged_wired_map_entries = clone_map_entries(&w.wired_map_entries);
            w.merged_unwired_map_entries = clone_map_entries(&w.unwired_map_entries);
        } else {
            if !w.parent_world.is_nil() {
                self.merge_parent_load_map();

                w.merged_wired_map_entries =
                    merge_a_map(&w, MapEntrySelector::Wired, MapEntrySelector::Wired)
                        .unwrap_or(Set::<LoadMapEntry>::new_ordered(&[], true));

                w.merged_unwired_map_entries =
                    merge_a_map(&w, MapEntrySelector::Unwired, MapEntrySelector::Unwired)
                        .unwrap_or(Set::<LoadMapEntry>::new_ordered(&[], true));
            }
        }
    }

    pub fn merge_load_maps(&mut self, world_search_path: String) {
        let mut w: &World = unsafe { self.worlds.get(&self .world).unwrap() };

        if w.generation == 0 {
            w.merged_wired_map_entries = clone_map_entries(&w.wired_map_entries);
            w.merged_unwired_map_entries = clone_map_entries(&w.unwired_map_entries);
        } else {
            self.find_parent_worlds(world_search_path);
            self.merge_parent_load_map();
        }
    }

    fn find_parent_worlds(&mut self, mut world_search_path: String) {
        // Remove all worlds apart from current one
        let current_world = self.world;
        self.worlds.retain(|&u, _|  u == current_world);


        let w = self.worlds.get(&current_world).unwrap();
        let mut dir_components = w.pathname.ancestors();
        let full_path = dir_components.next();
        let base_directory = dir_components.next();

        match base_directory {
            None => {
                vpunt(format!(
                    "Unable to determine pathname of directory containing world file {}",
                    w.pathname.display()
                ));
            }
            Some(base_dir) => {
                self.scan_one_directory(base_dir);
            }
        }

        while w.generation > 0 {
            for (w_uuid, w_world) in &mut self.worlds {
                if w_world.generation == w_world.generation - 1
                    && w_world.timestamp_1 == w_world.parent_timestamp_1
                    && w_world.timestamp_2 == w_world.parent_timestamp_2
                {
                    w_world.parent_world = w_uuid.clone();
                    break;
                }
            }

            if w.parent_world.is_nil() {
                vpunt(format!(
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
            vpunt(format!(
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
}
