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
    pub next_cp: Option<Box<InstructionCacheLine>>,
}

impl Default for InstructionCacheLine {
    fn default() -> Self {
        InstructionCacheLine {
            pc: MemoryCell::default(),
            next_pc: MemoryCell::default(),
            code: 0,
            operand: 0,
            instruction: 0,
            next_cp: None,
        }
    }
}

impl Clone for InstructionCacheLine {
    fn clone(&self) -> Self {
        let next_pc = match self.next_pc.is_nil() {
            | false => self.next_pc.clone(),
            | true => MemoryCell::default(),
        };

        let next_cp = match self.next_cp {
            | None => None,
            | _ => self.next_cp.clone(),
        };

        InstructionCacheLine {
            pc: self.pc.clone(),
            next_pc: next_pc,
            code: self.code,
            operand: self.operand,
            instruction: self.instruction,
            next_cp: next_cp,
        }
    }
}

pub const INSTRUCTION_CACHE_SIZE: u32 = 0x800;
pub const INSTRUCTION_CACHE_LINE_SIZE: u32 = 0x40;
