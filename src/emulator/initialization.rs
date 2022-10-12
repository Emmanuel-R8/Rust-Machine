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
    pub AluOp: Option<fn() -> isize>,
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


static mut allocatedCaches: Boole = 0 as usize as Boole;
#[no_mangle]
pub static mut run: usize = 1;

static mut ps: ProcessorState = ProcessorState {
    sp: 0 as *const LispObj as *mut LispObj,
    restartsp: 0 as *const LispObj as *mut LispObj,
    fp: 0 as *const LispObj as *mut LispObj,
    lp: 0 as *const LispObj as *mut LispObj,
    pc: LispObj {
        parts: _LispObj {
            tag: 0,
            data: C2RustUnnamed { u: 0 },
        },
    },
    continuation: LispObj {
        parts: _LispObj {
            tag: 0,
            data: C2RustUnnamed { u: 0 },
        },
    },
    InstructionCache: 0 as *const InstructionCacheLine as *mut InstructionCacheLine,
    StackCache: 0 as *const LispObj as *mut LispObj,
    StackCacheLimit: 0 as *const LispObj as *mut LispObj,
    bar: [_bar {
        address: LispObj {
            parts: _LispObj {
                tag: 0,
                data: C2RustUnnamed { u: 0 },
            },
        },
        mapped: 0 as *const LispObj as *mut LispObj,
    }; 4],
    ListCacheArea: LispObj {
        parts: _LispObj {
            tag: 0,
            data: C2RustUnnamed { u: 0 },
        },
    },
    ListCacheAddress: LispObj {
        parts: _LispObj {
            tag: 0,
            data: C2RustUnnamed { u: 0 },
        },
    },
    StructureCacheArea: LispObj {
        parts: _LispObj {
            tag: 0,
            data: C2RustUnnamed { u: 0 },
        },
    },
    StructureCacheAddress: LispObj {
        parts: _LispObj {
            tag: 0,
            data: C2RustUnnamed { u: 0 },
        },
    },
    CatchBlockPointer: LispObj {
        parts: _LispObj {
            tag: 0,
            data: C2RustUnnamed { u: 0 },
        },
    },
    control: 0,
    StackCacheBase: 0,
    ArrayEventCount: 0,
    ListCacheLength: 0,
    StructureCacheLength: 0,
    BindingStackPointer: 0,
    BindingStackLimit: 0,
    DeepBoundP: 0,
    PreemptRegister: 0,
    AluAndRotateControl: 0,
    AluOp: None,
    ByteSize: 0,
    ByteRotate: 0,
    RotateLatch: 0,
    ALUOverflow: 0,
    ALUBorrow: 0,
    ALULessThan: 0,
    EphemeralOldspaceRegister: 0,
    ZoneOldspaceRegister: 0,
    ControlStackLimit: 0,
    ControlStackExtraLimit: 0,
    DynamicBindingCacheBase: 0,
    DynamicBindingCacheMask: 0,
    FEPModeTrapVectorAddress: 0,
    MappingTableCache: 0,
    MappingTableLength: 0,
    running: 0,
    instruction_count: 0,
};

#[no_mangle]
pub static mut processor: *mut ProcessorState =
    unsafe { &ps as *const ProcessorState as *mut ProcessorState };

#[no_mangle]
pub  fn Runningp() -> Boole {
    return run as Boole;
}

#[no_mangle]
pub  fn HaltMachine() {
    if Runningp() != 0 {
        suspend = 1;
    }
}
#[no_mangle]
pub  fn ResetMachine() {}
#[no_mangle]
pub  fn StartMachine(mut resumeP: Boole) {
    ::std::ptr::write_volatile(&mut run as *mut u32, 1);
    suspend = 0;
}
#[no_mangle]
pub  fn IvoryProcessorSystemStartup(mut bootingP: Boole) -> Boole {
    let mut q: LispObj = LispObj {
        parts: _LispObj {
            tag: 0,
            data: C2RustUnnamed { u: 0 },
        },
    };
    if bootingP != 0 {
        InitializeIvoryProcessor(
            MapVirtualAddressData(0 as usize as isize),
            MapVirtualAddressTag(0 as usize as isize),
        );
        if ReadVirtualMemory(0xf8041002 as libc::c_long as isize, &mut q) == 0
            && LispObjTag(q) == TypeCompiledFunction as usize as libc::c_uint
            || ReadVirtualMemory(0xf8041100 as libc::c_long as isize, &mut q) == 0
                && LispObjTag(q) == TypeCompiledFunction as usize as libc::c_uint
        {
            (*((*processor).fp).offset(0 as usize as isize))
                .parts
                .tag = (0xc0 as usize | TypeEvenPC) as ui32;
            (*((*processor).fp).offset(0 as usize as isize))
                .parts
                .data
                .u = LispObjData(q);
        } else {
            return 0 as usize as Boole;
        }
    }
    ResetMachine();
    PopOneFakeFrame();
    PopOneFakeFrame();
    StartMachine(1 as usize as Boole);
    return 1 as usize as Boole;
}
#[no_mangle]
pub  fn PushOneFakeFrame() {
    let mut fp: *mut LispObj = 0 as *mut LispObj;
    fp = ((*processor).sp).offset(1 as usize as isize);
    *fp.offset(0 as usize as isize) = (*processor).continuation;
    let ref mut fresh0 = (*fp.offset(0 as usize as isize)).parts.tag;
    *fresh0 |= 0xc0 as usize as libc::c_uint;
    (*fp.offset(1 as usize as isize)).parts.tag =
        (0xc0 as usize | TypeFixnum) as ui32;
    (*fp.offset(1 as usize as isize)).parts.data.u = (*processor).control as ui32;
    (*processor).control = 0 as usize as isize;
    (*processor).control = ((2
        & ((1) << 8) - 1)
        << 0) as libc::c_ulong
        | (*processor).control
            & !((((1) << 8) - 1) << 0)
                as libc::c_ulong;
    (*processor).control = ((fp.offset_from((*processor).fp) as libc::c_long
        & (((1) << 8) - 1) as libc::c_long)
        << 9) as libc::c_ulong
        | (*processor).control
            & !((((1) << 8) - 1) << 9)
                as libc::c_ulong;
    (*processor).continuation = (*processor).pc;
    let ref mut fresh1 = (*processor).fp;
    *fresh1 = fp;
    let ref mut fresh2 = (*processor).sp;
    *fresh2 = fp.offset(1 as usize as isize);
}
#[no_mangle]
pub  fn PopOneFakeFrame() {
    let mut fp: *mut LispObj = 0 as *mut LispObj;
    fp = (*processor).fp;
    let ref mut fresh3 = (*processor).sp;
    *fresh3 = fp.offset(-(1 as usize as isize));
    let ref mut fresh4 = (*processor).fp;
    *fresh4 = fp.offset(
        -(((*processor).control >> 9
            & (((1) << 8) - 1) as libc::c_ulong)
            as usize as isize),
    );
    (*processor).pc = (*processor).continuation;
    (*processor).continuation = *fp.offset(0 as usize as isize);
    (*processor).control = (*fp.offset(1 as usize as isize)).parts.data.u as isize;
    let ref mut fresh5 = (*processor).lp;
    *fresh5 = ((*processor).fp).offset(
        ((*processor).control >> 0
            & (((1) << 8) - 1) as libc::c_ulong)
            as usize as isize,
    );
}
#[no_mangle]
pub  fn InitializeIvoryProcessor(
    mut dataBase: *mut isize,
    mut tagsBase: *mut Tag,
) {
    let mut p: *mut LispObj = 0 as *mut LispObj;
    let mut q: *mut *mut LispObj = 0 as *mut *mut LispObj;
    let mut i: usize = 0;
    let mut j: usize = 0;
    if allocatedCaches == 0 {
        let ref mut fresh6 = (*processor).InstructionCache;
        *fresh6 = malloc(
            (::std::mem::size_of::<InstructionCacheLine>() as libc::c_ulong)
                .wrapping_mul(2048 as usize as libc::c_ulong),
        ) as *mut InstructionCacheLine;
        if ((*processor).InstructionCache).is_null() {
            OutOfMemory(
                b"Initialize InstructionCache\0" as *const u8 as *const libc::c_char
                    as&str,
                (::std::mem::size_of::<InstructionCacheLine>() as libc::c_ulong)
                    .wrapping_mul(2048 as usize as libc::c_ulong)
                   ,
            );
        }
        let ref mut fresh7 = (*processor).StackCache;
        *fresh7 = malloc(
            (::std::mem::size_of::<LispObj>() as libc::c_ulong)
                .wrapping_mul(0x100 as usize as libc::c_ulong)
                .wrapping_mul(4 as usize as libc::c_ulong),
        ) as *mut LispObj;
        if ((*processor).StackCache).is_null() {
            OutOfMemory(
                b"Initialize StackCache\0" as *const u8 as *const libc::c_char as&str,
                (::std::mem::size_of::<LispObj>() as libc::c_ulong)
                    .wrapping_mul(0x100 as usize as libc::c_ulong)
                    .wrapping_mul(4 as usize as libc::c_ulong),
            );
        }
        let ref mut fresh8 = (*processor).StackCacheLimit;
        *fresh8 = ((*processor).StackCache)
            .offset((0x100 as usize * 4) as isize)
            .offset(-(128 as usize as isize));
        allocatedCaches = 1 as usize as Boole;
    }
    (*processor).running = 0 as usize as Boole;
    (*processor).instruction_count = 0 as usize as libc::c_uint;
    i = 0;
    while i < 2048 as usize {
        (*((*processor).InstructionCache).offset(i as isize))
            .pc
            .parts
            .tag = TypeEvenPC as usize as ui32;
        (*((*processor).InstructionCache).offset(i as isize))
            .pc
            .parts
            .data
            .u = -(1) as ui32;
        (*((*processor).InstructionCache).offset((i + 1) as isize))
            .pc
            .parts
            .tag = TypeOddPC as usize as ui32;
        (*((*processor).InstructionCache).offset((i + 1) as isize))
            .pc
            .parts
            .data
            .u = -(1) as ui32;
        i += 2;
    }
    (*processor).StackCacheBase = 0xf8000100 as libc::c_uint as isize;
    i = 0;
    while i < 0x100 as usize * 4 as usize {
        (*((*processor).StackCache).offset(i as isize)).parts.tag =
            TypeNull as usize as ui32;
        (*((*processor).StackCache).offset(i as isize)).parts.data.u =
            ((*processor).StackCacheBase).wrapping_add(i as libc::c_ulong) as ui32;
        i += 1;
    }
    let ref mut fresh9 = (*processor).fp;
    *fresh9 = ((*processor).StackCache).offset(4 as usize as isize);
    let ref mut fresh10 = (*processor).sp;
    *fresh10 = ((*processor).StackCache).offset(5 as usize as isize);
    let ref mut fresh11 = (*processor).lp;
    *fresh11 = ((*processor).StackCache).offset(6 as usize as isize);
    (*processor).control = 0 as usize as isize;
    (*processor).control = ((2
        & ((1) << 8) - 1)
        << 0) as libc::c_ulong
        | (*processor).control
            & !((((1) << 8) - 1) << 0)
                as libc::c_ulong;
    (*processor).control = ((2
        & ((1) << 8) - 1)
        << 9) as libc::c_ulong
        | (*processor).control
            & !((((1) << 8) - 1) << 9)
                as libc::c_ulong;
    (*processor).control = ((TrapModeFEP
        & ((1) << 2) - 1)
        << 30) as libc::c_ulong
        | (*processor).control
            & !((((1) << 2) - 1) << 30)
                as libc::c_ulong;
    (*processor).pc.parts.tag = (0xc0 as usize | TypeNIL) as ui32;
    (*processor).pc.parts.data.u = 0 as usize as ui32;
    (*processor).continuation = (*processor).pc;
    PushOneFakeFrame();
    PushOneFakeFrame();
    EnsureVirtualAddressRange(
        0xf8000100 as libc::c_uint as isize,
        0xf00,
        0 as usize as Boole,
    );
    EnsureVirtualAddressRange(
        0xf8062000 as libc::c_uint as isize,
        0x9e000,
        0 as usize as Boole,
    );
}
#[no_mangle]
pub  fn OutOfMemory(mut Where:&str, mut HowMuch: u32) {
    fprintf(
        stderr,
        b"%s was unable to allocate %d bytes.\n\0" as *const u8 as *const libc::c_char,
        Where,
        HowMuch,
    );
    exit(-(1));
}
#[no_mangle]
pub  fn ReadInternalRegister(
    mut regno: u32,
    mut val: *mut LispObj,
) -> Boole {
    match regno {
        1 => {
            (*val).parts.tag = TypeLocative as usize as ui32;
            (*val).parts.data.u =
                ((*processor).StackCacheBase)
                    .wrapping_add(((*processor).fp).offset_from((*processor).StackCache)
                        as libc::c_long as libc::c_ulong) as ui32;
        }
        2 => {
            (*val).parts.tag = TypeLocative as usize as ui32;
            (*val).parts.data.u =
                ((*processor).StackCacheBase)
                    .wrapping_add(((*processor).lp).offset_from((*processor).StackCache)
                        as libc::c_long as libc::c_ulong) as ui32;
        }
        3 => {
            (*val).parts.tag = TypeLocative as usize as ui32;
            (*val).parts.data.u =
                ((*processor).StackCacheBase)
                    .wrapping_add(((*processor).sp).offset_from((*processor).StackCache)
                        as libc::c_long as libc::c_ulong) as ui32;
        }
        6 | 134 | 262 | 390 => {
            *val = (*processor).bar[(regno >> 7
                & ((1) << 2) - 1)
                as usize]
                .address;
        }
        10 => {
            *val = (*processor).continuation;
        }
        12 => {
            (*val).parts.tag = TypeFixnum as usize as ui32;
            (*val).parts.data.u = (*processor).control as ui32;
        }
        _ => return 0 as usize as Boole,
    }
    return 1 as usize as Boole;
}
#[no_mangle]
pub  fn WriteInternalRegister(
    mut regno: u32,
    mut val: *mut LispObj,
) -> Boole {
    match regno {
        1 => {
            let ref mut fresh12 = (*processor).fp;
            *fresh12 = ((*processor).StackCache).offset(
                ((*val).parts.data.u as libc::c_ulong).wrapping_sub((*processor).StackCacheBase)
                    as isize,
            );
            while (*processor).fp < (*processor).StackCache {
                StackCacheScrollDown();
            }
            while (*processor).fp > (*processor).StackCacheLimit {
                StackCacheScrollUp();
            }
        }
        3 => {
            let ref mut fresh13 = (*processor).sp;
            *fresh13 = ((*processor).StackCache).offset(
                ((*val).parts.data.u as libc::c_ulong).wrapping_sub((*processor).StackCacheBase)
                    as isize,
            );
        }
        2 => {
            let ref mut fresh14 = (*processor).lp;
            *fresh14 = ((*processor).StackCache).offset(
                ((*val).parts.data.u as libc::c_ulong).wrapping_sub((*processor).StackCacheBase)
                    as isize,
            );
        }
        6 | 134 | 262 | 390 => {
            (*processor).bar[(regno >> 7
                & ((1) << 2) - 1)
                as usize]
                .address = *val;
        }
        10 => {
            (*processor).continuation = *val;
        }
        12 => {
            (*processor).control = (*val).parts.data.u as isize;
        }
        _ => return 0 as usize as Boole,
    }
    return 1 as usize as Boole;
}
