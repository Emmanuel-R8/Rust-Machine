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

pub fn read_control_argument_size(c: QWord) -> Option<u32> {
    return read_control_bits(c, 8, 0);
}

pub fn read_control_extra_argument(c: QWord) -> Option<u32> {
    return read_control_bits(c, 1, 8);
}

pub fn read_control_caller_frame_size(c: QWord) -> Option<u32> {
    return read_control_bits(c, 8, 9);
}

pub fn read_control_apply(c: QWord) -> Option<u32> {
    return read_control_bits(c, 1, 17);
}

pub fn read_control_value_disposition(c: QWord) -> Option<u32> {
    return read_control_bits(c, 2, 18);
}

pub fn read_control_cleanup_bits(c: QWord) -> Option<u32> {
    return read_control_bits(c, 3, 24);
}

pub fn read_control_cleanup_catch(c: QWord) -> Option<u32> {
    return read_control_bits(c, 1, 26);
}

pub fn read_control_cleanup_bindings(c: QWord) -> Option<u32> {
    return read_control_bits(c, 1, 25);
}

pub fn read_control_trap_on_exit(c: QWord) -> Option<u32> {
    return read_control_bits(c, 1, 24);
}

pub fn read_control_trap_mode(c: QWord) -> Option<u32> {
    return read_control_bits(c, 2, 30);
}

pub fn read_control_call_started(c: QWord) -> Option<u32> {
    return read_control_bits(c, 1, 22);
}

pub fn read_control_cleanup_in_progress(c: QWord) -> Option<u32> {
    return read_control_bits(c, 1, 23);
}

pub fn read_control_instruction_trace(c: QWord) -> Option<u32> {
    return read_control_bits(c, 1, 29);
}

pub fn read_control_call_trace(c: QWord) -> Option<u32> {
    return read_control_bits(c, 1, 28);
}

pub fn read_control_trace_pending(c: QWord) -> Option<u32> {
    return read_control_bits(c, 1, 27);
}

pub fn read_control_bits(c: QWord, ss: u8, pp: u8) -> Option<u32> {
    match c {
        QWord::CdrTagData(p) => match p.data {
            QImmediate::Unsigned(val) => return Some(ldb(ss, pp, val)),
            _ => return None,
        },
        _ => return None,
    }
}

pub fn write_control_argument_size(c: &mut QWord, x: u32) {
    write_control_bits(c, x, 8, 0);
}

pub fn write_control_extra_argument(c: &mut QWord, x: u32) {
    write_control_bits(c, x, 1, 8);
}

pub fn write_control_caller_frame_size(c: &mut QWord, x: u32) {
    write_control_bits(c, x, 8, 9);
}

pub fn write_control_apply(c: &mut QWord, x: u32) {
    write_control_bits(c, x, 1, 17);
}

pub fn write_control_value_disposition(c: &mut QWord, x: u32) {
    write_control_bits(c, x, 2, 18);
}

pub fn write_control_cleanup_bits(c: &mut QWord, x: u32) {
    write_control_bits(c, x, 3, 24);
}

pub fn write_control_cleanup_catch(c: &mut QWord, x: u32) {
    write_control_bits(c, x, 1, 26);
}

pub fn write_control_cleanup_bindings(c: &mut QWord, x: u32) {
    write_control_bits(c, x, 1, 25);
}

pub fn write_control_trap_on_exit(c: &mut QWord, x: u32) {
    write_control_bits(c, x, 1, 24);
}

pub fn write_control_trap_mode(c: &mut QWord, x: u32) {
    write_control_bits(c, x, 2, 30);
}

pub fn write_control_call_started(c: &mut QWord, x: u32) {
    write_control_bits(c, x, 1, 22);
}

pub fn write_control_cleanup_in_progress(c: &mut QWord, x: u32) {
    write_control_bits(c, x, 1, 23);
}

pub fn write_control_instruction_trace(c: &mut QWord, x: u32) {
    write_control_bits(c, x, 1, 29);
}

pub fn write_control_call_trace(c: &mut QWord, x: u32) {
    write_control_bits(c, x, 1, 28);
}

pub fn write_control_trace_pending(c: &mut QWord, x: u32) {
    write_control_bits(c, x, 1, 27);
}

pub fn write_control_bits(c: &mut QWord, x: u32, ss: u8, pp: u8) {
    match c {
        QWord::CdrTagData(p) => match p.data {
            QImmediate::Unsigned(mut val) => {
                val = dpb(x, ss, pp, val);
            }
            _ => {}
        },
        _ => {}
    }
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
    pub instruction_cache: Vec<InstructionCacheLine>,
    pub stack_cache: [QWord; (IVORY_PAGE_SIZE_QS * IVORY_STACK_CACHE_SIZE) as usize],
    pub stack_cache_limit: QWord,

    pub bar: [Bar; 4],
    pub list_cache_area: QWord,
    pub list_cache_address: QWord,
    pub structure_cache_area: QWord,
    pub structure_cache_address: QWord,
    pub catch_block_pointer: QWord,
    pub control: QWord,
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
            pc: QWord::CdrTagData(QCDRTagData {
                cdr: CDR::Normal,
                tag: QTag::EvenPC, // See IMAS p. 59
                data: QImmediate::Addr(PROGRAM_COUNTER_INIT),
            }),
            continuation: QWord::default(),
            instruction_cache: vec![
                InstructionCacheLine::default();
                INSTRUCTION_CACHE_SIZE as usize
            ],
            stack_cache: [QWord::Whole(0); (IVORY_PAGE_SIZE_QS * IVORY_STACK_CACHE_SIZE) as usize],
            stack_cache_limit: QWord::default(),
            allocated_caches: false,

            bar: [Bar::default(); 4],
            list_cache_area: QWord::default(),
            list_cache_address: QWord::default(),
            structure_cache_area: QWord::default(),
            structure_cache_address: QWord::default(),
            catch_block_pointer: QWord::default(),
            control: QWord::default(),
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

            self.stack_cache =
                [QWord::default(); (IVORY_PAGE_SIZE_QS * IVORY_STACK_CACHE_SIZE) as usize];

            self.stack_cache_limit = QWord::CdrTagData(QCDRTagData {
                cdr: CDR::Jump,
                tag: QTag::Fixnum,
                data: QImmediate::Unsigned(IVORY_PAGE_SIZE_QS * IVORY_STACK_CACHE_SIZE - 128),
            });

            self.allocated_caches = true;
        }

        self.running = false;
        self.instruction_count = 0;

        for i in 0..INSTRUCTION_CACHE_SIZE / 2 {
            self.instruction_cache[2 * i as usize] = InstructionCacheLine {
                pc: QWord::CdrTagData(QCDRTagData {
                    cdr: CDR::Jump,
                    tag: QTag::EvenPC,
                    data: QImmediate::Unsigned(0),
                }),
                next_pc: QWord::default(),
                code: 0,
                operand: 0,
                instruction: 0,
                next_cp: Rc::new(RefCell::default()),
            };

            self.instruction_cache[2 * i as usize + 1] = InstructionCacheLine {
                pc: QWord::CdrTagData(QCDRTagData {
                    cdr: CDR::Jump,
                    tag: QTag::OddPC,
                    data: QImmediate::Unsigned(0),
                }),
                next_pc: QWord::default(),
                code: 0,
                operand: 0,
                instruction: 0,
                next_cp: Rc::new(RefCell::default()),
            };
        }

        self.stack_cache_base = MEMORY_STACK_CACHE_BASE;
        for i in 0..IVORY_PAGE_SIZE_QS * IVORY_STACK_CACHE_SIZE {
            self.stack_cache[i as usize] = QWord::CdrTagData(QCDRTagData {
                cdr: CDR::Jump,
                tag: QTag::Null,
                data: QImmediate::Unsigned(i),
            });
        }

        self.fp = QWord::CdrTagData(QCDRTagData {
            cdr: CDR::Jump,
            tag: QTag::OddPC,
            data: QImmediate::Addr(4),
        });

        self.sp = QWord::CdrTagData(QCDRTagData {
            cdr: CDR::Jump,
            tag: QTag::OddPC,
            data: QImmediate::Addr(5),
        });

        self.lp = QWord::CdrTagData(QCDRTagData {
            cdr: CDR::Jump,
            tag: QTag::OddPC,
            data: QImmediate::Addr(6),
        });

        self.control = QWord::CdrTagData(QCDRTagData {
            cdr: CDR::Jump,
            tag: QTag::Fixnum,
            data: QImmediate::Unsigned(0),
        });

        write_control_argument_size(&mut self.control, 2);
        write_control_caller_frame_size(&mut self.control, 2);
        write_control_trap_mode(&mut self.control, TrapMode::FEP as u32);

        self.pc = QWord::CdrTagData(QCDRTagData {
            cdr: CDR::Jump,
            tag: QTag::NIL,
            data: QImmediate::Unsigned(0),
        });

        self.continuation = QWord::CdrTagData(QCDRTagData {
            cdr: CDR::Jump,
            tag: QTag::NIL,
            data: QImmediate::Unsigned(0),
        });
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
        match self.fp {
            QWord::CdrTagData(p) => match p.data {
                QImmediate::Addr(mut val) => {
                    val += addr;
                }
                _ => {}
            },
            _ => {}
        }
    }

    pub fn fp_dec(&mut self, addr: u32) {
        match self.fp {
            QWord::CdrTagData(p) => match p.data {
                QImmediate::Addr(mut val) => {
                    val -= addr;
                }
                _ => {}
            },
            _ => {}
        }
    }

    pub fn fp_jump(&mut self, addr: i32) {
        if addr >= 0 {
            return self.fp_inc(addr as u32);
        } else {
            return self.fp_dec(addr.abs() as u32);
        };
    }

    pub fn lp_inc(&mut self, addr: u32) {
        match self.lp {
            QWord::CdrTagData(p) => match p.data {
                QImmediate::Addr(mut val) => {
                    val += addr;
                }
                _ => {}
            },
            _ => {}
        }
    }

    pub fn lp_dec(&mut self, addr: u32) {
        match self.lp {
            QWord::CdrTagData(p) => match p.data {
                QImmediate::Addr(mut val) => {
                    val -= addr;
                }
                _ => {}
            },
            _ => {}
        }
    }

    pub fn lp_jump(&mut self, addr: i32) {
        if addr >= 0 {
            return self.lp_inc(addr as u32);
        } else {
            return self.lp_dec(addr.abs() as u32);
        };
    }

    pub fn set_control(&mut self, ctrl: u32) {
        match self.control {
            QWord::CdrTagData(p) => {
                p.data.set_u(ctrl);
            }
            _ => {}
        }
    }
}
