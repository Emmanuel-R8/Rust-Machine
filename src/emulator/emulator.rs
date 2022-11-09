use std::borrow::{Borrow, BorrowMut};
use std::cell::RefCell;
use std::fs::{read_dir, DirEntry};
use std::path::{Path, PathBuf};
use std::rc::Rc;

use crate::common::constants::{
    QTag, VMAttribute, CDR, MEMORY_ADDRESS_PAGE_SHIFT, VMATTRIBUTE_CREATED_DEFAULT,
    VMATTRIBUTE_EMPTY, VMATTRIBUTE_EXISTS,
};
use crate::common::types::{QCDRTagData, QImmediate, QWord};
use crate::hardware::cpu::{
    read_control_argument_size, read_control_caller_frame_size, write_control_argument_size,
    write_control_caller_frame_size, CPU,
};
use crate::world::world::{clone_map_entries, merge_a_map, vpunt, MapEntrySelector, World};

#[derive(Debug)]
pub struct GlobalContext<'a> {
    pub cpu: CPU,
    pub mem: [QWord; 1 << 31], /* 2^32 bytes of tags + data */
    pub attribute_table: [VMAttribute; 1 << (32 - MEMORY_ADDRESS_PAGE_SHIFT)],

    pub world: &'a mut World<'a>,
    pub worlds: Vec<&'a mut World<'a>>,
    pub total_worlds: u32,
    pub scanning_dir: PathBuf,

    pub unmapped_world_words: u32,
    pub mapped_world_words: u32,
    pub file_map_entries: u32,
    pub swap_map_entries: u32,
}

impl<'a> GlobalContext<'a> {
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

    // Close a world file
    pub fn close(&mut self, close_parent: bool) {
        // let mut w = self.world.borrow_mut();

        self.world.fd = None; // Drop the file descriptor and close it automatically.
        self.world.vlm_data_page = vec![];
        self.world.vlm_tags_page = vec![];
        self.world.ivory_data_page = vec![];
        self.world.merged_wired_map_entries = vec![];
        self.world.wired_map_entries = vec![];
        self.world.merged_unwired_map_entries = vec![];
        self.world.unwired_map_entries = vec![];

        // if close_parent && self.parent_world.is_some() {
        //     self.parent_world.unwrap().close(true);
        //     self.parent_world = None;
        // }
    }

    pub fn merge_parent_load_map(&mut self) {
        // If at the top of the topmost parent (no generation above)
        if self.world.generation == 0 {
            self.world.merged_wired_map_entries = clone_map_entries(&self.world.wired_map_entries);
            self.world.merged_unwired_map_entries =
                clone_map_entries(&self.world.unwired_map_entries);
        } else {
            match &self.world.parent_world {
                Some(pw) => {
                    &self.merge_parent_load_map();

                    self.world.merged_wired_map_entries = merge_a_map(
                        self.world,
                        MapEntrySelector::Wired,
                        &mut pw.get_mut().merged_wired_map_entries,
                    )
                    .unwrap_or(vec![]);

                    self.world.merged_unwired_map_entries = merge_a_map(
                        self.world,
                        MapEntrySelector::Unwired,
                        &mut pw.get_mut().merged_unwired_map_entries,
                    )
                    .unwrap_or(vec![]);
                }
                None => {}
            }
        }
    }

    pub fn merge_load_maps(&mut self, world_search_path: String) {
        if self.world.generation == 0 {
            self.world.merged_wired_map_entries = clone_map_entries(&self.world.wired_map_entries);
            self.world.merged_unwired_map_entries =
                clone_map_entries(&self.world.unwired_map_entries);
        } else {
            self.find_parent_worlds(world_search_path);
            self.merge_parent_load_map();
        }
    }

    fn find_parent_worlds(&mut self, mut world_search_path: String) {
        let mut failing_world_pathname: String = String::from("");
        let mut slash_position: String = String::from("");
        let mut colon_position: String = String::from("");

        self.total_worlds = 0;
        self.worlds = vec![];

        let mut dir_components = self.world.pathname.ancestors();
        let full_path = dir_components.next();
        let base_directory = dir_components.next();

        match base_directory {
            None => {
                self.world.close(true);
                vpunt(format!(
                    "Unable to determine pathname of directory containing world file {}",
                    self.world.pathname.display()
                ));
            }
            Some(base_dir) => {
                self.scan_one_directory(base_dir);
            }
        }

        while self.world.generation > 0 {
            for w in &self.worlds {
                if w.generation == self.world.generation - 1
                    && w.timestamp_1 == self.world.parent_timestamp_1
                    && w.timestamp_2 == self.world.parent_timestamp_2
                {
                    self.world.parent_world = Some(Rc::new(RefCell::new(w)));
                    break;
                }
            }

            match self.world.parent_world {
                None => {
                    self.close_extra_worlds();
                    self.world.close(true);
                    vpunt(format!(
                        "Unable to find parent of world file {}",
                        self.world.pathname.display()
                    ));
                }
                Some(pw) => {
                    self.world = &mut pw.get_mut();
                }
            }
        }
        self.close_extra_worlds();
    }

    fn scan_one_directory(&mut self, dir: &Path) -> Vec<DirEntry> {
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
            self.close_extra_worlds();
            vpunt(format!(
                "Unable to search directory {} to find parents of world file.",
                dir.display()
            ));
        }

        return results;
    }

    pub fn close_extra_worlds(&mut self) {
        for w in &mut self.worlds {
            w.close(true);
        }

        self.worlds = Vec::new();
        self.total_worlds = 0;
    }

    pub fn initialise(&mut self) -> &GlobalContext {
        let gc = &mut GlobalContext {
            cpu: CPU::default(),
            mem: [QWord::default(); 1 << 31],
            attribute_table: [VMATTRIBUTE_EMPTY; 1 << (32 - MEMORY_ADDRESS_PAGE_SHIFT)],

            world: &mut World::default(),
            worlds: vec![],
            total_worlds: 0,
            // n_worlds: 0,
            scanning_dir: PathBuf::from(""),

            unmapped_world_words: 0,
            mapped_world_words: 0,
            file_map_entries: 0,
            swap_map_entries: 0,
        };

        // Push initial frames:  These are a lie, they will be popped when you
        // start, so that the "continuation" at 4 becomes the PC.  The
        // continuation and control for the running frame are NIL and 0,
        // respectively, thus returning from that frame will not adjust the FP
        // and the sequencer will know to halt on seeing NIL as a PC.
        gc.push_one_fake_frame();
        gc.push_one_fake_frame();

        // EnsureVirtualAddressRange(0xf8000100, 0xf00, false);
        // EnsureVirtualAddressRange(0xf8062000, 0x9e000, false);

        return gc;
    }
}
