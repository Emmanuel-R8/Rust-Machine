use std::cell::RefCell;
use std::rc::Rc;

use crate::common::constants::{
    QTag,
    TrapMode,
    CDR,
    IVORY_PAGE_SIZE_QS,
    IVORY_STACK_CACHE_SIZE,
    MEMORY_STACK_CACHE_BASE,
};
use crate::common::types::{
    Bar,
    InstructionCacheLine,
    QCDRTagData,
    MemoryCell,
    INSTRUCTION_CACHE_SIZE, Address,
};
use crate::utils::{ dpb, ldb };

const CONTROL_ARGUMENT_SIZE: u32 = 0o377;
const CONTROL_EXTRA_ARGUMENT: u32 = 0o400;
const CONTROL_CALLER_FRAME_SIZE: u32 = 0o377_000;
const CONTROL_APPLY: u32 = 0o400_000;
const CONTROL_VALUE_DISPOSITION: u32 = 0o3_000_000;
const CONTROL_CALL_STARTED: u32 = 0o20_000_000;
const CONTROL_CLEANUP_BITS: u32 = 0o700_000_000;
const CONTROL_TRACE_BITS: u32 = 0o7_000_000_000;

pub fn read_control_argument_size(c: MemoryCell) -> u32 {
    return read_control_bits(c, 8, 0);
}

pub fn read_control_extra_argument(c: MemoryCell) -> u32 {
    return read_control_bits(c, 1, 8);
}

pub fn read_control_caller_frame_size(c: MemoryCell) -> u32 {
    return read_control_bits(c, 8, 9);
}

pub fn read_control_apply(c: MemoryCell) -> u32 {
    return read_control_bits(c, 1, 17);
}

pub fn read_control_value_disposition(c: MemoryCell) -> u32 {
    return read_control_bits(c, 2, 18);
}

pub fn read_control_cleanup_bits(c: MemoryCell) -> u32 {
    return read_control_bits(c, 3, 24);
}

pub fn read_control_cleanup_catch(c: MemoryCell) -> u32 {
    return read_control_bits(c, 1, 26);
}

pub fn read_control_cleanup_bindings(c: MemoryCell) -> u32 {
    return read_control_bits(c, 1, 25);
}

pub fn read_control_trap_on_exit(c: MemoryCell) -> u32 {
    return read_control_bits(c, 1, 24);
}

pub fn read_control_trap_mode(c: MemoryCell) -> u32 {
    return read_control_bits(c, 2, 30);
}

pub fn read_control_call_started(c: MemoryCell) -> u32 {
    return read_control_bits(c, 1, 22);
}

pub fn read_control_cleanup_in_progress(c: MemoryCell) -> u32 {
    return read_control_bits(c, 1, 23);
}

pub fn read_control_instruction_trace(c: MemoryCell) -> u32 {
    return read_control_bits(c, 1, 29);
}

pub fn read_control_call_trace(c: MemoryCell) -> u32 {
    return read_control_bits(c, 1, 28);
}

pub fn read_control_trace_pending(c: MemoryCell) -> u32 {
    return read_control_bits(c, 1, 27);
}

// This assumes that the memory content is the right one.
pub fn read_control_bits(c: MemoryCell, ss: u8, pp: u8) -> u32 {
    return ldb(ss, pp, c.as_raw());
}

pub fn write_control_argument_size(c: &mut MemoryCell, x: Address) {
    write_control_bits(c, x, 8, 0);
}

pub fn write_control_extra_argument(c: &mut MemoryCell, x: Address) {
    write_control_bits(c, x, 1, 8);
}

pub fn write_control_caller_frame_size(c: &mut MemoryCell, x: Address) {
    write_control_bits(c, x, 8, 9);
}

pub fn write_control_apply(c: &mut MemoryCell, x: Address) {
    write_control_bits(c, x, 1, 17);
}

pub fn write_control_value_disposition(c: &mut MemoryCell, x: Address) {
    write_control_bits(c, x, 2, 18);
}

pub fn write_control_cleanup_bits(c: &mut MemoryCell, x: Address) {
    write_control_bits(c, x, 3, 24);
}

pub fn write_control_cleanup_catch(c: &mut MemoryCell, x: Address) {
    write_control_bits(c, x, 1, 26);
}

pub fn write_control_cleanup_bindings(c: &mut MemoryCell, x: Address) {
    write_control_bits(c, x, 1, 25);
}

pub fn write_control_trap_on_exit(c: &mut MemoryCell, x: Address) {
    write_control_bits(c, x, 1, 24);
}

pub fn write_control_trap_mode(c: &mut MemoryCell, x: Address) {
    write_control_bits(c, x, 2, 30);
}

pub fn write_control_call_started(c: &mut MemoryCell, x: Address) {
    write_control_bits(c, x, 1, 22);
}

pub fn write_control_cleanup_in_progress(c: &mut MemoryCell, x: Address) {
    write_control_bits(c, x, 1, 23);
}

pub fn write_control_instruction_trace(c: &mut MemoryCell, x: Address) {
    write_control_bits(c, x, 1, 29);
}

pub fn write_control_call_trace(c: &mut MemoryCell, x: Address) {
    write_control_bits(c, x, 1, 28);
}

pub fn write_control_trace_pending(c: &mut MemoryCell, x: Address) {
    write_control_bits(c, x, 1, 27);
}

pub fn write_control_bits(c: &mut MemoryCell, x: Address, ss: u8, pp: u8) {
    c.set_raw(dpb(x as u32, ss, pp, c.as_raw()));
}

#[derive(Debug, Clone)]
pub struct CPU {
    pub sp: MemoryCell,
    pub restarts_p: MemoryCell,
    pub fp: MemoryCell,
    pub lp: MemoryCell,
    pub pc: MemoryCell,
    pub continuation: MemoryCell,

    pub allocated_caches: bool,
    pub instruction_cache: Vec<InstructionCacheLine>,
    pub stack_cache: [MemoryCell; (IVORY_PAGE_SIZE_QS * IVORY_STACK_CACHE_SIZE) as usize],
    pub stack_cache_limit: MemoryCell,

    pub bar: [Bar; 4],
    pub list_cache_area: MemoryCell,
    pub list_cache_address: MemoryCell,
    pub structure_cache_area: MemoryCell,
    pub structure_cache_address: MemoryCell,
    pub catch_block_pointer: MemoryCell,
    pub control: MemoryCell,
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

pub const PROGRAM_COUNTER_INIT: u32 = 0x07fe_0040;

impl Default for CPU {
    fn default() -> Self {
        Self {
            sp: MemoryCell::default(),
            restarts_p: MemoryCell::default(),
            fp: MemoryCell::default(),
            lp: MemoryCell::default(),
            pc: MemoryCell::new_cdr_tag_a(
                CDR::Normal,
                QTag::EvenPC, // See IMAS p. 59
                PROGRAM_COUNTER_INIT
            ),
            continuation: MemoryCell::default(),
            instruction_cache: vec![
                InstructionCacheLine::default();
                INSTRUCTION_CACHE_SIZE as usize
            ],
            stack_cache: [
                MemoryCell::default();
                (IVORY_PAGE_SIZE_QS * IVORY_STACK_CACHE_SIZE) as usize
            ],
            stack_cache_limit: MemoryCell::default(),
            allocated_caches: false,

            bar: [Bar::default(); 4],
            list_cache_area: MemoryCell::default(),
            list_cache_address: MemoryCell::default(),
            structure_cache_area: MemoryCell::default(),
            structure_cache_address: MemoryCell::default(),
            catch_block_pointer: MemoryCell::default(),
            control: MemoryCell::default(),
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
    pub fn initialise(&mut self) {
        if !self.allocated_caches {
            self.instruction_cache =
                vec![InstructionCacheLine::default(); INSTRUCTION_CACHE_SIZE as usize];

            self.stack_cache = [
                MemoryCell::default();
                (IVORY_PAGE_SIZE_QS * IVORY_STACK_CACHE_SIZE) as usize
            ];

            self.stack_cache_limit = MemoryCell::new_cdr_tag_u(
                CDR::Jump,
                QTag::Fixnum,
                IVORY_PAGE_SIZE_QS * IVORY_STACK_CACHE_SIZE - 128
            );

            self.allocated_caches = true;
        }

        self.running = false;
        self.instruction_count = 0;

        for i in 0..INSTRUCTION_CACHE_SIZE / 2 {
            self.instruction_cache[2 * (i as usize)] = InstructionCacheLine {
                pc: MemoryCell::new_cdr_tag_u(CDR::Jump, QTag::EvenPC, 0),
                next_pc: MemoryCell::default(),
                code: 0,
                operand: 0,
                instruction: 0,
                next_cp: Rc::new(RefCell::default()),
            };

            self.instruction_cache[2 * (i as usize) + 1] = InstructionCacheLine {
                pc: MemoryCell::new_cdr_tag_u(CDR::Jump, QTag::OddPC, 0),
                next_pc: MemoryCell::default(),
                code: 0,
                operand: 0,
                instruction: 0,
                next_cp: Rc::new(RefCell::default()),
            };
        }

        self.stack_cache_base = MEMORY_STACK_CACHE_BASE;
        for i in 0..IVORY_PAGE_SIZE_QS * IVORY_STACK_CACHE_SIZE {
            self.stack_cache[i as usize] = MemoryCell::new_cdr_tag_u(CDR::Jump, QTag::Null, 0);
        }

        self.fp = MemoryCell::new_cdr_tag_u(CDR::Jump, QTag::OddPC, 4);
        self.sp = MemoryCell::new_cdr_tag_u(CDR::Jump, QTag::OddPC, 5);
        self.lp = MemoryCell::new_cdr_tag_u(CDR::Jump, QTag::OddPC, 6);

        self.control = MemoryCell::new_cdr_tag_u(CDR::Jump, QTag::Null, 0);
        write_control_argument_size(&mut self.control, 2);
        write_control_caller_frame_size(&mut self.control, 2);
        write_control_trap_mode(&mut self.control, TrapMode::FEP as Address);

        self.pc = MemoryCell::new_cdr_tag_u(CDR::Jump, QTag::Null, 0);
        self.continuation = MemoryCell::new_cdr_tag_u(CDR::Jump, QTag::Null, 0);
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

    pub fn fp_inc(&mut self, addr: u32) {
        self.fp += MemoryCell::new_cdr_tag_a(CDR::Jump, QTag::OddPC, addr);
    }

    pub fn fp_dec(&mut self, addr: u32) {
        self.fp-=MemoryCell::new_cdr_tag_a(CDR::Jump, QTag::OddPC, addr);
    }

    pub fn fp_jump(&mut self, addr: i32) {
        if addr >= 0 {
            return self.fp_inc(addr as u32);
        } else {
            return self.fp_dec(addr.abs() as u32);
        }
    }

    pub fn lp_inc(&mut self, addr: u32) {
        self.lp += MemoryCell::new_cdr_tag_a(CDR::Jump, QTag::OddPC, addr);
    }

    pub fn lp_dec(&mut self, addr: u32) {
        self.lp-=MemoryCell::new_cdr_tag_a(CDR::Jump, QTag::OddPC, addr);
    }

    pub fn lp_jump(&mut self, addr: i32) {
        if addr >= 0 {
            return self.lp_inc(addr as u32);
        } else {
            return self.lp_dec(addr.abs() as u32);
        }
    }

    pub fn set_control(&mut self, ctrl: u32) {
        self.control.set_raw(ctrl);
    }
}
