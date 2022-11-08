pub type PC = LispObj;

#[derive(Copy, Clone)]
pub struct InstructionCacheLine {
    pub pc: PC,
    pub next_pc: PC,
    pub code: u32,
    pub operand: u32,
    pub instruction: u32,
    pub next_cp: *mut InstructionCacheLine,
}

#[derive(Copy, Clone, Default)]
pub struct ProcessorState {
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
    pub ListCacheArea: QWord,
    pub ListCacheAddress: QWord,
    pub StructureCacheArea: QWord,
    pub StructureCacheAddress: QWord,
    pub CatchBlockPointer: QWord,
    pub control: u32,
    pub StackCacheBase: u32,
    pub ArrayEventCount: u32,
    pub ListCacheLength: u32,
    pub StructureCacheLength: u32,
    pub BindingStackPointer: u32,
    pub BindingStackLimit: u32,
    pub DeepBoundP: bool,
    pub PreemptRegister: u32,
    pub AluAndRotateControl: u32,
    pub AluOp: Option<fn() -> u32>,
    pub ByteSize: u32,
    pub ByteRotate: u32,
    pub RotateLatch: u32,
    pub ALUOverflow: bool,
    pub ALUBorrow: bool,
    pub ALULessThan: bool,
    pub EphemeralOldspaceRegister: u32,
    pub ZoneOldspaceRegister: u32,
    pub ControlStackLimit: u32,
    pub ControlStackExtraLimit: u32,
    pub DynamicBindingCacheBase: u32,
    pub DynamicBindingCacheMask: u32,
    pub FEPModeTrapVectorAddress: u32,
    pub MappingTableCache: u32,
    pub MappingTableLength: u32,
    pub running: bool,
    pub instruction_count: libc::c_uint,
}
#[derive(Copy, Clone)]

pub struct _bar {
    pub address: QWord,
    pub mapped: *mut LispObj,
}


static mut allocatedCaches: bool = false;

pub static mut run: u32 = 1;

static mut ps: ProcessorState = ProcessorState {
    sp: 0 as *const LispObj as *mut LispObj,
    restartsp: 0 as *const LispObj as *mut LispObj,
    fp: 0 as *const LispObj as *mut LispObj,
    lp: 0 as *const LispObj as *mut LispObj,
    pc: QWord {
        parts: _LispObj {
            tag: 0,
            data: QData { u: 0 },
        },
    },
    continuation: QWord {
        parts: _LispObj {
            tag: 0,
            data: QData { u: 0 },
        },
    },
    InstructionCache: 0 as *const InstructionCacheLine as *mut InstructionCacheLine,
    StackCache: 0 as *const LispObj as *mut LispObj,
    StackCacheLimit: 0 as *const LispObj as *mut LispObj,
    bar: [_bar {
        address: QWord {
            parts: _LispObj {
                tag: 0,
                data: QData { u: 0 },
            },
        },
        mapped: 0 as *const LispObj as *mut LispObj,
    }; 4],
    ListCacheArea: QWord {
        parts: _LispObj {
            tag: 0,
            data: QData { u: 0 },
        },
    },
    ListCacheAddress: QWord {
        parts: _LispObj {
            tag: 0,
            data: QData { u: 0 },
        },
    },
    StructureCacheArea: QWord {
        parts: _LispObj {
            tag: 0,
            data: QData { u: 0 },
        },
    },
    StructureCacheAddress: QWord {
        parts: _LispObj {
            tag: 0,
            data: QData { u: 0 },
        },
    },
    CatchBlockPointer: QWord {
        parts: _LispObj {
            tag: 0,
            data: QData { u: 0 },
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


pub static mut processor: *mut ProcessorState =
    unsafe { &ps as *const ProcessorState as *mut ProcessorState };



// Done
    pub  fn Runningp() -> Boole {
    return run as Boole;
}


// Don
pub  fn HaltMachine() {
    if Runningp() != 0 {
        suspend = 1;
    }
}


// Done
pub  fn ResetMachine() {}

// Done
pub  fn StartMachine(mut resumeP: bool) {
    ::std::ptr::write_volatile(&mut run as *mut u32, 1);
    suspend = 0;
}

pub  fn IvoryProcessorSystemStartup(mut bootingP: bool) -> Boole {
    let mut q: QWord = LispObj {
        parts: _LispObj {
            tag: 0,
            data: QData { u: 0 },
        },
    };
    if bootingP != 0 {
        InitializeIvoryProcessor(
            MapVirtualAddressData(0  ),
            MapVirtualAddressTag(0  ),
        );
        if ReadVirtualMemory(0xf8041002  , &mut q) == 0
            && LispObjTag(q) == TypeCompiledFunction
            || ReadVirtualMemory(0xf8041100  , &mut q) == 0
                && LispObjTag(q) == TypeCompiledFunction
        {
            (*((*processor).fp).offset(0  ))
                .parts
                .tag = (0xc0  | TypeEvenPC) ;
            (*((*processor).fp).offset(0  ))
                .parts
                .data
                .u = LispObjData(q);
        } else {
            return false;
        }
    }
    ResetMachine();
    PopOneFakeFrame();
    PopOneFakeFrame();
    StartMachine(true);
    return true;
}

pub  fn PushOneFakeFrame() {
    let mut fp: *mut LispObj = 0 as *mut LispObj;
    fp = ((*processor).sp).offset(1  );
    *fp.offset(0  ) = (*processor).continuation;
    let ref mut fresh0 = (*fp.offset(0  )).parts.tag;
    *fresh0 |= 0xc0;
    (*fp.offset(1  )).parts.tag =
        (0xc0  | TypeFixnum) ;
    (*fp.offset(1  )).parts.data.u = (*processor).control ;
    (*processor).control = 0  ;
    (*processor).control = ((2
        & ((1) << 8) - 1)
        << 0)
        | (*processor).control
            & !((((1) << 8) - 1) << 0)
              ;
    (*processor).control = ((fp.offset_from((*processor).fp)
        & (((1) << 8) - 1) )
        << 9)
        | (*processor).control
            & !((((1) << 8) - 1) << 9)
              ;
    (*processor).continuation = (*processor).pc;
    let ref mut fresh1 = (*processor).fp;
    *fresh1 = fp;
    let ref mut fresh2 = (*processor).sp;
    *fresh2 = fp.offset(1  );
}

pub  fn PopOneFakeFrame() {
    let mut fp: *mut LispObj = 0 as *mut LispObj;
    fp = (*processor).fp;
    let ref mut fresh3 = (*processor).sp;
    *fresh3 = fp.offset(-(1  ));
    let ref mut fresh4 = (*processor).fp;
    *fresh4 = fp.offset(
        -(((*processor).control >> 9
            & (((1) << 8) - 1))
             ),
    );
    (*processor).pc = (*processor).continuation;
    (*processor).continuation = *fp.offset(0  );
    (*processor).control = (*fp.offset(1  )).parts.data.u ;
    let ref mut fresh5 = (*processor).lp;
    *fresh5 = ((*processor).fp).offset(
        ((*processor).control >> 0
            & (((1) << 8) - 1))
             ,
    );
}

pub  fn initialize_ivory_processor(
    mut dataBase: *mut isize,
    mut tagsBase: *mut Tag,
) {
    let mut p: *mut LispObj = 0 as *mut LispObj;
    let mut q: *mut *mut LispObj = 0 as *mut *mut LispObj;
    let mut i: u32 = 0;
    let mut j: u32 = 0;
    if allocatedCaches == 0 {
        let ref mut fresh6 = (*processor).InstructionCache;
        *fresh6 = malloc(
            (::std::mem::size_of::<InstructionCacheLine>())
                .wrapping_mul(2048),
        ) as *mut InstructionCacheLine;
        if ((*processor).InstructionCache).is_null() {
            OutOfMemory(
                b"Initialize InstructionCache\0"
                    as&str,
                (::std::mem::size_of::<InstructionCacheLine>())
                    .wrapping_mul(2048)
                   ,
            );
        }
        let ref mut fresh7 = (*processor).StackCache;
        *fresh7 = malloc(
            (::std::mem::size_of::<LispObj>())
                .wrapping_mul(0x100)
                .wrapping_mul(4),
        ) as *mut LispObj;
        if ((*processor).StackCache).is_null() {
            OutOfMemory(
                b"Initialize StackCache\0"   as&str,
                (::std::mem::size_of::<LispObj>())
                    .wrapping_mul(0x100)
                    .wrapping_mul(4),
            );
        }
        let ref mut fresh8 = (*processor).StackCacheLimit;
        *fresh8 = ((*processor).StackCache)
            .offset((0x100  * 4) )
            .offset(-(128  ));
        allocatedCaches = true;
    }
    (*processor).running = false;
    (*processor).instruction_count = 0;
    i = 0;
    while i < 2048  {
        (*((*processor).InstructionCache).offset(i ))
            .pc
            .parts
            .tag = TypeEvenPC ;
        (*((*processor).InstructionCache).offset(i ))
            .pc
            .parts
            .data
            .u = -(1) ;
        (*((*processor).InstructionCache).offset((i + 1) ))
            .pc
            .parts
            .tag = TypeOddPC ;
        (*((*processor).InstructionCache).offset((i + 1) ))
            .pc
            .parts
            .data
            .u = -(1) ;
        i += 2;
    }
    (*processor).StackCacheBase = 0xf8000100  ;
    i = 0;
    while i < 0x100  * 4  {
        (*((*processor).StackCache).offset(i )).parts.tag =
            TypeNull ;
        (*((*processor).StackCache).offset(i )).parts.data.u =
            ((*processor).StackCacheBase).wrapping_add(i) ;
        i += 1;
    }
    let ref mut fresh9 = (*processor).fp;
    *fresh9 = ((*processor).StackCache).offset(4  );
    let ref mut fresh10 = (*processor).sp;
    *fresh10 = ((*processor).StackCache).offset(5  );
    let ref mut fresh11 = (*processor).lp;
    *fresh11 = ((*processor).StackCache).offset(6  );
    (*processor).control = 0  ;
    (*processor).control = ((2
        & ((1) << 8) - 1)
        << 0)
        | (*processor).control
            & !((((1) << 8) - 1) << 0)
              ;
    (*processor).control = ((2
        & ((1) << 8) - 1)
        << 9)
        | (*processor).control
            & !((((1) << 8) - 1) << 9)
              ;
    (*processor).control = ((TrapModeFEP
        & ((1) << 2) - 1)
        << 30)
        | (*processor).control
            & !((((1) << 2) - 1) << 30)
              ;
    (*processor).pc.parts.tag = (0xc0  | TypeNIL) ;
    (*processor).pc.parts.data.u = 0 ;
    (*processor).continuation = (*processor).pc;
    PushOneFakeFrame();
    PushOneFakeFrame();
    EnsureVirtualAddressRange(
        0xf8000100  ,
        0xf00,
        false,
    );
    EnsureVirtualAddressRange(
        0xf8062000  ,
        0x9e000,
        false,
    );
}

pub  fn OutOfMemory(mut Where:&str, mut HowMuch: u32) {
    fprintf(
        stderr,
        b"%s was unable to allocate %d bytes.\n\0"  ,
        Where,
        HowMuch,
    );
    exit(-(1));
}

pub  fn ReadInternalRegister(
    mut regno: u32,
    mut val: *mut LispObj,
) -> Boole {
    match regno {
        1 => {
            (*val).parts.tag = TypeLocative ;
            (*val).parts.data.u =
                ((*processor).StackCacheBase)
                    .wrapping_add(((*processor).fp).offset_from((*processor).StackCache)
                        ) ;
        }
        2 => {
            (*val).parts.tag = TypeLocative ;
            (*val).parts.data.u =
                ((*processor).StackCacheBase)
                    .wrapping_add(((*processor).lp).offset_from((*processor).StackCache)
                        ) ;
        }
        3 => {
            (*val).parts.tag = TypeLocative ;
            (*val).parts.data.u =
                ((*processor).StackCacheBase)
                    .wrapping_add(((*processor).sp).offset_from((*processor).StackCache)
                        ) ;
        }
        6 | 134 | 262 | 390 => {
            *val = (*processor).bar[(regno >> 7
                & ((1) << 2) - 1)
                ]
                .address;
        }
        10 => {
            *val = (*processor).continuation;
        }
        12 => {
            (*val).parts.tag = TypeFixnum ;
            (*val).parts.data.u = (*processor).control ;
        }
        _ => return false,
    }
    return true;
}

pub  fn WriteInternalRegister(
    mut regno: u32,
    mut val: *mut LispObj,
) -> Boole {
    match regno {
        1 => {
            let ref mut fresh12 = (*processor).fp;
            *fresh12 = ((*processor).StackCache).offset(
                ((*val).parts.data.u).wrapping_sub((*processor).StackCacheBase)
                    ,
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
                ((*val).parts.data.u).wrapping_sub((*processor).StackCacheBase)
                    ,
            );
        }
        2 => {
            let ref mut fresh14 = (*processor).lp;
            *fresh14 = ((*processor).StackCache).offset(
                ((*val).parts.data.u).wrapping_sub((*processor).StackCacheBase)
                    ,
            );
        }
        6 | 134 | 262 | 390 => {
            (*processor).bar[(regno >> 7
                & ((1) << 2) - 1)
                ]
                .address = *val;
        }
        10 => {
            (*processor).continuation = *val;
        }
        12 => {
            (*processor).control = (*val).parts.data.u ;
        }
        _ => return false,
    }
    return true;
}
