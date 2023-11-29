use std::{ cell::RefCell, rc::Rc };

use crate::common::memory_cell::MemoryCell;

////
///
/// Instruction cache line
///
#[derive(Debug)]
#[repr(C)]
pub struct InstructionCacheLine {
    pub pc: MemoryCell,
    pub next_pc: MemoryCell,
    pub code: u32,
    pub operand: u32,
    pub instruction: u64,
    pub next_cp: Rc<RefCell<InstructionCacheLine>>,
}

impl Default for InstructionCacheLine {
    fn default() -> Self {
        InstructionCacheLine {
            pc: MemoryCell::default(),
            next_pc: MemoryCell::default(),
            code: 0,
            operand: 0,
            instruction: 0,
            next_cp: Rc::default(),
        }
    }
}

impl Clone for InstructionCacheLine {
    fn clone(&self) -> Self {
        InstructionCacheLine {
            pc: self.pc.clone(),
            next_pc: self.next_pc.clone(),
            code: self.code,
            operand: self.operand,
            instruction: self.instruction,
            next_cp: Rc::new(RefCell::new(self.clone())),
        }
    }
}

pub const INSTRUCTION_CACHE_SIZE: u32 = 0x800;
pub const INSTRUCTION_CACHE_LINE_SIZE: u32 = 0x40;
