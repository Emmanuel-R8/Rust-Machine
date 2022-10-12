#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    fn MemoryReadInternal(vma: isize, object: *mut LispObj, row: *mut Byte) -> isize;
    static mut MemoryActionTable: [[Byte; 64]; 12];
    fn StackCacheScrollUp();
    static mut processor: *mut ProcessorState;
}
pub type u8 = libc::c_uchar;
pub type i32 = u32;
pub type u32 = libc::c_uint;
pub type u64 = libc::c_ulong;
pub type i32 = i32;
pub type u8 = u8;
pub type ui32 = u32;
pub type u64 = u64;
pub type Byte = u8;
pub type isize = u64;
pub type Boole = u8;
#[derive(Copy, Clone)]
#[repr(C)]
pub union LispObj {
    pub parts: _LispObj,
    pub whole: u64,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _LispObj {
    pub tag: ui32,
    pub data: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub u: ui32,
    pub s: i32,
    pub f: libc::c_float,
}
pub type PC = LispObj;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _InstructionCacheLine {
    pub pc: PC,
    pub next_pc: PC,
    pub code: u32,
    pub operand: u32,
    pub instruction: libc::c_uint,
    pub next_cp: *mut _InstructionCacheLine,
}
pub type InstructionCacheLine = _InstructionCacheLine;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _ProcessorState {
    pub sp: *mut LispObj,
    pub restartsp: *mut LispObj,
    pub fp: *mut LispObj,
    pub lp: *mut LispObj,
    pub pc: PC,
    pub continuation: PC,
    pub InstructionCache: *mut InstructionCacheLine,
    pub StackCache: *mut LispObj,
    pub StackCacheLimit: *mut LispObj,
    pub bar: [_bar; 4],
    pub ListCacheArea: LispObj,
    pub ListCacheAddress: LispObj,
    pub StructureCacheArea: LispObj,
    pub StructureCacheAddress: LispObj,
    pub CatchBlockPointer: LispObj,
    pub control: isize,
    pub StackCacheBase: isize,
    pub ArrayEventCount: isize,
    pub ListCacheLength: isize,
    pub StructureCacheLength: isize,
    pub BindingStackPointer: isize,
    pub BindingStackLimit: isize,
    pub DeepBoundP: Boole,
    pub PreemptRegister: isize,
    pub AluAndRotateControl: isize,
    pub AluOp: Option::<fn() -> isize>,
    pub ByteSize: isize,
    pub ByteRotate: isize,
    pub RotateLatch: isize,
    pub ALUOverflow: Boole,
    pub ALUBorrow: Boole,
    pub ALULessThan: Boole,
    pub EphemeralOldspaceRegister: isize,
    pub ZoneOldspaceRegister: isize,
    pub ControlStackLimit: isize,
    pub ControlStackExtraLimit: isize,
    pub DynamicBindingCacheBase: isize,
    pub DynamicBindingCacheMask: isize,
    pub FEPModeTrapVectorAddress: isize,
    pub MappingTableCache: isize,
    pub MappingTableLength: isize,
    pub running: Boole,
    pub instruction_count: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _bar {
    pub address: LispObj,
    pub mapped: *mut LispObj,
}
pub type ProcessorState = _ProcessorState;
pub type _MemoryCycleTypes = libc::c_uint;
pub const CycleRawTranslate: _MemoryCycleTypes = 12;
pub const CycleRaw: _MemoryCycleTypes = 11;
pub const CycleGCCopy: _MemoryCycleTypes = 10;
pub const CycleCdr: _MemoryCycleTypes = 9;
pub const CycleScavenge: _MemoryCycleTypes = 8;
pub const CycleStructureOffset: _MemoryCycleTypes = 7;
pub const CycleHeader: _MemoryCycleTypes = 6;
pub const CycleBindWriteNoMonitor: _MemoryCycleTypes = 5;
pub const CycleBindReadNoMonitor: _MemoryCycleTypes = 4;
pub const CycleBindWrite: _MemoryCycleTypes = 3;
pub const CycleBindRead: _MemoryCycleTypes = 2;
pub const CycleDataWrite: _MemoryCycleTypes = 1;
pub const CycleDataRead: _MemoryCycleTypes = 0;
pub type _IvoryType = libc::c_uint;
pub const TypePackedInstruction77: _IvoryType = 63;
pub const TypePackedInstruction76: _IvoryType = 62;
pub const TypePackedInstruction75: _IvoryType = 61;
pub const TypePackedInstruction74: _IvoryType = 60;
pub const TypePackedInstruction73: _IvoryType = 59;
pub const TypePackedInstruction72: _IvoryType = 58;
pub const TypePackedInstruction71: _IvoryType = 57;
pub const TypePackedInstruction70: _IvoryType = 56;
pub const TypePackedInstruction67: _IvoryType = 55;
pub const TypePackedInstruction66: _IvoryType = 54;
pub const TypePackedInstruction65: _IvoryType = 53;
pub const TypePackedInstruction64: _IvoryType = 52;
pub const TypePackedInstruction63: _IvoryType = 51;
pub const TypePackedInstruction62: _IvoryType = 50;
pub const TypePackedInstruction61: _IvoryType = 49;
pub const TypePackedInstruction60: _IvoryType = 48;
pub const TypeCallGenericPrefetch: _IvoryType = 47;
pub const TypeCallIndirectPrefetch: _IvoryType = 46;
pub const TypeCallCompiledOddPrefetch: _IvoryType = 45;
pub const TypeCallCompiledEvenPrefetch: _IvoryType = 44;
pub const TypeCallGeneric: _IvoryType = 43;
pub const TypeCallIndirect: _IvoryType = 42;
pub const TypeCallCompiledOdd: _IvoryType = 41;
pub const TypeCallCompiledEven: _IvoryType = 40;
pub const TypeOddPC: _IvoryType = 39;
pub const TypeEvenPC: _IvoryType = 38;
pub const TypeGCForward: _IvoryType = 37;
pub const TypeLogicVariable: _IvoryType = 36;
pub const TypeCharacter: _IvoryType = 35;
pub const TypeBoundLocation: _IvoryType = 34;
pub const TypeSpareImmediate1: _IvoryType = 33;
pub const TypePhysicalAddress: _IvoryType = 32;
pub const TypeSparePointer2: _IvoryType = 31;
pub const TypeSparePointer1: _IvoryType = 30;
pub const TypeGenericFunction: _IvoryType = 29;
pub const TypeCompiledFunction: _IvoryType = 28;
pub const TypeDynamicClosure: _IvoryType = 27;
pub const TypeLexicalClosure: _IvoryType = 26;
pub const TypeLocative: _IvoryType = 25;
pub const TypeSymbol: _IvoryType = 24;
pub const TypeString: _IvoryType = 23;
pub const TypeArray: _IvoryType = 22;
pub const TypeList: _IvoryType = 21;
pub const TypeNIL: _IvoryType = 20;
pub const TypeStringInstance: _IvoryType = 19;
pub const TypeArrayInstance: _IvoryType = 18;
pub const TypeListInstance: _IvoryType = 17;
pub const TypeInstance: _IvoryType = 16;
pub const TypeSpareNumber: _IvoryType = 15;
pub const TypeComplex: _IvoryType = 14;
pub const TypeBigRatio: _IvoryType = 13;
pub const TypeBignum: _IvoryType = 12;
pub const TypeDoubleFloat: _IvoryType = 11;
pub const TypeSingleFloat: _IvoryType = 10;
pub const TypeSmallRatio: _IvoryType = 9;
pub const TypeFixnum: _IvoryType = 8;
pub const TypeElementForward: _IvoryType = 7;
pub const TypeHeaderForward: _IvoryType = 6;
pub const TypeOneQForward: _IvoryType = 5;
pub const TypeExternalValueCellPointer: _IvoryType = 4;
pub const TypeHeaderI: _IvoryType = 3;
pub const TypeHeaderP: _IvoryType = 2;
pub const TypeMonitorForward: _IvoryType = 1;
pub const TypeNull: _IvoryType = 0;
pub type _IvoryValueDisposition = libc::c_uint;
pub const ValueDispositionMultiple: _IvoryValueDisposition = 3;
pub const ValueDispositionReturn: _IvoryValueDisposition = 2;
pub const ValueDispositionValue: _IvoryValueDisposition = 1;
pub const ValueDispositionEffect: _IvoryValueDisposition = 0;
pub type _TrapVectors = libc::c_uint;
pub const DBUnwindCatchTrapVector: _TrapVectors = 2647;
pub const DBUnwindFrameTrapVector: _TrapVectors = 2646;
pub const DBCacheMissTrapVector: _TrapVectors = 2645;
pub const MemoryBusErrorTrapVector: _TrapVectors = 2644;
pub const UncorrectableMemoryErrorTrapVector: _TrapVectors = 2643;
pub const PageWriteFaultTrapVector: _TrapVectors = 2642;
pub const PageFaultRequestTrapVector: _TrapVectors = 2641;
pub const PageNotResidentTrapVector: _TrapVectors = 2640;
pub const MessageDispatchTrapVector: _TrapVectors = 2638;
pub const GenericDispatchTrapVector: _TrapVectors = 2636;
pub const MonitorTrapVector: _TrapVectors = 2634;
pub const HighPrioritySequenceBreakTrapVector: _TrapVectors = 2633;
pub const LowPrioritySequenceBreakTrapVector: _TrapVectors = 2632;
pub const FepModeTrapVector: _TrapVectors = 2631;
pub const TransportTrapVector: _TrapVectors = 2630;
pub const PreemptRequestTrapVector: _TrapVectors = 2629;
pub const TraceTrapVector: _TrapVectors = 2628;
pub const StackOverflowTrapVector: _TrapVectors = 2627;
pub const PullApplyArgsTrapVector: _TrapVectors = 2626;
pub const ResetTrapVector: _TrapVectors = 2625;
pub const ErrorTrapVector: _TrapVectors = 2624;
pub const GenericDispatchVector: _TrapVectors = 2560;
pub const InterpreterFunctionVector: _TrapVectors = 2304;
pub const InstructionExceptionVector: _TrapVectors = 2048;
pub const ArithmeticInstructionExceptionVector: _TrapVectors = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _ExceptionInfo {
    pub arity: u32,
    pub stackp: u32,
    pub arithp: u32,
}
pub type ExceptionInfo = _ExceptionInfo;
#[no_mangle]
pub static mut InstructionExceptionInfo: [ExceptionInfo; 256] = [
    {
        let mut init = _ExceptionInfo {
            arity: 1,
            stackp: 0,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 1,
            stackp: 0,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 0,
            stackp: 1,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 1,
            stackp: 0,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 1,
            stackp: 0,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 1,
            stackp: 0,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 1,
            stackp: 0,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 0,
            stackp: 1,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 0,
            stackp: 1,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 0,
            stackp: 1,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 0,
            stackp: 1,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 0,
            stackp: 1,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 1,
            stackp: 0,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 0,
            stackp: 1,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 0,
            stackp: 1,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 0,
            stackp: 1,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 1,
            stackp: 0,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 0,
            stackp: 1,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 0,
            stackp: 1,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 0,
            stackp: 1,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 0,
            stackp: 1,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 0,
            stackp: 1,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 0,
            stackp: 1,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 0,
            stackp: 1,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 0,
            stackp: 1,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 0,
            stackp: 1,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 0,
            stackp: 1,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 0,
            stackp: 1,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 1,
            stackp: 0,
            arithp: 1,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 1,
            stackp: 0,
            arithp: 1,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 1,
            stackp: 0,
            arithp: 1,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 0,
            stackp: 1,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 0,
            stackp: 1,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 0,
            stackp: 1,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 0,
            stackp: 1,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 0,
            stackp: 1,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 0,
            stackp: 1,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 0,
            stackp: 1,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 0,
            stackp: 1,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 0,
            stackp: 1,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 0,
            stackp: 1,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 0,
            stackp: 1,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 0,
            stackp: 1,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 0,
            stackp: 1,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 0,
            stackp: 1,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 0,
            stackp: 1,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 0,
            stackp: 1,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 0,
            stackp: 1,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 0,
            stackp: 1,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 0,
            stackp: 1,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 0,
            stackp: 1,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 0,
            stackp: 1,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 0,
            stackp: 1,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 0,
            stackp: 1,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 0,
            stackp: 1,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 0,
            stackp: 1,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 0,
            stackp: 1,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 0,
            stackp: 1,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 0,
            stackp: 1,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 0,
            stackp: 1,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 0,
            stackp: 1,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 0,
            stackp: 1,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 0,
            stackp: 1,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 0,
            stackp: 1,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 0,
            stackp: 1,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 0,
            stackp: 1,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 1,
            stackp: 0,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 0,
            stackp: 1,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 0,
            stackp: 1,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 0,
            stackp: 1,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 0,
            stackp: 1,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 0,
            stackp: 1,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 1,
            stackp: 0,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 1,
            stackp: 0,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 0,
            stackp: 1,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 0,
            stackp: 1,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 1,
            stackp: 0,
            arithp: 1,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 0,
            stackp: 1,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 0,
            stackp: 1,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 0,
            stackp: 1,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 0,
            stackp: 1,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 0,
            stackp: 1,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 0,
            stackp: 1,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 0,
            stackp: 1,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 0,
            stackp: 1,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 0,
            stackp: 1,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 0,
            stackp: 1,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 0,
            stackp: 1,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 2,
            stackp: 1,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 2,
            stackp: 1,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 2,
            stackp: 1,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 2,
            stackp: 1,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 0,
            stackp: 1,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 0,
            stackp: 1,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 0,
            stackp: 1,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 0,
            stackp: 1,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 1,
            stackp: 0,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 1,
            stackp: 0,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 1,
            stackp: 0,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 1,
            stackp: 0,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 1,
            stackp: 0,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 0,
            stackp: 1,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 0,
            stackp: 1,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 0,
            stackp: 1,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 0,
            stackp: 1,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 0,
            stackp: 1,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 0,
            stackp: 1,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 0,
            stackp: 1,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 0,
            stackp: 1,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 0,
            stackp: 1,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 0,
            stackp: 1,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 0,
            stackp: 1,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 1,
            stackp: 0,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 1,
            stackp: 0,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 1,
            stackp: 0,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 1,
            stackp: 0,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 0,
            stackp: 1,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 0,
            stackp: 1,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 0,
            stackp: 1,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 0,
            stackp: 1,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 1,
            stackp: 1,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 1,
            stackp: 1,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 0,
            stackp: 1,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 0,
            stackp: 1,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 0,
            stackp: 1,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 1,
            stackp: 1,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 0,
            stackp: 1,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 0,
            stackp: 1,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 2,
            stackp: 0,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 2,
            stackp: 0,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 2,
            stackp: 0,
            arithp: 1,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 2,
            stackp: 0,
            arithp: 1,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 2,
            stackp: 0,
            arithp: 1,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 2,
            stackp: 0,
            arithp: 1,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 2,
            stackp: 0,
            arithp: 1,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 2,
            stackp: 0,
            arithp: 1,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 0,
            stackp: 1,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 2,
            stackp: 0,
            arithp: 1,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 2,
            stackp: 0,
            arithp: 1,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 2,
            stackp: 0,
            arithp: 1,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 2,
            stackp: 0,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 2,
            stackp: 0,
            arithp: 1,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 2,
            stackp: 0,
            arithp: 1,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 2,
            stackp: 0,
            arithp: 1,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 0,
            stackp: 1,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 0,
            stackp: 1,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 0,
            stackp: 1,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 0,
            stackp: 1,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 2,
            stackp: 0,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 2,
            stackp: 0,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 2,
            stackp: 0,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 2,
            stackp: 0,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 0,
            stackp: 1,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 0,
            stackp: 1,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 2,
            stackp: 0,
            arithp: 1,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 0,
            stackp: 1,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 0,
            stackp: 1,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 0,
            stackp: 1,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 2,
            stackp: 0,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 2,
            stackp: 0,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 2,
            stackp: 0,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 0,
            stackp: 1,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 0,
            stackp: 1,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 0,
            stackp: 1,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 0,
            stackp: 1,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 0,
            stackp: 1,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 0,
            stackp: 1,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 0,
            stackp: 1,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 2,
            stackp: 0,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 0,
            stackp: 1,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 0,
            stackp: 1,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 0,
            stackp: 1,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 0,
            stackp: 1,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 0,
            stackp: 1,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 0,
            stackp: 1,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 0,
            stackp: 1,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 2,
            stackp: 0,
            arithp: 1,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 2,
            stackp: 0,
            arithp: 1,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 2,
            stackp: 0,
            arithp: 1,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 2,
            stackp: 0,
            arithp: 1,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 2,
            stackp: 0,
            arithp: 1,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 2,
            stackp: 0,
            arithp: 1,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 2,
            stackp: 0,
            arithp: 1,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 2,
            stackp: 0,
            arithp: 1,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 0,
            stackp: 1,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 0,
            stackp: 1,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 0,
            stackp: 1,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 2,
            stackp: 0,
            arithp: 1,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 0,
            stackp: 1,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 0,
            stackp: 1,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 0,
            stackp: 1,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 2,
            stackp: 0,
            arithp: 1,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 2,
            stackp: 0,
            arithp: 1,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 2,
            stackp: 0,
            arithp: 1,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 0,
            stackp: 1,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 0,
            stackp: 1,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 0,
            stackp: 1,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 0,
            stackp: 1,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 0,
            stackp: 1,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 0,
            stackp: 1,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 3,
            stackp: 0,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 2,
            stackp: 0,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 2,
            stackp: 0,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 2,
            stackp: 0,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 3,
            stackp: 0,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 2,
            stackp: 0,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 2,
            stackp: 0,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 2,
            stackp: 0,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 2,
            stackp: 0,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 2,
            stackp: 0,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 0,
            stackp: 1,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 0,
            stackp: 1,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 2,
            stackp: 0,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 3,
            stackp: 0,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 2,
            stackp: 0,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 0,
            stackp: 1,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 0,
            stackp: 1,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 0,
            stackp: 1,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 0,
            stackp: 1,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 0,
            stackp: 1,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 0,
            stackp: 1,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 0,
            stackp: 1,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 0,
            stackp: 1,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 0,
            stackp: 1,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 0,
            stackp: 1,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 0,
            stackp: 1,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 0,
            stackp: 1,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 0,
            stackp: 1,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 0,
            stackp: 1,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 0,
            stackp: 1,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 0,
            stackp: 1,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 0,
            stackp: 1,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 2,
            stackp: 0,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 3,
            stackp: 0,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 2,
            stackp: 0,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 0,
            stackp: 1,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 0,
            stackp: 1,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 0,
            stackp: 1,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 0,
            stackp: 1,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 0,
            stackp: 1,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 0,
            stackp: 1,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 0,
            stackp: 1,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 0,
            stackp: 1,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 0,
            stackp: 1,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 0,
            stackp: 1,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 0,
            stackp: 1,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 0,
            stackp: 1,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 0,
            stackp: 1,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 2,
            stackp: 1,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 2,
            stackp: 1,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 0,
            stackp: 1,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 0,
            stackp: 1,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 0,
            stackp: 1,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 2,
            stackp: 1,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 0,
            stackp: 1,
            arithp: 0,
        };
        init
    },
    {
        let mut init = _ExceptionInfo {
            arity: 0,
            stackp: 1,
            arithp: 0,
        };
        init
    },
];
 fn FetchTrapVectorEntry(
    mut index: isize,
    mut entry: *mut LispObj,
) -> usize {
    let mut ps: *mut ProcessorState = processor;
    let mut previous: usize = ((*ps).control >> 30
        & (((1) << 2) - 1) as libc::c_ulong)
       ;
    (*ps)
        .control = ((3
        & ((1) << 2) - 1)
        << 30) as libc::c_ulong
        | (*ps).control
            & !((((1) << 2) - 1)
                << 30) as libc::c_ulong;
    MemoryReadInternal(
        (0xf8040000 as libc::c_uint as libc::c_ulong)
            .wrapping_add(
                (if previous < 3 as usize {
                    index
                } else {
                    FepModeTrapVector as usize as libc::c_ulong
                }),
            ),
        entry,
        (MemoryActionTable[CycleDataRead as usize as usize]).as_mut_ptr(),
    );
    if !(((*entry).parts.tag ^ TypeOddPC as usize as libc::c_uint)
        & (((1) << 6) - 1) as libc::c_uint
        == 0 as usize as libc::c_uint
        || ((*entry).parts.tag ^ TypeEvenPC as usize as libc::c_uint)
            & (((1) << 6) - 1)
                as libc::c_uint == 0 as usize as libc::c_uint)
    {
        if previous == 3 as usize || FetchTrapVectorEntry(index, entry) == 0 {
            return 0;
        }
    }
    (*ps)
        .control = ((previous
        & ((1) << 2) - 1)
        << 30) as libc::c_ulong
        | (*ps).control
            & !((((1) << 2) - 1)
                << 30) as libc::c_ulong;
    return 1;
}
#[no_mangle]
pub  fn TakePreTrap(
    mut index: isize,
    mut extra1: *mut LispObj,
    mut extra2: *mut LispObj,
) -> usize {
    let mut ps: *mut ProcessorState = processor;
    let mut oldfp: *mut LispObj = (*ps).fp;
    let mut restartsp: *mut LispObj = (*ps).restartsp;
    let mut entry: LispObj = LispObj {
        parts: _LispObj {
            tag: 0,
            data: C2RustUnnamed { u: 0 },
        },
    };
    let ref mut fresh0 = (*ps).sp;
    *fresh0 = restartsp;
    if ((*ps).sp).offset(8 as usize as isize) > (*ps).StackCacheLimit {
        StackCacheScrollUp();
    }
    (*((*ps).sp).offset(1 as usize as isize))
        .parts
        .tag = 0o300 as usize as libc::c_uint | (*ps).continuation.parts.tag;
    (*((*ps).sp).offset(1 as usize as isize))
        .parts
        .data = (*ps).continuation.parts.data;
    let ref mut fresh1 = (*ps).sp;
    *fresh1 = (*fresh1).offset(1);
    (*((*ps).sp).offset(1 as usize as isize))
        .parts
        .tag = (0o300 as usize | TypeFixnum) as ui32;
    (*((*ps).sp).offset(1 as usize as isize))
        .parts
        .data
        .u = (*ps).control as ui32;
    let ref mut fresh2 = (*ps).sp;
    *fresh2 = (*fresh2).offset(1);
    (*((*ps).sp).offset(1 as usize as isize))
        .parts
        .tag = TypeFixnum as usize as ui32;
    (*((*ps).sp).offset(1 as usize as isize)).parts.data.u = index as ui32;
    let ref mut fresh3 = (*ps).sp;
    *fresh3 = (*fresh3).offset(1);
    (*((*ps).sp).offset(1 as usize as isize))
        .parts
        .tag = (*ps).pc.parts.tag
        & (((1) << 6) - 1) as libc::c_uint;
    (*((*ps).sp).offset(1 as usize as isize)).parts.data = (*ps).pc.parts.data;
    let ref mut fresh4 = (*ps).sp;
    *fresh4 = (*fresh4).offset(1);
    if !extra1.is_null() {
        (*((*ps).sp).offset(1 as usize as isize))
            .parts
            .tag = (*extra1).parts.tag
            & (((1) << 6) - 1)
                as libc::c_uint;
        (*((*ps).sp).offset(1 as usize as isize))
            .parts
            .data = (*extra1).parts.data;
        let ref mut fresh5 = (*ps).sp;
        *fresh5 = (*fresh5).offset(1);
    }
    if !extra2.is_null() {
        (*((*ps).sp).offset(1 as usize as isize))
            .parts
            .tag = (*extra2).parts.tag
            & (((1) << 6) - 1)
                as libc::c_uint;
        (*((*ps).sp).offset(1 as usize as isize))
            .parts
            .data = (*extra2).parts.data;
        let ref mut fresh6 = (*ps).sp;
        *fresh6 = (*fresh6).offset(1);
    }
    let ref mut fresh7 = (*ps).fp;
    *fresh7 = restartsp.offset(1 as usize as isize);
    let ref mut fresh8 = (*ps).lp;
    *fresh8 = ((*ps).sp).offset(1 as usize as isize);
    (*ps)
        .control = (*ps).control
        & !(0o400000 as usize | 0o7000000000
            | 0o700000000 as usize | 0o400
            | 0o20000000 as usize | 0o377 as usize | 0o3000000
            | 0o377000) as libc::c_ulong
        | ((*ps).lp).offset_from((*ps).fp) as libc::c_long as libc::c_ulong
        | ((ValueDispositionEffect) << 10) as libc::c_ulong
        | ((((*ps).fp).offset_from(oldfp) as libc::c_long) << 9)
            as libc::c_ulong;
    (*ps).continuation = (*ps).pc;
    if FetchTrapVectorEntry(index, &mut entry) == 0 {
        return 0;
    }
    if (((*ps).control >> 30
        & (((1) << 2) - 1) as libc::c_ulong)
        as usize as libc::c_uint) < entry.parts.tag >> 6
    {
        (*ps)
            .control = ((entry.parts.tag >> 6
            & (((1) << 2) - 1)
                as libc::c_uint) << 30) as libc::c_ulong
            | (*ps).control
                & !((((1) << 2) - 1)
                    << 30) as libc::c_ulong;
    }
    (*ps).pc = entry;
    return 1;
}
#[no_mangle]
pub  fn TakePostTrap(
    mut index: u32,
    mut arity: u32,
    mut nextpc: *mut LispObj,
) -> usize {
    let mut ps: *mut ProcessorState = processor;
    let mut oldfp: *mut LispObj = (*ps).fp;
    let mut entry: LispObj = LispObj {
        parts: _LispObj {
            tag: 0,
            data: C2RustUnnamed { u: 0 },
        },
    };
    let mut i: usize = 0;
    if ((*ps).sp).offset(8 as usize as isize) > (*ps).StackCacheLimit {
        StackCacheScrollUp();
    }
    i = 0;
    while i < arity {
        *((*ps).sp)
            .offset((4 as usize - i) as isize) = *((*ps).sp).offset(-i as isize);
        i += 1;
    }
    let ref mut fresh9 = (*ps).fp;
    *fresh9 = ((*ps).sp).offset(-((arity - 1) as isize));
    let ref mut fresh10 = (*ps).sp;
    *fresh10 = (*fresh10).offset(4 as usize as isize);
    (*((*ps).fp).offset(0 as usize as isize))
        .parts
        .tag = 0o300 as usize as libc::c_uint | (*ps).continuation.parts.tag;
    (*((*ps).fp).offset(0 as usize as isize))
        .parts
        .data = (*ps).continuation.parts.data;
    (*((*ps).fp).offset(1 as usize as isize))
        .parts
        .tag = (0o300 as usize | TypeFixnum) as ui32;
    (*((*ps).fp).offset(1 as usize as isize))
        .parts
        .data
        .u = (*ps).control as ui32;
    if ((*ps).control >> 29
        & (((1) << 1) - 1) as libc::c_ulong)
        as usize != 0
    {
        (*((*ps).fp).offset(1 as usize as isize))
            .parts
            .data
            .u = ((1
            & ((1) << 1) - 1)
            << 27) as libc::c_uint
            | (*((*ps).fp).offset(1 as usize as isize)).parts.data.u
                & !((((1) << 1) - 1)
                    << 27) as libc::c_uint;
    }
    (*((*ps).fp).offset(2 as usize as isize))
        .parts
        .tag = TypeFixnum as usize as ui32;
    (*((*ps).fp).offset(2 as usize as isize)).parts.data.u = index as ui32;
    (*((*ps).fp).offset(3 as usize as isize))
        .parts
        .tag = (*ps).pc.parts.tag
        & (((1) << 6) - 1) as libc::c_uint;
    (*((*ps).fp).offset(3 as usize as isize)).parts.data = (*ps).pc.parts.data;
    let ref mut fresh11 = (*ps).lp;
    *fresh11 = ((*ps).sp).offset(1 as usize as isize);
    (*ps)
        .control = (*ps).control
        & !(0o400000 as usize | 0o7000000000
            | 0o700000000 as usize | 0o400
            | 0o20000000 as usize | 0o377 as usize | 0o3000000
            | 0o377000) as libc::c_ulong
        | ((*ps).lp).offset_from((*ps).fp) as libc::c_long as libc::c_ulong
        | ((ValueDispositionEffect) << 10) as libc::c_ulong
        | ((((*ps).fp).offset_from(oldfp) as libc::c_long) << 9)
            as libc::c_ulong;
    (*ps).continuation = *nextpc;
    if FetchTrapVectorEntry(index as isize, &mut entry) == 0 {
        return 0;
    }
    if (((*ps).control >> 30
        & (((1) << 2) - 1) as libc::c_ulong)
        as usize as libc::c_uint) < entry.parts.tag >> 6
    {
        (*ps)
            .control = ((entry.parts.tag >> 6
            & (((1) << 2) - 1)
                as libc::c_uint) << 30) as libc::c_ulong
            | (*ps).control
                & !((((1) << 2) - 1)
                    << 30) as libc::c_ulong;
    }
    (*ps).pc = entry;
    return 1;
}
#[no_mangle]
pub  fn TakeInstructionException(
    mut instruction: u32,
    mut op2: *mut LispObj,
    mut nextpc: *mut LispObj,
) -> usize {
    let mut opcode: usize = instruction >> 10
        & ((1) << 8) - 1;
    let mut ei: *const ExceptionInfo = &*InstructionExceptionInfo
        .as_ptr()
        .offset(opcode as isize) as *const ExceptionInfo;
    let mut ps: *mut ProcessorState = processor;
    let mut vector: usize = 0;
    let ref mut fresh12 = (*ps).sp;
    *fresh12 = (*ps).restartsp;
    if (*ei).stackp == 0 {
        if instruction >> 15
            & ((1) << 2) - 1
            == 3
        {
            (*((*ps).sp).offset(1 as usize as isize))
                .parts
                .tag = TypeLocative as usize as ui32;
            (*((*ps).sp).offset(1 as usize as isize))
                .parts
                .data
                .u = ((*ps).StackCacheBase)
                .wrapping_add(
                    op2.offset_from((*ps).StackCache) as libc::c_long as libc::c_ulong,
                ) as ui32;
            let ref mut fresh13 = (*ps).sp;
            *fresh13 = (*fresh13).offset(1);
        } else if instruction >> 0
            & ((1) << 10) - 1
            != 0o1000
        {
            (*((*ps).sp).offset(1 as usize as isize))
                .parts
                .tag = (*op2).parts.tag
                & (((1) << 6) - 1)
                    as libc::c_uint;
            (*((*ps).sp).offset(1 as usize as isize))
                .parts
                .data = (*op2).parts.data;
            let ref mut fresh14 = (*ps).sp;
            *fresh14 = (*fresh14).offset(1);
        }
    }
    if (*ei).arithp == 0 {
        vector = InstructionExceptionVector as usize + opcode;
    } else if (*ei).arity > 1 as usize {
        vector = (ArithmeticInstructionExceptionVector as usize as libc::c_uint)
            .wrapping_add(
                ((opcode & ((1) << 5) - 1)
                    << 6) as libc::c_uint
                    | (((*((*ps).sp).offset(-(1) as isize)).parts.tag
                        & (((1) << 3) - 1)
                            as libc::c_uint) << 3
                        | (*((*ps).sp).offset(0 as usize as isize)).parts.tag
                            & !((((1) << 3)
                                - 1) << 3) as libc::c_uint)
                        & !((((1) << 5) - 1)
                            << 6) as libc::c_uint,
            );
    } else {
        vector = (ArithmeticInstructionExceptionVector as usize as libc::c_uint)
            .wrapping_add(
                ((opcode & ((1) << 5) - 1)
                    << 6) as libc::c_uint
                    | (((*((*ps).sp).offset(0 as usize as isize)).parts.tag
                        & (((1) << 3) - 1)
                            as libc::c_uint) << 3
                        | (0
                            & !((((1) << 3)
                                - 1) << 3)) as libc::c_uint)
                        & !((((1) << 5) - 1)
                            << 6) as libc::c_uint,
            );
    }
    return TakePostTrap(vector, (*ei).arity, nextpc);
}
