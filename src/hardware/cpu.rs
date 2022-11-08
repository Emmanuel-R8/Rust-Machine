use std::cell::RefCell;
use std::rc::Rc;

use crate::common::constants::{
    QTag, TrapMode, CDR, IVORY_PAGE_SIZE_QS, IVORY_STACK_CACHE_SIZE, MEMORY_STACK_CACHE_BASE,
};
use crate::common::types::{
    Bar, InstructionCacheLine, QCDRTagData, QImmediate, QWord, INSTRUCTION_CACHE_SIZE,
};
use crate::utils::{dpb, ldb};

const CONTROL_ARGUMENT_SIZE: u32 = 0o377;
const CONTROL_EXTRA_ARGUMENT: u32 = 0o400;
const CONTROL_CALLER_FRAME_SIZE: u32 = 0o377_000;
const CONTROL_APPLY: u32 = 0o400_000;
const CONTROL_VALUE_DISPOSITION: u32 = 0o3_000_000;
const CONTROL_CALL_STARTED: u32 = 0o20_000_000;
const CONTROL_CLEANUP_BITS: u32 = 0o700_000_000;
const CONTROL_TRACE_BITS: u32 = 0o7_000_000_000;

pub fn read_control_argument_size(c: u32) -> u32 {
    return ldb(8, 0, c);
}
pub fn read_control_extra_argument(c: u32) -> u32 {
    return ldb(1, 8, c);
}
pub fn read_control_caller_frame_size(c: u32) -> u32 {
    return ldb(8, 9, c);
}
pub fn read_control_apply(c: u32) -> u32 {
    return ldb(1, 17, c);
}
pub fn read_control_value_disposition(c: u32) -> u32 {
    return ldb(2, 18, c);
}
pub fn read_control_cleanup_bits(c: u32) -> u32 {
    return ldb(3, 24, c);
}
pub fn read_control_cleanup_catch(c: u32) -> u32 {
    return ldb(1, 26, c);
}
pub fn read_control_cleanup_bindings(c: u32) -> u32 {
    return ldb(1, 25, c);
}
pub fn read_control_trap_on_exit(c: u32) -> u32 {
    return ldb(1, 24, c);
}
pub fn read_control_trap_mode(c: u32) -> u32 {
    return ldb(2, 30, c);
}
pub fn read_control_call_started(c: u32) -> u32 {
    return ldb(1, 22, c);
}
pub fn read_control_cleanup_in_progress(c: u32) -> u32 {
    return ldb(1, 23, c);
}
pub fn read_control_instruction_trace(c: u32) -> u32 {
    return ldb(1, 29, c);
}
pub fn read_control_call_trace(c: u32) -> u32 {
    return ldb(1, 28, c);
}
pub fn read_control_trace_pending(c: u32) -> u32 {
    return ldb(1, 27, c);
}

pub fn write_control_argument_size(c: &mut u32, x: u32) {
    *c = dpb(x, 8, 0, c)
}
pub fn write_control_extra_argument(c: &mut u32, x: u32) {
    *c = dpb(x, 1, 8, c)
}
pub fn write_control_caller_frame_size(c: &mut u32, x: u32) {
    *c = dpb(x, 8, 9, c)
}
pub fn write_control_apply(c: &mut u32, x: u32) {
    *c = dpb(x, 1, 17, c)
}
pub fn write_control_value_disposition(c: &mut u32, x: u32) {
    *c = dpb(x, 2, 18, c)
}
pub fn write_control_cleanup_bits(c: &mut u32, x: u32) {
    *c = dpb(x, 3, 24, c)
}
pub fn write_control_cleanup_catch(c: &mut u32, x: u32) {
    *c = dpb(x, 1, 26, c)
}
pub fn write_control_cleanup_bindings(c: &mut u32, x: u32) {
    *c = dpb(x, 1, 25, c)
}
pub fn write_control_trap_on_exit(c: &mut u32, x: u32) {
    *c = dpb(x, 1, 24, c)
}
pub fn write_control_trap_mode(c: &mut u32, x: u32) {
    *c = dpb(x, 2, 30, c)
}
pub fn write_control_call_started(c: &mut u32, x: u32) {
    *c = dpb(x, 1, 22, c)
}
pub fn write_control_cleanup_in_progress(c: &mut u32, x: u32) {
    *c = dpb(x, 1, 23, c)
}
pub fn write_control_instruction_trace(c: &mut u32, x: u32) {
    *c = dpb(x, 1, 29, c)
}
pub fn write_control_call_trace(c: &mut u32, x: u32) {
    *c = dpb(x, 1, 28, c)
}
pub fn write_control_trace_pending(c: &mut u32, x: u32) {
    *c = dpb(x, 1, 27, c)
}

#[derive(Debug, Clone)]
pub struct CPU {
    pub sp: QWord,
    pub restarts_p: QWord,
    pub fp: QWord,
    pub lp: QWord,
    pub pc: QWord,
    pub continuation: QWord,

    pub allocated_caches: bool,
    pub instruction_cache: [InstructionCacheLine; INSTRUCTION_CACHE_SIZE as usize],
    pub stack_cache: [QWord; IVORY_PAGE_SIZE_QS * IVORY_STACK_CACHE_SIZE],
    pub stack_cache_limit: QWord,

    pub bar: [Bar; 4],
    pub list_cache_area: QWord,
    pub list_cache_address: QWord,
    pub structure_cache_area: QWord,
    pub structure_cache_address: QWord,
    pub catch_block_pointer: QWord,
    pub control: u32,
    pub stack_cache_base: u32,
    pub array_event_count: u32,
    pub list_cache_length: u32,
    pub structure_cache_length: u32,
    pub binding_stack_pointer: u32,
    pub binding_stack_limit: u32,
    pub deep_bound_p: bool,
    pub preempt_register: u32,
    pub alu_and_rotate_control: u32,
    pub alu_op: u32,
    pub byte_size: u32,
    pub byte_rotate: u32,
    pub rotate_latch: u32,
    pub alu_overflow: bool,
    pub alu_borrow: bool,
    pub alu_less_than: bool,
    pub ephemeral_oldspace_register: u32,
    pub zone_oldspace_register: u32,
    pub control_stack_limit: u32,
    pub control_stack_extra_limit: u32,
    pub dynamic_binding_cache_base: u32,
    pub dynamic_binding_cache_mask: u32,
    pub fep_mode_trap_vector_address: u32,
    pub mapping_table_cachee: u32,
    pub mapping_table_length: u32,
    pub running: bool,
    pub suspend: bool,
    pub instruction_count: u32,
}

pub const PROGRAM_COUNTER_INIT: u32 = 0x07FE_0040;

impl Default for CPU {
    fn default() -> Self {
        Self {
            sp: QWord::default(),
            restarts_p: QWord::default(),
            fp: QWord::default(),
            lp: QWord::default(),
            pc: QWord {
                parts: QCDRTagData {
                    cdr: CDR::Normal,
                    tag: QTag::EvenPC, // See IMAS p. 59
                    data: QImmediate {
                        a: PROGRAM_COUNTER_INIT,
                    },
                },
            },
            continuation: QWord::default(),
            instruction_cache: [InstructionCacheLine::default(); INSTRUCTION_CACHE_SIZE as usize],
            stack_cache: [QWord { whole: 0 }; IVORY_PAGE_SIZE_QS * IVORY_STACK_CACHE_SIZE],
            stack_cache_limit: QWord::default(),
            allocated_caches: false,

            bar: [Bar::default(); 4],
            list_cache_area: QWord::default(),
            list_cache_address: QWord::default(),
            structure_cache_area: QWord::default(),
            structure_cache_address: QWord::default(),
            catch_block_pointer: QWord::default(),
            control: 0,
            stack_cache_base: 0,
            array_event_count: 0,
            list_cache_length: 0,
            structure_cache_length: 0,
            binding_stack_pointer: 0,
            binding_stack_limit: 0,
            deep_bound_p: false,
            preempt_register: 0,
            alu_and_rotate_control: 0,
            alu_op: 0,
            byte_size: 0,
            byte_rotate: 0,
            rotate_latch: 0,
            alu_overflow: false,
            alu_borrow: false,
            alu_less_than: false,
            ephemeral_oldspace_register: 0,
            zone_oldspace_register: 0,
            control_stack_limit: 0,
            control_stack_extra_limit: 0,
            dynamic_binding_cache_base: 0,
            dynamic_binding_cache_mask: 0,
            fep_mode_trap_vector_address: 0,
            mapping_table_cachee: 0,
            mapping_table_length: 0,
            running: false,
            suspend: false,
            instruction_count: 0,
        }
    }
}

impl CPU {
    pub fn initialise(&mut self, mut dataBase: *mut isize, mut tagsBase: *mut QTag) {
        let mut p: *mut QWord = 0 as *mut QWord;
        let mut q: *mut *mut QWord = 0 as *mut *mut QWord;
        let mut j: u32 = 0;

        if !self.allocated_caches {
            self.instruction_cache = [InstructionCacheLine::default(); INSTRUCTION_CACHE_SIZE];
            self.stack_cache = [QWord::default(); IVORY_PAGE_SIZE_QS * IVORY_STACK_CACHE_SIZE];
            self.stack_cache_limit = IVORY_PAGE_SIZE_QS * IVORY_STACK_CACHE_SIZE - 128;
            self.allocated_caches = true;
        }

        self.running = false;
        self.instruction_count = 0;

        let mut i: u32 = 0;
        for i in 0..INSTRUCTION_CACHE_SIZE / 2 {
            self.instruction_cache[2 * i as usize] = InstructionCacheLine {
                pc: QWord {
                    parts: QCDRTagData {
                        cdr: CDR::Nil,
                        tag: QTag::EvenPC,
                        data: QImmediate { u: 0 },
                    },
                },
                next_pc: QWord::default(),
                code: 0,
                operand: 0,
                instruction: 0,
                next_cp: Rc::new(RefCell::default()),
            };

            self.instruction_cache[2 * i as usize + 1] = InstructionCacheLine {
                pc: QWord {
                    parts: QCDRTagData {
                        cdr: CDR::Nil,
                        tag: QTag::OddPC,
                        data: QImmediate { u: 0 },
                    },
                },
                next_pc: QWord::default(),
                code: 0,
                operand: 0,
                instruction: 0,
                next_cp: Rc::new(RefCell::default()),
            };
        }

        self.StackCacheBase = MEMORY_STACK_CACHE_BASE;
        for i in 0..IVORY_PAGE_SIZE_QS * IVORY_STACK_CACHE_SIZE {
            self.stack_cache[i as usize] = QWord {
                parts: QCDRTagData {
                    cdr: CDR::Nil,
                    tag: QTag::Null,
                    data: QImmediate { u: i },
                },
            };
        }

        self.fp = 4;
        self.sp = 5;
        self.lp = 6;

        self.control = 0;
        write_control_argument_size(&mut self.control, 2);
        write_control_caller_frame_size(&mut self.control, 2);
        write_control_trap_mode(&mut self.control, TrapMode::FEP);

        self.pc = QWord {
            parts: QCDRTagData {
                cdr: CDR::Jump,
                tag: QTag::NIL,
                data: QImmediate { u: 0 },
            },
        };
        self.continuation = QWord {
            parts: QCDRTagData {
                cdr: CDR::Jump,
                tag: QTag::NIL,
                data: QImmediate { u: 0 },
            },
        };

        // Push initial frames:  These are a lie, they will be popped when you
        // start, so that the "continuation" at 4 becomes the PC.  The
        // continuation and control for the running frame are NIL and 0,
        // respectively, thus returning from that frame will not adjust the FP
        // and the sequencer will know to halt on seeing NIL as a PC.
        self.push_one_fake_frame();
        self.push_one_fake_frame();

        // EnsureVirtualAddressRange(0xf8000100, 0xf00, false);
        // EnsureVirtualAddressRange(0xf8062000, 0x9e000, false);
    }

    pub fn running_p(&self) -> bool {
        return self.running;
    }

    pub fn halt_machine(&mut self) {
        if self.running_p() {
            self.suspend = true;
        }
    }

    pub fn reset_machine(self) {}

    pub fn start_machine(&mut self, resume_p: bool) {
        self.suspend = false;
    }
}
