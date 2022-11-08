use std::cell::RefCell;
use std::fs::{read_dir, DirEntry};
use std::path::{Path, PathBuf};
use std::ptr::write;
use std::rc::Rc;

use crate::common::constants::{
    QTag, VMAttribute, CDR, MEMORY_ADDRESS_PAGE_SHIFT, VMATTRIBUTE_CREATED_DEFAULT,
    VMATTRIBUTE_EMPTY, VMATTRIBUTE_EXISTS,
};
use crate::common::types::QWord;
use crate::hardware::cpu::{
    read_control_argument_size, read_control_caller_frame_size, write_control_argument_size,
    write_control_caller_frame_size, CPU,
};
use crate::world::world::{clone_map_entries, merge_a_map, vpunt, World};

#[derive(Debug)]
pub struct GlobalContext<'a> {
    pub cpu: CPU,
    pub mem: [QWord; 1 << 31], /* 2^32 bytes of tags + data */
    pub attribute_table: [VMAttribute; 1 << (32 - MEMORY_ADDRESS_PAGE_SHIFT)],

    pub world: Rc<RefCell<World<'a>>>,
    pub worlds: Vec<World<'a>>,
    pub total_worlds: u32,
    pub scanning_dir: PathBuf,

    pub unmapped_world_words: u32,
    pub mapped_world_words: u32,
    pub file_map_entries: u32,
    pub swap_map_entries: u32,
}

impl Default for GlobalContext<'static> {
    fn default() -> Self {
        GlobalContext {
            cpu: CPU::default(),
            mem: [QWord::default(); 1 << 31],
            attribute_table: [VMATTRIBUTE_EMPTY; 1 << (32 - MEMORY_ADDRESS_PAGE_SHIFT)],

            world: Rc::new(RefCell::new(World::default())),
            worlds: vec![],
            total_worlds: 0,
            // n_worlds: 0,
            scanning_dir: PathBuf::from(""),

            unmapped_world_words: 0,
            mapped_world_words: 0,
            file_map_entries: 0,
            swap_map_entries: 0,
        }
    }
}

impl<'a> GlobalContext<'a> {
    pub fn write_at(&mut self, addr: QWord, val: QWord) {
        self.mem[addr.parts.data.a as usize] = val;
    }

    pub fn inc_and_write_at(&mut self, &mut addr: QWord, val: QWord) {
        addr.parts.data.a += 1;
        self.write_at(addr, val);
    }

    pub fn write_at_and_inc(&mut self, &mut addr: QWord, val: QWord) {
        self.write_at(addr, val);
        addr.parts.data.a += 1;
    }

    pub fn read_at(&mut self, addr: QWord) -> QWord {
        return self.mem[addr.parts.data.a as usize];
    }

    pub fn inc_and_read_at(&mut self, &mut addr: QWord) -> QWord {
        addr.parts.data.a += 1;
        return self.read_at(addr);
    }

    pub fn read_at_and_inc(&mut self, &mut addr: QWord) -> QWord {
        let a = addr;
        addr.parts.data.a += 1;
        return self.read_at(a);
    }

    /// Push one empty frame
    /// See IMAS p 242 for frame format
    pub fn push_one_fake_frame(&mut self) {
        let mut q: QWord;

        // Push continuation
        q = self.cpu.continuation;
        q.parts.cdr = CDR::Jump;
        self.inc_and_write_at(&self.cpu.sp, q);

        // This new stack pointer is stored as the new frame pointer
        self.cpu.fp = self.cpu.sp;

        // Push control register
        q = self.cpu.control;
        q.parts.cdr = CDR::Jump;
        q.parts.tag = QTag::Fixnum;
        self.inc_and_write_at(&self.cpu.sp, q);

        // Create a new control register
        self.cpu.control = 0;
        write_control_argument_size(&mut self.cpu.control, 2);
        write_control_caller_frame_size(&mut self.cpu.control, q - self.cpu.fp);

        self.cpu.continuation = self.cpu.pc;
    }

    /// Pop one empty frame
    pub fn pop_one_fake_frame(&mut self) {
        // Set the stack pointer at the address of the current frame
        self.cpu.sp = self.cpu.fp;

        // Determine the next frame pointer by decreasing by the frmae size
        self.cpu.fp -= read_control_caller_frame_size(self.cpu.control);

        // Restore the PC using the stored continuation
        self.cpu.pc = self.cpu.continuation;

        // Temporary copy of FP
        let &mut fp = self.cpu.fp;

        self.cpu.continuation = self.read_at_and_inc(fp);
        self.cpu.control.parts.data.u = self.read_at(fp).parts.data.u;

        self.cpu.lp = self.cpu.fp.clone();
        self.cpu.lp.parts.data.a += read_control_argument_size(self.cpu.control);
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
        let mut w = self.world.borrow_mut();

        w.fd = None; // Drop the file descriptor and close it automatically.
        w.vlm_data_page = vec![];
        w.vlm_tags_page = vec![];
        w.ivory_data_page = vec![];
        w.merged_wired_map_entries = vec![];
        w.wired_map_entries = vec![];
        w.merged_unwired_map_entries = vec![];
        w.unwired_map_entries = vec![];

        // if close_parent && self.parent_world.is_some() {
        //     self.parent_world.unwrap().close(true);
        //     self.parent_world = None;
        // }
    }

    pub fn merge_parent_load_map(&mut self) {
        let mut world = self.world.as_ref().borrow_mut();

        // If at the top of the topmost parent (no generation above)
        if world.generation == 0 {
            world.merged_wired_map_entries = clone_map_entries(&world.wired_map_entries);
            world.merged_unwired_map_entries = clone_map_entries(&world.unwired_map_entries);
        } else {
            match &world.parent_world {
                Some(pw) => {
                    &self.merge_parent_load_map();

                    world.merged_wired_map_entries = merge_a_map(
                        self.world,
                        world.wired_map_entries,
                        pw.merged_wired_map_entries,
                    )
                    .unwrap_or(vec![]);

                    world.merged_unwired_map_entries = merge_a_map(
                        self.world,
                        world.unwired_map_entries,
                        pw.merged_unwired_map_entries,
                    )
                    .unwrap_or(vec![]);
                }
                None => {}
            }
        }
    }

    pub fn merge_load_maps(&mut self, world_search_path: String) {
        let world = self.world.borrow_mut();

        if world.generation == 0 {
            world.merged_wired_map_entries = clone_map_entries(&world.wired_map_entries);
            world.merged_unwired_map_entries = clone_map_entries(&world.unwired_map_entries);
        } else {
            self.find_parent_worlds(world_search_path);
            self.merge_parent_load_map();
        }
    }

    fn find_parent_worlds(&mut self, mut world_search_path: String) {
        let mut failing_world_pathname: String = String::from("");
        let mut slash_position: String = String::from("");
        let mut colon_position: String = String::from("");

        let mut world = &self.world;
        // ctx.n_worlds = 0;
        self.total_worlds = 0;
        self.worlds = vec![];

        match world.get_mut().pathname {
            Some(path) => {
                let dir_components = path.ancestors();
                let full_path = dir_components.next();
                let base_directory = dir_components.next();

                match base_directory {
                    None => {
                        world.get_mut().close(true);
                        vpunt(format!(
                            "Unable to determine pathname of directory containing world file {}",
                            path.display()
                        ));
                    }
                    Some(base_dir) => {
                        self.scan_one_directory(base_dir);
                    }
                }

                while world.get_mut().generation != 0 {
                    for w in self.worlds {
                        if w.generation == world.get_mut().generation - 1
                            && w.timestamp_1 == world.get_mut().parent_timestamp_1
                            && w.timestamp_2 == world.get_mut().parent_timestamp_2
                        {
                            world.get_mut().parent_world = Some(&mut w);
                            break;
                        }
                    }

                    match world.get_mut().parent_world {
                        None => {
                            self.close_extra_worlds();
                            world.get_mut().close(true);
                            vpunt(format!(
                                "Unable to find parent of world file {}",
                                path.display()
                            ));
                        }
                        Some(pw) => {
                            world = &Rc::new(RefCell::new(*pw));
                        }
                    }
                }
                self.close_extra_worlds();
            }
            None => {}
        }
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
}
