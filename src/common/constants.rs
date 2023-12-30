use num_derive::FromPrimitive;
use num_enum::TryFromPrimitive;
use std::fmt::{ self };

///////////////////////////////////////////////////////////////////////////////////////////////////
//
// TYPES tag - 6 top bits of a Lisp objects
//
///////////////////////////////////////////////////////////////////////////////////////////////////

pub const QTAG_FIXNUM: u8 = 0o10;
pub const QTAG_SINGLEFLOAT: u8 = 0o12;

#[derive(Default, Copy, Clone, Debug, PartialEq, Eq, FromPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum QTag {
    //  Headers  special markers and forwarding pointers.
    #[default] // Default constructor
    Null = 0o_0, //  00 Unbound variable/function
    MonitorForward = 0o__1, //  01 This cell being monitored
    HeaderP = 0o__2, //  02 Structure header
    HeaderI = 0o__3, //  03 Structure header
    ExternalValueCellPointer = 0o__4, //  04 Invisible except for binding
    OneQForward = 0o__5, //  05 Invisible pointer (forwards one cell)
    HeaderForward = 0o__6, //  06 Invisible pointer (forwards whole structure)
    ElementForward = 0o__7, //  07 Invisible pointer in element of structure
    //
    //  Numeric data types. - All have the following bits on/off 000001xxxxxx
    Fixnum = 0o_10, //  10 Small integer
    SmallRatio = 0o_11, //  11 Ratio with small numerator and denominator
    SingleFloat = 0o_12, //  12 SinglePrecision floating point
    DoubleFloat = 0o_13, //  13 DoublePrecision floating point
    Bignum = 0o_14, //  14 Big integer
    BigRatio = 0o_15, //  15 Ratio with big numerator or denominator
    Complex = 0o_16, //  16 Complex number
    SpareNumber = 0o_17, //  17 A number to the hardware trap mechanism

    //  Instance data types.
    Instance = 0o_20, //  20 Ordinary instance
    ListInstance = 0o_21, //  21 Instance that masquerades as a cons
    ArrayInstance = 0o_22, //  22 Instance that masquerades as an array
    StringInstance = 0o_23, //  23 Instance that masquerades as a string

    //  Primitive data types.
    NIL = 0o_24, //  24 The symbol NIL
    List = 0o_25, //  25 A cons
    Array = 0o_26, //  26 An array that is not a string
    String = 0o_27, //  27 A string
    Symbol = 0o_30, //  30 A symbol other than NIL
    Locative = 0o_31, //  31 Locative pointer
    LexicalClosure = 0o_32, //  32 Lexical closure of a function
    DynamicClosure = 0o_33, //  33 Dynamic closure of a function
    CompiledFunction = 0o_34, //  34 Compiled code
    GenericFunction = 0o_35, //  35 Generic function (see later section)
    SparePointer1 = 0o_36, //  36 Spare
    SparePointer2 = 0o_37, //  37 Spare
    PhysicalAddress = 0o_40, //  40 Physical address
    NativeInstruction = 0o_41, //  41 Spare
    BoundLocation = 0o_42, //  42 Deep bound marker
    Character = 0o_43, //  43 Common Lisp character object
    LogicVariable = 0o_44, //  44 Unbound logic variable marker
    GCForward = 0o_45, //  45 ObjectMoved flag for garbage collector
    EvenPC = 0o_46, //  46 PC at first instruction in word
    OddPC = 0o_47, //  47 PC at second instruction in word

    //  FullWord instructions.
    CallCompiledEven = 0o_50, //  50 Start call
    CallCompiledOdd = 0o_51, //  51 Start call
    CallIndirect = 0o_52, //  52 Start call
    CallGeneric = 0o_53, //  53 Start call
    CallCompiledEvenPrefetch = 0o_54, //  54 Like above
    CallCompiledOddPrefetch = 0o_55, //  55 Like above
    CallIndirectPrefetch = 0o_56, //  56 Like above
    CallGenericPrefetch = 0o_57, //  57 Like above

    //  HalfWord (packed) instructions consume 4 bits of data type field (opcodes 60..77).
    PackedInstruction60 = 0o_60,
    TypePackedInstruction61 = 0o_61,
    TypePackedInstruction62 = 0o_62,
    PackedInstruction63 = 0o_63,
    TypePackedInstruction64 = 0o_64,
    TypePackedInstruction65 = 0o_65,
    PackedInstruction66 = 0o_66,
    TypePackedInstruction67 = 0o_67,
    TypePackedInstruction70 = 0o_70,
    PackedInstruction71 = 0o_71,
    TypePackedInstruction72 = 0o_72,
    TypePackedInstruction73 = 0o_73,
    PackedInstruction74 = 0o_74,
    TypePackedInstruction75 = 0o_75,
    TypePackedInstruction76 = 0o_76,
    PackedInstruction77 = 0o_77,

    //
    TagCdrMask = 0o300,
}

impl fmt::Display for QTag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////
//
// CDR tag for data
//
///////////////////////////////////////////////////////////////////////////////////////////////////
pub const CDR_NEXT: u8 = 0;
pub const CDR_ILLEGAL: u8 = 1;
pub const CDR_NORMAL: u8 = 2;
pub const CDR_JUMP: u8 = 3;
pub const CDR_DEFAULT: u8 = CDR_JUMP;

#[derive(Default, Copy, Clone, Debug, PartialEq, Eq, FromPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum CDR {
    Next = 0,
    Illegal = 1,
    Normal = 2,
    #[default]
    Jump = 3,
}

impl fmt::Display for CDR {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////
//
// Sequencing tag for instructions (replaces CDR)
// IMAS p74.
//
///////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Default, Copy, Clone, Debug, PartialEq, Eq, FromPrimitive)]
pub enum SEQ {
    #[default]
    Normal = 0, // PC +1 Normal half-word sequencing
    Fence = 1, // Marks end ofcompiled function
    Back = 2, // PC - 1 On some constants
    NextWord = 3, // PC even + 2, PC odd + 3 Before or on some constants
}

impl fmt::Display for SEQ {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////
//
// ARRAY element
//
///////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Default, Copy, Clone, Debug, PartialEq, Eq, FromPrimitive)]
pub enum ArrayElement {
    #[default] // Default constructor
    Fixnum = 0,
    Character = 1,
    Boole = 2,
    Object = 3,
}

impl fmt::Display for ArrayElement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////
//
// ARRAY bit fields
//
///////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Default, Copy, Clone, Debug, PartialEq, Eq, FromPrimitive)]
pub enum ArrayElementType {
    #[default] // Default constructor
    Fixnum = 0,
    Character = 1,
    Boole = 2,
    Object = 3,
}

impl fmt::Display for ArrayElementType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

// see aihead.h
// 32 bits = 0o37_777_777_777
// The top 6 bits describes the type of array
// Format of those 6 bits

// Type field mask
pub const ARRAY_TYPE_FIELD_MASK: u32 = 0b111111_000_00000000_00_0000000000_000;
pub const ARRAY_ELEMENT_TYPE_MASK: u32 = 0b110000_000_00000000_00_0000000000_000;
pub const ARRAY_BYTE_PACKING_MASK: u32 = 0b001110_000_00000000_00_0000000000_000;
pub const ARRAY_LIST_BIT_MASK: u32 = 0b000001_000_00000000_00_0000000000_000;

//
pub const ARRAY_NAMED_STRUCTURE_BIT_MASK: u32 = 0b000000_100_00000000_00_0000000000_000;
pub const ARRAY_SPARE1_MASK: u32 = 0b000000_010_00000000_00_0000000000_000;
pub const ARRAY_LONG_PREFIX_BIT_MASK: u32 = 0b000000_001_00000000_00_0000000000_000;

// Length field mask
pub const ARRAY_LENGTH_MASK: u32 = 0b000000_000_00000000_11_1111111111_111;
pub const ARRAY_LEADER_LENGTH_FIELD_MASK: u32 = 0b000000_000_11111111_00_0000000000_000;
pub const ARRAY_LONG_DIMENSIONS_FIELD_MASK: u32 = 0b000000_000_00000000_00_0000000000_111;
pub const ARRAY_LONG_SPARE_MASK: u32 = 0b000000_000_00000000_11_1111111111_000;
pub const ARRAY_DISCONTIGUOUS_BIT_MASK: u32 = 0b000000_000_00000000_01_0000000000_000;
pub const ARRAY_DISPLACED_BIT_MASK: u32 = 0b000000_000_00000000_10_0000000000_000;

// Register masks
pub const ARRAY_REGISTER_ELEMENT_TYPE_MASK: u32 = 0b110000_000_00000000_00_0000000000_000;
pub const ARRAY_REGISTER_BYTE_PACKING_MASK: u32 = 0b001110_000_00000000_00_0000000000_000;
pub const ARRAY_REGISTER_BYTE_OFFSET_MASK: u32 = 0b000001_111_10000000_00_0000000000_000;
pub const ARRAY_REGISTER_EVENT_COUNT_MASK: u32 = 0b000000_000_01111111_11_1111111111_111;

///////////////////////////////////////////////////////////////////////////////////////////////////
//
// FIXME What are those?
//
///////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Default, Copy, Clone, Debug, PartialEq, Eq, FromPrimitive)]
pub enum ValueDisposition {
    #[default] // Default constructor
    Effect = 0,
    Value = 1,
    Return = 2,
    Multiple = 3,
}

impl fmt::Display for ValueDisposition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////
//
// Instructions opcode - FIXME Should be replaced by macros to include string representation, disassembly, code
//
///////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Default, Copy, Clone, Debug, PartialEq, Eq, FromPrimitive)]
pub enum OpCode {
    // List manipulation
    Car = 0o__0,
    Cdr = 0o__1,
    SetToCar = 0o140,
    SetToCdr = 0o141,
    SetToCdrPushCar = 0o142,
    Rplaca = 0o200,
    Rplacd = 0o201,
    Rgetf = 0o225,
    Member = 0o226,
    Assoc = 0o227,

    // AI Instructions
    Dereference = 0o_13,
    LogicTailTest = 0o_14,
    PushGlobalLogicVariable = 0o_55,
    PushLocalLogicVariables = 0o103,
    Unify = 0o237,

    // Binary predicates
    Eq = 0o270,
    EqNoPop = 0o274,
    Eql = 0o263,
    EqlNoPop = 0o267,
    EqualNumber = 0o260,
    EqualNumberNoPop = 0o264,
    Greaterp = 0o262,
    GreaterpNoPop = 0o266,
    Lessp = 0o261,
    LesspNoPop = 0o265,
    Logtest = 0o273,
    LogtestNoPop = 0o277,
    TypeMember1 = 0o_40,
    TypeMember2 = 0o_41,
    TypeMember3 = 0o_42,
    TypeMember4 = 0o_43,
    TypeMember1NoPop = 0o_44,
    TypeMember2NoPop = 0o_45,
    TypeMember3NoPop = 0o_46,
    TypeMember4NoPop = 0o_47,

    // Unary predicates
    Endp = 0o__2,
    Plusp = 0o_36,
    Minusp = 0o_35,
    Zerop = 0o_34,

    // Numeric operations
    Add = 0o300,
    Sub = 0o301,
    UnaryMinus = 0o114,
    Increment = 0o143,
    Decrement = 0o144,
    Multiply = 0o202,
    Quotient = 0o203,
    Ceiling = 0o204,
    Floor = 0o205,
    Truncate = 0o206,
    Round = 0o207,
    RationalQuotient = 0o211,
    Max = 0o213,
    Min = 0o212,
    Logand = 0o215,
    Logior = 0o217,
    Logxor = 0o216,
    Ash = 0o232,
    Rot = 0o220,
    Lsh = 0o221,
    Op32BitPlus = 0o302,
    Op32BitDifference = 0o303,
    MultiplyDouble = 0o222,
    AddBignumStep = 0o304,
    SubBignumStep = 0o305,
    MultiplyBignumStep = 0o306,
    DivideBignumStep = 0o307,
    LshcBignumStep = 0o223,

    // Data movement
    Push = 0o100,
    Pop = 0o340,
    Movem = 0o341,
    PushNNils = 0o101,
    PushAddress = 0o150,
    SetSpToAddress = 0o151,
    SetSpToAddressSaveTos = 0o152,
    PushAddressSpRelative = 0o102,
    StackBlt = 0o224,
    StackBltAddress = 0o352,

    // FieldExtraction instructions
    Ldb = 0o170,
    Dpb = 0o370,
    CharLdb = 0o171,
    CharDpb = 0o371,
    PLdb = 0o172,
    PDpb = 0o372,
    PTagLdb = 0o173,
    PTagDpb = 0o373,

    // Array operations
    Aref1 = 0o312,
    Aset1 = 0o310,
    Aloc1 = 0o313,
    Setup1DArray = 0o__3,
    SetupForce1DArray = 0o__4,
    FastAref1 = 0o350,
    FastAset1 = 0o351,
    ArrayLeader = 0o316,
    StoreArrayLeader = 0o314,
    AlocLeader = 0o317,

    // Branch instructions
    Branch = 0o174,
    BranchTrue = 0o_60,
    BranchTrueElseExtraPop = 0o_61,
    BranchTrueAndExtraPop = 0o_62,
    BranchTrueExtraPop = 0o_63,
    BranchTrueNoPop = 0o_64,
    BranchTrueAndNoPop = 0o_65,
    BranchTrueElseNoPop = 0o_66,
    BranchTrueAndNoPopElseNoPopExtraPop = 0o_67,
    BranchFalse = 0o_70,
    BranchFalseElseExtraPop = 0o_71,
    BranchFalseAndExtraPop = 0o_72,
    BranchFalseExtraPop = 0o_73,
    BranchFalseNoPop = 0o_74,
    BranchFalseAndNoPop = 0o_75,
    BranchFalseElseNoPop = 0o_76,
    BranchFalseAndNoPopElseNoPopExtraPop = 0o_77,
    LoopDecrementTos = 0o175,
    LoopIncrementTosLessThan = 0o375,

    // Block instructions
    Block0Read = 0o120,
    Block1Read = 0o121,
    Block2Read = 0o122,
    Block3Read = 0o123,
    Block0ReadShift = 0o124,
    Block1ReadShift = 0o125,
    Block2ReadShift = 0o126,
    Block3ReadShift = 0o127,
    Block0ReadAlu = 0o160,
    Block1ReadAlu = 0o161,
    Block2ReadAlu = 0o162,
    Block3ReadAlu = 0o163,
    Block0ReadTest = 0o130,
    Block1ReadTest = 0o131,
    Block2ReadTest = 0o132,
    Block3ReadTest = 0o133,
    Block0Write = 0o_30,
    Block1Write = 0o_31,
    Block2Write = 0o_32,
    Block3Write = 0o_33,

    // Function calling
    StartCall = 0o_10,
    FinishCallN = 0o134,
    FinishCallNApply = 0o135,
    FinishCallTos = 0o136,
    FinishCallTosApply = 0o137,
    EntryRestAccepted = 0o176,
    EntryRestNotAccepted = 0o177,
    LocateLocals = 0o_50,
    ReturnSingle = 0o115,
    ReturnMultiple = 0o104,
    ReturnKludge = 0o105,
    TakeValues = 0o106,

    // Binding instructions
    BindLocativeToValue = 0o236,
    BindLocative = 0o__5,
    UnbindN = 0o107,
    RestoreBindingStack = 0o__6,

    // Catch
    CatchOpen = 0o376,
    CatchClose = 0o_51,

    // Lexical variables
    PushLexicalVar0 = 0o_20,
    PushLexicalVar1 = 0o_21,
    PushLexicalVar2 = 0o_22,
    PushLexicalVar3 = 0o_23,
    PushLexicalVar4 = 0o_24,
    PushLexicalVar5 = 0o_25,
    PushLexicalVar6 = 0o_26,
    PushLexicalVar7 = 0o_27,
    PopLexicalVar0 = 0o240,
    PopLexicalVar1 = 0o241,
    PopLexicalVar2 = 0o242,
    PopLexicalVar3 = 0o243,
    PopLexicalVar4 = 0o244,
    PopLexicalVar5 = 0o245,
    PopLexicalVar6 = 0o246,
    PopLexicalVar7 = 0o247,
    MovemLexicalVar0 = 0o250,
    MovemLexicalVar1 = 0o251,
    MovemLexicalVar2 = 0o252,
    MovemLexicalVar3 = 0o253,
    MovemLexicalVar4 = 0o254,
    MovemLexicalVar5 = 0o255,
    MovemLexicalVar6 = 0o256,
    MovemLexicalVar7 = 0o257,

    // Instance variables
    PushInstanceVariable = 0o110,
    PopInstanceVariable = 0o320,
    MovemInstanceVariable = 0o321,
    PushAddressInstanceVariable = 0o111,
    PushInstanceVariableOrdered = 0o112,
    PopInstanceVariableOrdered = 0o322,
    MovemInstanceVariableOrdered = 0o323,
    PushAddressInstanceVariableOrdered = 0o113,
    InstanceRef = 0o324,
    InstanceSet = 0o325,
    InstanceLoc = 0o326,

    // Sub-primitives
    Ephemeralp = 0o__7,
    UnsignedLessp = 0o331,
    UnsignedLesspNoPop = 0o335,
    Alu = 0o214,
    AllocateListBlock = 0o311,
    AllocateStructureBlock = 0o315,
    PointerPlus = 0o230,
    PointerDifference = 0o231,
    PointerIncrement = 0o145,

    // Read/Write
    ReadInternalRegister = 0o154,
    WriteInternalRegister = 0o155,
    CoprocessorRead = 0o156,
    CoprocessorWrite = 0o157,
    MemoryRead = 0o116,
    MemoryReadAddress = 0o117,
    Tag = 0o_12,
    SetTag = 0o327,
    StoreConditional = 0o233,
    MemoryWrite = 0o234,
    PStoreContents = 0o235,
    SetCdrCode1 = 0o146,
    SetCdrCode2 = 0o147,
    MergeCdrNoPop = 0o342,
    GenericDispatch = 0o_52,
    MessageDispatch = 0o_53,

    // Other
    Jump = 0o_11,
    CheckPreemptRequest = 0o_54,
    #[default] // Default constructor
    NoOp = 0o_56,
    Halt = 0o_57,
}

impl fmt::Display for OpCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////
//
// CPU internal registers
//
///////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Default, Copy, Clone, Debug, PartialEq, Eq, FromPrimitive)]
pub enum InternalRegister {
    BAR0 = 0o___6,
    PHTHash0 = 0o___7,
    InvalidateMap0 = 0o__32,
    LoadMap0 = 0o__33,
    LoadBAR0 = 0o__52,
    //
    BAR1 = 0o_206,
    PHTHash1 = 0o_207,
    InvalidateMap1 = 0o_232,
    LoadMap1 = 0o_233,
    LoadBAR1 = 0o_252,
    //
    BAR2 = 0o_406,
    PHTHash2 = 0o_407,
    InvalidateMap2 = 0o_432,
    LoadMap2 = 0o_433,
    LoadBAR2 = 0o_452,
    //
    BAR3 = 0o_606,
    PHTHash3 = 0o_607,
    InvalidateMap3 = 0o_632,
    LoadMap3 = 0o_633,
    LoadBAR3 = 0o_652,
    //
    EA = 0o___0,
    FP = 0o___1,
    LP = 0o___2,
    SP = 0o___3,
    MacroSP = 0o___4,
    StackCacheLowerBound = 0o___5,
    EPC = 0o__10,
    DPC = 0o__11,
    Continuation = 0o__12,
    AluAndRotateControl = 0o__13,
    ControlRegister = 0o__14,
    CRArgumentSize = 0o__15,
    EphemeralOldspaceRegister = 0o__16,
    ZoneOldspaceRegister = 0o__17,
    ChipRevision = 0o__20,
    FPCoprocessorPresent = 0o__21,
    PreemptRegister = 0o__23,
    IcacheControl = 0o__24,
    PrefetcherControl = 0o__25,
    MapCacheControl = 0o__26,
    MemoryControl = 0o__27,
    ECCLog = 0o__30,
    ECCLogAddress = 0o__31,
    StackCacheOverflowLimit = 0o__34,
    UcodeROMContents = 0o__35,
    AddressMask = 0o__37,
    EntryMaximumArguments = 0o__40,
    LexicalVariable = 0o__41,
    Instruction = 0o__42,
    MemoryData = 0o__44,
    DataPins = 0o__45,
    ExtensionRegister = 0o__46,
    MicrosecondClock = 0o__47,
    ArrayHeaderLength = 0o__50,
    TOS = 0o1000,
    ArrayEventCount = 0o1001,
    BindingStackPointer = 0o1002,
    CatchBlockList = 0o1003,
    ControlStackLimit = 0o1004,
    ControlStackExtraLimit = 0o1005,
    BindingStackLimit = 0o1006,
    PHTBase = 0o1007,
    PHTMask = 0o1010,
    CountMapReloads = 0o1011,
    ListCacheArea = 0o1012,
    ListCacheAddress = 0o1013,
    ListCacheLength = 0o1014,
    StructureCacheArea = 0o1015,
    StructureCacheAddress = 0o1016,
    StructureCacheLength = 0o1017,
    DynamicBindingCacheBase = 0o1020,
    DynamicBindingCacheMask = 0o1021,
    ChoicePointer = 0o1022,
    StructureStackChoicePointer = 0o1023,
    FEPModeTrapVectorAddress = 0o1024,
    MappingTableCache = 0o1026,
    MappingTableLength = 0o1027,
    StackFrameMaximumSize = 0o1030,
    StackCacheDumpQuantum = 0o1031,
    #[default]
    ConstantNIL = 0o1040,
    ConstantT = 0o1041,
}

impl fmt::Display for InternalRegister {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////
//
// Coprocessor registers
//
///////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Default, Copy, Clone, Debug, PartialEq, Eq, FromPrimitive)]
pub enum CoprocessorRegister {
    #[default]
    MicrosecondClock = 0o1_002,
    HostInterrupt = 0o1_010,
    VMRegisterCommand = 0o1_100,
    VMRegisterAddress = 0o1_101,
    VMRegisterExtent = 0o1_102,
    VMRegisterAttributes = 0o1_103,
    VMRegisterDestination = 0o1_104,
    VMRegisterData = 0o1_105,
    VMRegisterMaskLow = 0o1_106,
    VMRegisterMaskHigh = 0o1_107,
    VMRegisterCommandBlock = 0o1_110,
    StackSwitch = 640,
    FlushStackCache = 641,
    FlushIDCaches = 642,
    CalendarClock = 643,
    FlushCachesForVMA = 644,
    FlipToStack = 645,
    UnwindStackForRestartOrApply = 646,
    SaveWorld = 647,
    ConsoleInputAvailableP = 648,
    WaitForEvent = 649,
    FlushHiddenArrayRegisters = 650,
    ConsoleIO = 651,
    AttachDiskChannel = 652,
    GrowDiskPartition = 653,
    DetachDiskChannel = 654,
}

impl fmt::Display for CoprocessorRegister {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////
//
// Arithmetic / logical unit condition constants
//
///////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Default, Copy, Clone, Debug, PartialEq, Eq, FromPrimitive)]
pub enum ALUCondition {
    SignedLessThanOrEqual = 0,
    SignedLessThan = 1,
    Negative = 2,
    SignedOverflow = 3,
    UnsignedLessThanOrEqual = 4,
    UnsignedLessThan = 5,
    #[default]
    Zero = 6,
    High25Zero = 7,
    Eq = 8,
    Op1Ephemeralp = 9,
    Op1TypeAcceptable = 10,
    Op1TypeCondition = 11,
    ResultTypeNil = 12,
    Op2Fixnum = 13,
    False = 14,
    ResultCdrLow = 15,
    CleanupBitsSet = 16,
    AddressInStackCache = 17,
    PendingSequenceBreakEnabled = 18,
    ExtraStackMode = 19,
    FepMode = 20,
    FpCoprocessorPresent = 21,
    Op1Oldspacep = 22,
    StackCacheOverflow = 23,
    OrLogicVariable = 24,
}

impl fmt::Display for ALUCondition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

#[derive(Default, Copy, Clone, Debug, PartialEq, Eq, FromPrimitive)]
pub enum ALUAdderOP2 {
    Op2 = 0,
    #[default]
    Zero = 1,
    Invert = 2,
    MinusOne = 3,
}

impl fmt::Display for ALUAdderOP2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

#[derive(Default, Copy, Clone, Debug, PartialEq, Eq, FromPrimitive)]
pub enum ALUByteFunction {
    #[default]
    DPB = 0,
    LDB = 1,
}

impl fmt::Display for ALUByteFunction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

#[derive(Default, Copy, Clone, Debug, PartialEq, Eq, FromPrimitive)]
pub enum ALUByteBackground {
    #[default]
    Op1 = 0,
    RotateLatch = 1,
    Zero = 2,
}

impl fmt::Display for ALUByteBackground {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

#[derive(Default, Copy, Clone, Debug, PartialEq, Eq, FromPrimitive)]
pub enum Boole {
    #[default]
    Clear = 0,
    And = 1,
    AndC1 = 2,
    Boole2 = 3,
    AndC2 = 4,
    Boole1 = 5,
    Xor = 6,
    Ior = 7,
    Nor = 8,
    Equiv = 9,
    C1 = 10,
    OrC1 = 11,
    C2 = 12,
    OrC2 = 13,
    Nand = 14,
    Set = 15,
}

impl fmt::Display for Boole {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

#[derive(Default, Copy, Clone, Debug, PartialEq, Eq, FromPrimitive)]
pub enum ALUFunction {
    #[default]
    Boole = 0,
    Byte = 1,
    Adder = 2,
    MultiplyDivide = 3,
}

impl fmt::Display for ALUFunction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////
//
// Exceptions
//
///////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Default, Copy, Clone, Debug, PartialEq, Eq, FromPrimitive)]
pub enum TrapReason {
    #[default]
    HighPrioritySequenceBreak = 1,
    LowPrioritySequenceBreak = 2,
}

impl fmt::Display for TrapReason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

#[derive(Default, Copy, Clone, Debug, PartialEq, Eq, FromPrimitive)]
pub enum TrapMode {
    #[default]
    Emulator = 0,
    ExtraStack = 1,
    IO = 2,
    FEP = 3,
}

impl fmt::Display for TrapMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

#[derive(Default, Copy, Clone, Debug, PartialEq, Eq, FromPrimitive)]
pub enum TrapVectors {
    #[default]
    ArithmeticInstructionException = 0o____0,
    InstructionException = 0o4_000, // 2048
    InterpreterFunction = 0o4_400, // 2304
    GenericDispatchMask = 0o5_000, // 2560

    // Trpps
    Error = 0o5_100, // 2624
    Reset = 0o5_101, // 2625
    PullApplyArgs = 0o5_102, // 2626
    StackOverflow = 0o5_103, // 2627
    Trace = 0o5_104, // 2628
    PreemptRequest = 0o5_105, // 2629
    Transport = 0o5_106, // 2630
    FepMode = 0o5_107, // 2631
    LowPrioritySequenceBreak = 0o5_110, // 2632
    HighPrioritySequenceBreak = 0o5_111, // 2633
    Monitor = 0o5_112, // 2634
    GenericDispatch = 0o5_114, // 2636
    MessageDispatch = 0o5_116, // 2638
    PageNotResident = 0o5_120, // 2640
    PageFaultRequest = 0o5_121, // 2641
    PageWriteFault = 0o5_122, // 2642
    UncorrectableMemoryError = 0o5_123, // 2643
    MemoryBusError = 0o5_124, // 2644
    DBCacheMiss = 0o5_125, // 2645
    DBUnwindFrame = 0o5_126, // 2646
    DBUnwindCatch = 0o5_127, // 2647
}

impl fmt::Display for TrapVectors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////
//
// Exceptions
//
///////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Default, Copy, Clone, Debug, PartialEq, Eq, FromPrimitive)]
pub enum ReturnValue {
    #[default]
    Normal = 0,
    Exception = 1,
    IllegalOperand = 2,
}

impl fmt::Display for ReturnValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

#[derive(Default, Copy, Clone, Debug, PartialEq, Eq, FromPrimitive)]
pub enum HaltReason {
    #[default]
    IllegalInstruction = 1,
    Halted = 2,
    SpyCalled = 3,
    FatalStackOverflow = 4,
    IllegalTrapVector = 5,
}

impl fmt::Display for HaltReason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

#[derive(Default, Copy, Clone, Debug, PartialEq, Eq, FromPrimitive)]
pub enum DoubleFloatOp {
    #[default]
    Add = 0,
    Sub = 1,
    Multiply = 2,
    Divide = 3,
}

impl fmt::Display for DoubleFloatOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

pub type VMAttribute = u8;
pub const VMATTRIBUTE_EMPTY: VMAttribute = 0o__0; // 0000_0000
pub const VMATTRIBUTE_ACCESS_FAULT: VMAttribute = 0o__1; // 0000_0001
pub const VMATTRIBUTE_WRITE_FAULT: VMAttribute = 0o__2; // 0000_0010
pub const VMATTRIBUTE_TRANSPORT_FAULT: VMAttribute = 0o__4; // 0000_0100
pub const VMATTRIBUTE_TRANSPORT_DISABLE: VMAttribute = 0o_10; // 0000_1000
pub const VMATTRIBUTE_EPHEMERAL: VMAttribute = 0o_20; // 0001_0000
pub const VMATTRIBUTE_MODIFIED: VMAttribute = 0o_40; // 0010_0000
pub const VMATTRIBUTE_EXISTS: VMAttribute = 0o100; // 0100_0000
pub const VMATTRIBUTE_CREATED_DEFAULT: VMAttribute = 0b01_000_101; // AccessFault | TransportFault | Exists,

// Protections are chosen from these bits, OR'd together.  The implementation does not necessarily
// support PROT_EXEC or PROT_WRITE without PROT_READ.  The only guarantees are that no writing will
// be allowed without PROT_WRITE and no access will be allowed for PROT_NONE.
pub const PROT_NONE: u32 = 0x_________0; // Page can not be accessed.
pub const PROT_READ: u32 = 0x_________1; // Page can be read.
pub const PROT_WRITE: u32 = 0x_________2; // Page can be written.
pub const PROT_EXEC: u32 = 0x_________4; // Page can be executed.
pub const PROT_GROWSDOWN: u32 = 0x_100_0000; // Extend change to start of growsdown vma (mprotect only).
pub const PROT_GROWSUP: u32 = 0x_2000000; // Extend change to start of growsup vma (mprotect only).

// Possible world file formats */
#[derive(Default, Copy, Clone, Debug, PartialEq, Eq, FromPrimitive)]
pub enum LoadFileFormat {
    #[default]
    VLM, // VLM world file (.VLOD)
    Ivory, // Ivory world file (.ILOD)
}

impl fmt::Display for LoadFileFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

// Common world format format definitions
pub const VERSION_AND_ARCHITECTURE_Q: u32 = 0;

// // VLM world file format definitions
pub const VLMWORLD_SUFFIX: &str = ".vlod";

// The SWAPPED version is the one found in the images typically found on the internet.
pub const VLMWORLD_FILE_COOKIE_SWAPPED: u32 = 0x88_89_8a_a3; // 0o21_042_305_243
pub const VLMWORLD_FILE_COOKIE: u32 = 0xa3_8a_89_88; // 0o24_342_504_610
pub const VLMBLOCK_SIZE: u32 = 0x2_000; // 8192, 0o20_000
pub const VLMBLOCKS_PER_DATA_PAGE: u32 = 4;
pub const VLMBLOCKS_PER_TAGS_PAGE: u32 = 1; // tag+`cdr` = 1 byte
pub const VLMMAXIMUM_HEADER_BLOCKS: u32 = 14;
pub const VLMPAGE_SIZE_QS: u32 = 0x2_000; // 8192, 0o20_000
pub const VLMDATA_PAGE_SIZE_BYTES: u32 = 4 * VLMPAGE_SIZE_QS;
pub const VLMTAGS_PAGE_SIZE_BYTES: u32 = 1 * VLMPAGE_SIZE_QS;

pub const VLMVERSION1_AND_ARCHITECTURE: u32 = 0x80_00_80; // 0o40_000_200
pub const VLMWORLD_FILE_V1_UNWIRED_COUNT_Q: u32 = 0;
pub const VLMWORLD_FILE_V1_WIRED_COUNT_Q: u32 = 1;
pub const VLMWORLD_FILE_V1_PAGE_BASES_Q: u32 = 3;
pub const VLMWORLD_FILE_V1_FIRST_SYSOUT_Q: u32 = 0;
pub const VLMWORLD_FILE_V1_FIRST_MAP_Q: u32 = 8;

// The version 2 is the one found in the images typically found on the internet.
// (0x80_00_81 being 0x00_80_00_81 coded as 0x81_00_80_00 swapped on disk)
pub const VLMVERSION2_AND_ARCHITECTURE: u32 = 0x80_00_81; // 0o40_000_201
pub const VLMWORLD_FILE_V2_UNWIRED_COUNT_Q: u32 = 0;
pub const VLMWORLD_FILE_V2_WIRED_COUNT_Q: u32 = 1;
pub const VLMWORLD_FILE_V2_PAGE_BASES_Q: u32 = 2;
pub const VLMWORLD_FILE_V2_FIRST_SYSOUT_Q: u32 = 3;
pub const VLMWORLD_FILE_V2_FIRST_MAP_Q: u32 = 8;

// // Ivory world file format definitions
pub const IVORY_WORLD_SUFFIX: &str = ".ilod";

// #if BYTE_ORDER == LITTLE_ENDIAN
pub const IVORY_WORLD_FILE_COOKIE: u32 = 0x63_4a_49_48; // 0o14_322_444_510;
// #else
pub const IVORY_WORLD_FILE_COOKIE_SWAPPED: u32 = 0x48_49_4a_63;
// pub const  IvoryWorldFileCookie  : u32 = 0x48_49_4A_63; // 011022245143L
// #endif
pub const IVORY_PAGE_SIZE_QS: u32 = 0x100;
pub const IVORY_PAGE_SIZE_BYTES: u32 = 0x100 + 4 * 0x100; // 1280 bytes. For each Q, 1 byte for tag+cdr, 4 bytes for the data
pub const IVORY_WORLD_FILE_WIRED_COUNT_Q: u32 = 1;
pub const IVORY_WORLD_FILE_UNWIRED_COUNT_Q: u32 = 2;
pub const IVORY_WORLD_FILE_FIRST_SYSOUT_Q: u32 = 0;
pub const IVORY_WORLD_FILE_FIRST_MAP_Q: u32 = 8;

pub const IVORY_STACK_CACHE_SIZE: u32 = 0x4;

// size reflects 'count from 0' array indices
// Memory_TotalSize*: QAddress = bottomMask(0)

// Page size is 13 bits = 0x2000 = 8,192
pub const MEMORY_ADDRESS_PAGE_SHIFT: u32 = 13;
pub const MEMORY_PAGE_SIZE: u32 = 1 << MEMORY_ADDRESS_PAGE_SHIFT;
pub const MEMORY_PAGE_MASK: u32 = MEMORY_PAGE_SIZE - 1;
//  MemoryPage_Total* : QAddress = (2 ^ (VMArchitecture_In_Bits - MemoryPage_AddressShift)).QAddress

// Wads are clusters of pages for swap contiguity.  The current value is
// chosen so that all the attributes of a wad fit in one long
// Note that MemoryWad_AddressShift = MemoryPage_AddressShift + 3
pub const MEMORYWAD_ADDRESS_SHIFT: u32 = MEMORY_ADDRESS_PAGE_SHIFT + 3;
pub const MEMORYWAD_SIZE: u32 = 1 << MEMORYWAD_ADDRESS_SHIFT;
pub const MEMORYWAD_MASK: u32 = MEMORYWAD_SIZE - 1;

pub const ADDRESS_TRAPVECBASE: u32 = 0xf8_04_00_00;
pub const MEMORY_STACK_CACHE_BASE: u32 = 0xf8_00_01_00;

///////////////////////////////////////////////////////////////////////////////////
//
// Genera version of FEP Communications area
//
pub const ADDRESS_FEPCOMM_AREA: u32 = 0xf8_04_10_00;
pub const FEPCOMM_AREA_SIZE: u32 = 256;
pub const ADDRESS_FEPMODETRAPVECADDRESS: u32 = 0xf8_04_0a47;
// pub const WADEXISTSMASK: u32 = 0x4040_4040_4040_4040;

#[derive(Default, Copy, Clone, Debug, PartialEq, Eq, FromPrimitive)]
#[repr(u32)]
pub enum FEPCommArea {
    #[default]
    FepVersionNumber = ADDRESS_FEPCOMM_AREA,
    SystemType = 0xf8_04_10_04,
    FepStartup = 0xf8_04_10_08,
    SpyCommand = 0xf8_04_10_0c, // Obsolete
    SpyStatus = 0xf8_04_10_10, // Obsolete
    SpyPc = 0xf8_04_10_14, // Obsolete
    LoadMapSize = 0xf8_04_10_18,
    LoadMapVmaaddress = 0xf8_04_10_1c,
    LoadMapOpcodeAddress = 0xf8_04_10_20,
    LoadMapOperandAddress = 0xf8_04_10_24,
    SwapMapSize = 0xf8_04_10_28,
    SwapMapAddress = 0xf8_04_10_2c,
    SwapMapDpnaddress = 0xf8_04_10_30,
    MainMemoryMapSize = 0xf8_04_10_34,
    MainMemoryMapAddress = 0xf8_04_10_38,
    BadMemoryPagesSize = 0xf8_04_10_3c,
    BadMemoryPagesAddress = 0xf8_04_10_40,
    FEPPhysicalAddressHigh = 0xf8_04_10_44,
    UnwiredVirtualAddressLow = 0xf8_04_10_48,
    UnwiredVirtualAddressHigh = 0xf8_04_10_4c,
    UnwiredPhysicalAddressLow = 0xf8_04_10_50,
    UnwiredPhysicalAddressHigh = 0xf8_04_10_54,
    RequestingLispToStop = 0xf8_04_10_58,
    CurrentFEPOverlays = 0xf8_04_10_5c,
    EmbCommunicationArea = 0xf8_04_10_60,
    LoadedBandName = 0xf8_04_10_64,
    NetbootControlString = 0xf8_04_10_68,
    SoftwareConfiguration = 0xf8_04_10_6c,
    NetAddress1 = 0xf8_04_10_70,
    NetAddress2 = 0xf8_04_10_74,
    PrimaryNetworkAddress = 0xf8_04_10_78,
    FEPCommandString = 0xf8_04_10_7c,
    FEPCrashDataRequest = 0xf8_04_10_80,
    ColdLoadStreamReadCharacter = 0xf8_04_10_84,
    ColdLoadStreamListen = 0xf8_04_10_88,
    ColdLoadStreamReadHardwareCharacter = 0xf8_04_10_8c,
    ColdLoadStreamDrawCharacter = 0xf8_04_10_90,
    ColdLoadStreamDisplayLozengedString = 0xf8_04_10_94,
    ColdLoadStreamSelect = 0xf8_04_10_98,
    ColdLoadStreamBeep = 0xf8_04_10_9c,
    ColdLoadStreamFinish = 0xf8_04_10_a0,
    ColdLoadStreamInsideSize = 0xf8_04_10_a4,
    ColdLoadStreamSetCursorpos = 0xf8_04_10_a8,
    ColdLoadStreamReadCursorpos = 0xf8_04_10_ac,
    ColdLoadStreamComputeMotion = 0xf8_04_10_b0,
    ColdLoadStreamClearBetweenCursorposes = 0xf8_04_10_b4,
    ColdLoadStreamSetEdges = 0xf8_04_10_b8,
    MainScreenParameters = 0xf8_04_10_bc,
    WiredFormat = 0xf8_04_10_c0,
    FEPSequenceBreak = 0xf8_04_10_c4, // Obsolete
    LispStoppedCleanly = 0xf8_04_10_c8,
    LoadPagesToSwapAreaP = 0xf8_04_10_cc,
    RemoteDebugLoop = 0xf8_04_10_d0,
    TimezoneOffsetMinutes = 0xf8_04_10_d4,
    TimezoneName = 0xf8_04_10_d8,
    NamespaceDescriptorFile = 0xf8_04_10_dc,
    SiteName = 0xf8_04_10_e0,
    SavedLispRegisters = 0xf8_04_10_e4,
    LispStateSaved = 0xf8_04_10_e8,
    EnableFpap = 0xf8_04_10_ec,
    DiskUnitTable = 0xf8_04_10_f0,
    HardwareConfiguration = 0xf8_04_10_f4,
    SlaveBufferBaseAddress = 0xf8_04_10_f8,
    KernelCompressedStringArray = 0xf8_04_10_fc,
    Domino8032state = 0xf8_04_11_00,
}

///////////////////////////////////////////////////////////////////////////////////
//
// Genera version of System Communications area
// Define SYS:I-SYS;SYSDF1 line 403+
// Location 0x041100 / Size 0x100 (256) EmbWords
//
pub const ADDRESS_SYSTEM_COMM_AREA: u32 = 0xf8_04_11_00;
#[repr(u32)]
#[derive(Default, Copy, Clone, Debug, PartialEq, Eq, FromPrimitive)]
pub enum SystemCommAreaSlot {
    #[default]
    SyscomMajorVersionNumber = ADDRESS_SYSTEM_COMM_AREA,
    SyscomMinorVersionNumber = 0xf8_04_11_04,
    SystemStartup = 0xf8_04_11_08,

    // Address Spacemap
    AddressSpaceMapAddress = 0xf8_04_110c, // Maps quanta to regions.  See STORAGE for format info.
    OblastFreeSize = 0xf8_04_11_10, // Contiguous free quanta in each oblast.

    // Per-area tables.  These are arrays.  They are here for the console program.
    AreaName = 0xf8_04_11_14, // A symbol
    AreaMaximumQuantumSize = 0xf8_04_11_18,
    AreaRegionQuantumSize = 0xf8_04_11_1c,
    AreaRegionList = 0xf8_04_11_20,
    AreaRegionBits = 0xf8_04_11_24,

    // Per-region tables.  These are arrays.  They are here for the console program.
    RegionQuantumOrigin = 0xf8_04_11_28,
    RegionQuantumLength = 0xf8_04_11_2c,
    RegionFreePointer = 0xf8_04_11_30, // Number of words actually used
    RegionGcpointer = 0xf8_04_11_34, // Number of words scanned by (long-term) GC
    RegionBits = 0xf8_04_11_38, // Fixnum of random fields (see %%REGION- bytes)
    RegionListThread = 0xf8_04_11_3c,
    RegionArea = 0xf8_04_11_40,
    RegionCreatedPages = 0xf8_04_11_44,
    RegionFreePointerBeforeFlip = 0xf8_04_11_48,
    RegionConsAlarm = 0xf8_04_11_4c,
    PageConsAlarm = 0xf8_04_11_50,
    StructureCacheRegion = 0xf8_04_11_54,
    ListCacheRegion = 0xf8_04_11_58,
    DefaultConsArea = 0xf8_04_11_5c,

    // Pointers to critical storage-system tables (these are displaced arrays)
    PHT = 0xf8_04_11_60, // Page hash table
    MainMemoryPageTableY = 0xf8_04_11_64, // Main Memory Y subscript table
    MainMemoryPageTable = 0xf8_04_11_68, // Main Memory page table
    SecondaryMemoryPageTable = 0xf8_04_11_6c, // Secondary Memory page table
    LoadBitmaps = 0xf8_04_11_70,

    //  The following are red herrings for functionality that is really in FEPCOM.
    //  Presumably they leaked in from L-world during the IMach project.
    LoadMap = 0xf8_04_11_74, // Red herring */
    LoadMapDpn = 0xf8_04_11_78, // Red herring */
    SwapMap = 0xf8_04_11_7c, // Red herring */
    SwapMapDpn = 0xf8_04_11_80, // Red herring */
    SysoutBitmaps = 0xf8_04_11_84,

    // Dynamic storage array					4 bits per PHT bucket.
    PHTCollisionCounts = 0xf8_04_11_88,
    Mmpt1 = 0xf8_04_11_8c,
    StorageColdBoot = 0xf8_04_11_90,
    FlushableQueueHead = 0xf8_04_11_94,
    FlushableQueueTail = 0xf8_04_11_98,
    FlushableQueueModified = 0xf8_04_11_9c,
    WiredPhysicalAddressHigh = 0xf8_04_11_a0,
    WiredVirtualAddressHigh = 0xf8_04_11_a4,
    EnableSysoutAtColdBoot = 0xf8_04_11_a8,
    SysoutGenerationNumber = 0xf8_04_11_ac,
    SysoutTimestamp1 = 0xf8_04_11_b0,
    SysoutTimestamp2 = 0xf8_04_11_b4,

    // microsecond clock at some convenient time
    // of disk-save/sysout.
    SysoutParentTimestamp1 = 0xf8_04_11_b8,
    SysoutParentTimestamp2 = 0xf8_04_11_bc,
    InitialStackGroup = 0xf8_04_11_c0,
    CurrentStackGroup = 0xf8_04_11_c4,
    StackGroupLock = 0xf8_04_11_c8,
    CurrentStackGroupStatusBits = 0xf8_04_11_cc,
    InhibitSchedulingFlag = 0xf8_04_11_d0,
    ControlStackLow = 0xf8_04_11_d4,
    BindingStackLow = 0xf8_04_11_d8,

    // Floating-point control registers
    FloatOperatingMode = 0xf8_04_11_dc,
    FloatOperationStatus = 0xf8_04_11_e0,

    PackageNameTable = 0xf8_04_11_e4,
    LispReleaseString = 0xf8_04_11_e8,
    BusMode = 0xf8_04_11_ec,
    Usused1 = 0xf8_04_11_f0,
    Usused2 = 0xf8_04_11_f4,
    Usused3 = 0xf8_04_11_f8,
    Usused4 = 0xf8_04_11_fc,
}

pub const ADDRESS_T: u32 = 0xf8_04_12_08;
pub const ADDRESS_NIL: u32 = 0xf8_04_12_00;

pub const PAGE_SIZE: u32 = 0x100; // 256
pub const PAGE_NUMBER_MASK: u32 = 0xff00;
pub const PAGE_OFFSET_MASK: u32 = 0x00ff;
pub const PAGE_ADDRESS_SHIFT: u32 = 8;

// CHECK: unused?
// pub const LOAD_MAP_DATA_PAGES: u32 = 0;
// pub const LOAD_MAP_CONSTANT: u32 = 1;
// pub const LOAD_MAP_CONSTANT_INCREMENTED: u32 = 2;
// pub const LOAD_MAP_COPY: u32 = 3;

pub const ADDRESS_QUANTUM_SHIFT: u32 = 20;
pub const QUANTUM_SIZE: u32 = 2 ^ ADDRESS_QUANTUM_SHIFT;

#[derive(Default, Copy, Clone, Debug, PartialEq, Eq, FromPrimitive)]
pub enum MemoryCycleTypes {
    #[default] // Default constructor
    DataRead = 0,
    DataWrite = 1,
    BindRead = 2,
    BindWrite = 3,
    BindReadNoMonitor = 4,
    BindWriteNoMonitor = 5,
    Header = 6,
    StructureOffset = 7,
    Scavenge = 8,
    Cdr = 9,
    GCCopy = 10,
    Raw = 11,
    RawTranslate = 12,
}

impl fmt::Display for MemoryCycleTypes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

#[derive(Default, Copy, Clone, Debug, PartialEq, Eq, FromPrimitive)]
pub enum VMOpcode {
    #[default] // Default constructor
    Lookup = 0, // reply is index
    Create = 1,
    Destroy = 2,
    ReadAttributes = 3, // operand is index
    WriteAttributes = 4, // operand is index
    Fill = 5, // operand is increment (of fill data)
    Search = 6, // operand is increment (of address)
    Copy = 7, // operand is memory-cycle?
    Scan = 8,
    Enable = 9,
}

impl fmt::Display for VMOpcode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

#[derive(Default, Copy, Clone, Debug, PartialEq, Eq, FromPrimitive)]
pub enum VMResultCode {
    #[default] // Default constructor
    Success = 0,
    Failure = 1,
}

impl fmt::Display for VMResultCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

#[derive(Default, Copy, Clone, Debug, PartialEq, Eq, FromPrimitive)]
pub enum VMRegisterNumber {
    #[default] // Default constructor
    Command = 0o1100,
    Address = 0o1101,
    Extent = 0o1102,
    Attributes = 0o1103,
    Destination = 0o1104,
    Data = 0o1105,
}

impl fmt::Display for VMRegisterNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

#[derive(Default, Copy, Clone, Debug, PartialEq, Eq, FromPrimitive)]
pub enum IvoryDispatch {
    CarFP = 0o__0, // 0x0 --- 0
    CarLP = 0o__1, // 0x1 --- 1
    CarSP = 0o__2, // 0x2 --- 2
    CarImmediate = 0o__3, // 0x3 --- 3
    CarPop = 0o__4, // 0x4 --- 4
    CdrFP = 0o__5, // 0x5 --- 5
    CdrLP = 0o__6, // 0x6 --- 6
    CdrSP = 0o__7, // 0x7 --- 7
    CdrImmediate = 0o_10, // 0x8 --- 8
    CdrPop = 0o_11, // 0x9 --- 9
    EndpFP = 0o_12, // 0xA --- 10
    EndpLP = 0o_13, // 0xB --- 11
    EndpSP = 0o_14, // 0xC --- 12
    EndpImmediate = 0o_15, // 0xD --- 13
    EndpPop = 0o_16, // 0xE --- 14
    Setup1dArrayFP = 0o_17, // 0xF --- 15
    Setup1dArrayLP = 0o_20, // 0x10 --- 16
    Setup1dArraySP = 0o_21, // 0x11 --- 17
    Setup1dArrayImmediate = 0o_22, // 0x12 --- 18
    Setup1dArrayPop = 0o_23, // 0x13 --- 19
    SetupForce1dArrayFP = 0o_24, // 0x14 --- 20
    SetupForce1dArrayLP = 0o_25, // 0x15 --- 21
    SetupForce1dArraySP = 0o_26, // 0x16 --- 22
    SetupForce1dArrayImmediate = 0o_27, // 0x17 --- 23
    SetupForce1dArrayPop = 0o_30, // 0x18 --- 24
    BindLocativeFP = 0o_31, // 0x19 --- 25
    BindLocativeLP = 0o_32, // 0x1A --- 26
    BindLocativeSP = 0o_33, // 0x1B --- 27
    BindLocativeImmediate = 0o_34, // 0x1C --- 28
    BindLocativePop = 0o_35, // 0x1D --- 29
    RestoreBindingStackFP = 0o_36, // 0x1E --- 30
    RestoreBindingStackLP = 0o_37, // 0x1F --- 31
    RestoreBindingStackSP = 0o_40, // 0x20 --- 32
    RestoreBindingStackImmediate = 0o_41, // 0x21 --- 33
    RestoreBindingStackPop = 0o_42, // 0x22 --- 34
    EphemeralpFP = 0o_43, // 0x23 --- 35
    EphemeralpLP = 0o_44, // 0x24 --- 36
    EphemeralpSP = 0o_45, // 0x25 --- 37
    EphemeralpImmediate = 0o_46, // 0x26 --- 38
    EphemeralpPop = 0o_47, // 0x27 --- 39
    StartCallFP = 0o_50, // 0x28 --- 40
    StartCallLP = 0o_51, // 0x29 --- 41
    StartCallSP = 0o_52, // 0x2A --- 42
    StartCallImmediate = 0o_53, // 0x2B --- 43
    StartCallPop = 0o_54, // 0x2C --- 44
    JumpFP = 0o_55, // 0x2D --- 45
    JumpLP = 0o_56, // 0x2E --- 46
    JumpSP = 0o_57, // 0x2F --- 47
    JumpImmediate = 0o_60, // 0x30 --- 48
    JumpPop = 0o_61, // 0x31 --- 49
    TagFP = 0o_62, // 0x32 --- 50
    TagLP = 0o_63, // 0x33 --- 51
    TagSP = 0o_64, // 0x34 --- 52
    TagImmediate = 0o_65, // 0x35 --- 53
    TagPop = 0o_66, // 0x36 --- 54
    DereferenceFP = 0o_67, // 0x37 --- 55
    DereferenceLP = 0o_70, // 0x38 --- 56
    DereferenceSP = 0o_71, // 0x39 --- 57
    DereferenceImmediate = 0o_72, // 0x3A --- 58
    DereferencePop = 0o_73, // 0x3B --- 59
    LogicTailTestFP = 0o_74, // 0x3C --- 60
    LogicTailTestLP = 0o_75, // 0x3D --- 61
    LogicTailTestSP = 0o_76, // 0x3E --- 62
    LogicTailTestImmediate = 0o_77, // 0x3F --- 63
    LogicTailTestPop = 0o100, // 0x40 --- 64
    ProcBreakpointFP = 0o101, // 0x41 --- 65
    ProcBreakpointLP = 0o102, // 0x42 --- 66
    ProcBreakpointSP = 0o103, // 0x43 --- 67
    ProcBreakpointImmediate = 0o104, // 0x44 --- 68
    ProcBreakpointPop = 0o105, // 0x45 --- 69
    PushLexicalVarFP = 0o106, // 0x46 --- 70
    PushLexicalVarLP = 0o107, // 0x47 --- 71
    PushLexicalVarSP = 0o110, // 0x48 --- 72
    PushLexicalVarImmediate = 0o111, // 0x49 --- 73
    PushLexicalVarPop = 0o112, // 0x4A --- 74
    Block0WriteFP = 0o113, // 0x4B --- 75
    Block0WriteLP = 0o114, // 0x4C --- 76
    Block0WriteSP = 0o115, // 0x4D --- 77
    Block0WriteImmediate = 0o116, // 0x4E --- 78
    Block0WritePop = 0o117, // 0x4F --- 79
    Block1WriteFP = 0o120, // 0x50 --- 80
    Block1WriteLP = 0o121, // 0x51 --- 81
    Block1WriteSP = 0o122, // 0x52 --- 82
    Block1WriteImmediate = 0o123, // 0x53 --- 83
    Block1WritePop = 0o124, // 0x54 --- 84
    Block2WriteFP = 0o125, // 0x55 --- 85
    Block2WriteLP = 0o126, // 0x56 --- 86
    Block2WriteSP = 0o127, // 0x57 --- 87
    Block2WriteImmediate = 0o130, // 0x58 --- 88
    Block2WritePop = 0o131, // 0x59 --- 89
    Block3WriteFP = 0o132, // 0x5A --- 90
    Block3WriteLP = 0o133, // 0x5B --- 91
    Block3WriteSP = 0o134, // 0x5C --- 92
    Block3WriteImmediate = 0o135, // 0x5D --- 93
    Block3WritePop = 0o136, // 0x5E --- 94
    ZeropFP = 0o137, // 0x5F --- 95
    ZeropLP = 0o140, // 0x60 --- 96
    ZeropSP = 0o141, // 0x61 --- 97
    ZeropImmediate = 0o142, // 0x62 --- 98
    ZeropPop = 0o143, // 0x63 --- 99
    MinuspFP = 0o144, // 0x64 --- 100
    MinuspLP = 0o145, // 0x65 --- 101
    MinuspSP = 0o146, // 0x66 --- 102
    MinuspImmediate = 0o147, // 0x67 --- 103
    MinuspPop = 0o150, // 0x68 --- 104
    PluspFP = 0o151, // 0x69 --- 105
    PluspLP = 0o152, // 0x6A --- 106
    PluspSP = 0o153, // 0x6B --- 107
    PluspImmediate = 0o154, // 0x6C --- 108
    PluspPop = 0o155, // 0x6D --- 109
    TypeMember = 0o156, // 0x6E --- 110
    TypeMemberNoPop = 0o157, // 0x6F --- 111
    LocateLocals = 0o160, // 0x70 --- 112
    CatchClose = 0o161, // 0x71 --- 113
    GenericDispatch = 0o162, // 0x72 --- 114
    MessageDispatch = 0o163, // 0x73 --- 115
    CheckPreemptRequest = 0o164, // 0x74 --- 116
    PushGlobalLogicVariable = 0o165, // 0x75 --- 117
    NoOp = 0o166, // 0x76 --- 118
    Halt = 0o167, // 0x77 --- 119
    BranchTrue = 0o170, // 0x78 --- 120
    BranchTrueElseExtraPop = 0o171, // 0x79 --- 121
    BranchTrueAndExtraPop = 0o172, // 0x7A --- 122
    BranchTrueExtraPop = 0o173, // 0x7B --- 123
    BranchTrueNoPop = 0o174, // 0x7C --- 124
    BranchTrueAndNoPop = 0o175, // 0x7D --- 125
    BranchTrueElseNoPop = 0o176, // 0x7E --- 126
    BranchTrueAndNoPopElseNoPopExtraPop = 0o177, // 0x7F --- 127
    BranchFalse = 0o200, // 0x80 --- 128
    BranchFalseElseExtraPop = 0o201, // 0x81 --- 129
    BranchFalseAndExtraPop = 0o202, // 0x82 --- 130
    BranchFalseExtraPop = 0o203, // 0x83 --- 131
    BranchFalseNoPop = 0o204, // 0x84 --- 132
    BranchFalseAndNoPop = 0o205, // 0x85 --- 133
    BranchFalseElseNoPop = 0o206, // 0x86 --- 134
    BranchFalseAndNoPopElseNoPopExtraPop = 0o207, // 0x87 --- 135
    PushFP = 0o210, // 0x88 --- 136
    PushLP = 0o211, // 0x89 --- 137
    PushSP = 0o212, // 0x8A --- 138
    PushImmediate = 0o213, // 0x8B --- 139
    PushPop = 0o214, // 0x8C --- 140
    PushNNils = 0o215, // 0x8D --- 141
    PushAddressSpRelativeFP = 0o216, // 0x8E --- 142
    PushAddressSpRelativeLP = 0o217, // 0x8F --- 143
    PushAddressSpRelativeSP = 0o220, // 0x90 --- 144
    PushAddressSpRelativeImmediate = 0o221, // 0x91 --- 145
    PushAddressSpRelativePop = 0o222, // 0x92 --- 146
    PushLocalLogicVariablesFP = 0o223, // 0x93 --- 147
    PushLocalLogicVariablesLP = 0o224, // 0x94 --- 148
    PushLocalLogicVariablesSP = 0o225, // 0x95 --- 149
    PushLocalLogicVariablesImmediate = 0o226, // 0x96 --- 150
    PushLocalLogicVariablesPop = 0o227, // 0x97 --- 151
    ReturnMultipleFP = 0o230, // 0x98 --- 152
    ReturnMultipleLP = 0o231, // 0x99 --- 153
    ReturnMultipleSP = 0o232, // 0x9A --- 154
    ReturnMultipleImmediate = 0o233, // 0x9B --- 155
    ReturnMultiplePop = 0o234, // 0x9C --- 156
    ReturnKludgeFP = 0o235, // 0x9D --- 157
    ReturnKludgeLP = 0o236, // 0x9E --- 158
    ReturnKludgeSP = 0o237, // 0x9F --- 159
    ReturnKludgeImmediate = 0o240, // 0xA0 --- 160
    ReturnKludgePop = 0o241, // 0xA1 --- 161
    TakeValues = 0o242, // 0xA2 --- 162
    UnbindNImmediate = 0o243, // 0xA3 --- 163
    UnbindNPop = 0o244, // 0xA4 --- 164
    PushInstanceVariable = 0o245, // 0xA5 --- 165
    PushAddressInstanceVariable = 0o246, // 0xA6 --- 166
    PushInstanceVariableOrdered = 0o247, // 0xA7 --- 167
    PushAddressInstanceVariableOrdered = 0o250, // 0xA8 --- 168
    UnaryMinusFP = 0o251, // 0xA9 --- 169
    UnaryMinusLP = 0o252, // 0xAA --- 170
    UnaryMinusSP = 0o253, // 0xAB --- 171
    UnaryMinusImmediate = 0o254, // 0xAC --- 172
    UnaryMinusPop = 0o255, // 0xAD --- 173
    ReturnSingleNIL = 0o256, // 0xAE --- 174
    ReturnSingleT = 0o257, // 0xAF --- 175
    ReturnSingleTOS = 0o260, // 0xB0 --- 176
    MemoryRead = 0o261, // 0xB1 --- 177
    MemoryReadAddress = 0o262, // 0xB2 --- 178
    Block0Read = 0o263, // 0xB3 --- 179
    Block1Read = 0o264, // 0xB4 --- 180
    Block2Read = 0o265, // 0xB5 --- 181
    Block3Read = 0o266, // 0xB6 --- 182
    Block0ReadShift = 0o267, // 0xB7 --- 183
    Block1ReadShift = 0o270, // 0xB8 --- 184
    Block2ReadShift = 0o271, // 0xB9 --- 185
    Block3ReadShift = 0o272, // 0xBA --- 186
    Block0ReadTest = 0o273, // 0xBB --- 187
    Block1ReadTest = 0o274, // 0xBC --- 188
    Block2ReadTest = 0o275, // 0xBD --- 189
    Block3ReadTest = 0o276, // 0xBE --- 190
    FinishCallN = 0o277, // 0xBF --- 191
    FinishCallNApply = 0o300, // 0xC0 --- 192
    FinishCallTos = 0o301, // 0xC1 --- 193
    FinishCallTosApply = 0o302, // 0xC2 --- 194
    SetToCarFP = 0o303, // 0xC3 --- 195
    SetToCarLP = 0o304, // 0xC4 --- 196
    SetToCarSP = 0o305, // 0xC5 --- 197
    SetToCarImmediate = 0o306, // 0xC6 --- 198
    SetToCarPop = 0o307, // 0xC7 --- 199
    SetToCdrFP = 0o310, // 0xC8 --- 200
    SetToCdrLP = 0o311, // 0xC9 --- 201
    SetToCdrSP = 0o312, // 0xCA --- 202
    SetToCdrImmediate = 0o313, // 0xCB --- 203
    SetToCdrPop = 0o314, // 0xCC --- 204
    SetToCdrPushCarFP = 0o315, // 0xCD --- 205
    SetToCdrPushCarLP = 0o316, // 0xCE --- 206
    SetToCdrPushCarSP = 0o317, // 0xCF --- 207
    SetToCdrPushCarImmediate = 0o320, // 0xD0 --- 208
    SetToCdrPushCarPop = 0o321, // 0xD1 --- 209
    IncrementFP = 0o322, // 0xD2 --- 210
    IncrementLP = 0o323, // 0xD3 --- 211
    IncrementSP = 0o324, // 0xD4 --- 212
    IncrementImmediate = 0o325, // 0xD5 --- 213
    IncrementPop = 0o326, // 0xD6 --- 214
    DecrementFP = 0o327, // 0xD7 --- 215
    DecrementLP = 0o330, // 0xD8 --- 216
    DecrementSP = 0o331, // 0xD9 --- 217
    DecrementImmediate = 0o332, // 0xDA --- 218
    DecrementPop = 0o333, // 0xDB --- 219
    PointerIncrementFP = 0o334, // 0xDC --- 220
    PointerIncrementLP = 0o335, // 0xDD --- 221
    PointerIncrementSP = 0o336, // 0xDE --- 222
    PointerIncrementImmediate = 0o337, // 0xDF --- 223
    PointerIncrementPop = 0o340, // 0xE0 --- 224
    SetCdrCode1FP = 0o341, // 0xE1 --- 225
    SetCdrCode1LP = 0o342, // 0xE2 --- 226
    SetCdrCode1SP = 0o343, // 0xE3 --- 227
    SetCdrCode1Immediate = 0o344, // 0xE4 --- 228
    SetCdrCode1Pop = 0o345, // 0xE5 --- 229
    SetCdrCode2FP = 0o346, // 0xE6 --- 230
    SetCdrCode2LP = 0o347, // 0xE7 --- 231
    SetCdrCode2SP = 0o350, // 0xE8 --- 232
    SetCdrCode2Immediate = 0o351, // 0xE9 --- 233
    SetCdrCode2Pop = 0o352, // 0xEA --- 234
    PushAddressFP = 0o353, // 0xEB --- 235
    PushAddressLP = 0o354, // 0xEC --- 236
    PushAddressSP = 0o355, // 0xED --- 237
    PushAddressImmediate = 0o356, // 0xEE --- 238
    PushAddressPop = 0o357, // 0xEF --- 239
    SetSpToAddressFP = 0o360, // 0xF0 --- 240
    SetSpToAddressLP = 0o361, // 0xF1 --- 241
    SetSpToAddressSP = 0o362, // 0xF2 --- 242
    SetSpToAddressImmediate = 0o363, // 0xF3 --- 243
    SetSpToAddressPop = 0o364, // 0xF4 --- 244
    SetSpToAddressSaveTosFP = 0o365, // 0xF5 --- 245
    SetSpToAddressSaveTosLP = 0o366, // 0xF6 --- 246
    SetSpToAddressSaveTosSP = 0o367, // 0xF7 --- 247
    SetSpToAddressSaveTosImmediate = 0o370, // 0xF8 --- 248
    SetSpToAddressSaveTosPop = 0o371, // 0xF9 --- 249
    ReadInternalRegister = 0o372, // 0xFA --- 250
    WriteInternalRegister = 0o373, // 0xFB --- 251
    CoprocessorRead = 0o374, // 0xFC --- 252
    CoprocessorWrite = 0o375, // 0xFD --- 253
    Block0ReadAluFP = 0o376, // 0xFE --- 254
    Block0ReadAluLP = 0o377, // 0xFF --- 255
    Block0ReadAluSP = 0o400, // 0x100 --- 256
    Block0ReadAluImmediate = 0o401, // 0x101 --- 257
    Block0ReadAluPop = 0o402, // 0x102 --- 258
    Block1ReadAluFP = 0o403, // 0x103 --- 259
    Block1ReadAluLP = 0o404, // 0x104 --- 260
    Block1ReadAluSP = 0o405, // 0x105 --- 261
    Block1ReadAluImmediate = 0o406, // 0x106 --- 262
    Block1ReadAluPop = 0o407, // 0x107 --- 263
    Block2ReadAluFP = 0o410, // 0x108 --- 264
    Block2ReadAluLP = 0o411, // 0x109 --- 265
    Block2ReadAluSP = 0o412, // 0x10A --- 266
    Block2ReadAluImmediate = 0o413, // 0x10B --- 267
    Block2ReadAluPop = 0o414, // 0x10C --- 268
    Block3ReadAluFP = 0o415, // 0x10D --- 269
    Block3ReadAluLP = 0o416, // 0x10E --- 270
    Block3ReadAluSP = 0o417, // 0x10F --- 271
    Block3ReadAluImmediate = 0o420, // 0x110 --- 272
    Block3ReadAluPop = 0o421, // 0x111 --- 273
    Ldb = 0o422, // 0x112 --- 274
    CharLdb = 0o423, // 0x113 --- 275
    PLdb = 0o424, // 0x114 --- 276
    PTagLdb = 0o425, // 0x115 --- 277
    Branch = 0o426, // 0x116 --- 278
    LoopDecrementTos = 0o427, // 0x117 --- 279
    EntryRestAccepted = 0o430, // 0x118 --- 280
    EntryRestNotAccepted = 0o431, // 0x119 --- 281
    RplacaFP = 0o432, // 0x11A --- 282
    RplacaLP = 0o433, // 0x11B --- 283
    RplacaSP = 0o434, // 0x11C --- 284
    RplacaImmediate = 0o435, // 0x11D --- 285
    RplacaPop = 0o436, // 0x11E --- 286
    RplacdFP = 0o437, // 0x11F --- 287
    RplacdLP = 0o440, // 0x120 --- 288
    RplacdSP = 0o441, // 0x121 --- 289
    RplacdImmediate = 0o442, // 0x122 --- 290
    RplacdPop = 0o443, // 0x123 --- 291
    MultiplyFP = 0o444, // 0x124 --- 292
    MultiplyLP = 0o445, // 0x125 --- 293
    MultiplySP = 0o446, // 0x126 --- 294
    MultiplyImmediate = 0o447, // 0x127 --- 295
    MultiplyPop = 0o450, // 0x128 --- 296
    QuotientFP = 0o451, // 0x129 --- 297
    QuotientLP = 0o452, // 0x12A --- 298
    QuotientSP = 0o453, // 0x12B --- 299
    QuotientImmediate = 0o454, // 0x12C --- 300
    QuotientPop = 0o455, // 0x12D --- 301
    CeilingFP = 0o456, // 0x12E --- 302
    CeilingLP = 0o457, // 0x12F --- 303
    CeilingSP = 0o460, // 0x130 --- 304
    CeilingImmediate = 0o461, // 0x131 --- 305
    CeilingPop = 0o462, // 0x132 --- 306
    FloorFP = 0o463, // 0x133 --- 307
    FloorLP = 0o464, // 0x134 --- 308
    FloorSP = 0o465, // 0x135 --- 309
    FloorImmediate = 0o466, // 0x136 --- 310
    FloorPop = 0o467, // 0x137 --- 311
    TruncateFP = 0o470, // 0x138 --- 312
    TruncateLP = 0o471, // 0x139 --- 313
    TruncateSP = 0o472, // 0x13A --- 314
    TruncateImmediate = 0o473, // 0x13B --- 315
    TruncatePop = 0o474, // 0x13C --- 316
    RoundFP = 0o475, // 0x13D --- 317
    RoundLP = 0o476, // 0x13E --- 318
    RoundSP = 0o477, // 0x13F --- 319
    RoundImmediate = 0o500, // 0x140 --- 320
    RoundPop = 0o501, // 0x141 --- 321
    RationalQuotientFP = 0o502, // 0x142 --- 322
    RationalQuotientLP = 0o503, // 0x143 --- 323
    RationalQuotientSP = 0o504, // 0x144 --- 324
    RationalQuotientImmediate = 0o505, // 0x145 --- 325
    RationalQuotientPop = 0o506, // 0x146 --- 326
    MinFP = 0o507, // 0x147 --- 327
    MinLP = 0o510, // 0x148 --- 328
    MinSP = 0o511, // 0x149 --- 329
    MinImmediate = 0o512, // 0x14A --- 330
    MinPop = 0o513, // 0x14B --- 331
    MaxFP = 0o514, // 0x14C --- 332
    MaxLP = 0o515, // 0x14D --- 333
    MaxSP = 0o516, // 0x14E --- 334
    MaxImmediate = 0o517, // 0x14F --- 335
    MaxPop = 0o520, // 0x150 --- 336
    AluFP = 0o521, // 0x151 --- 337
    AluLP = 0o522, // 0x152 --- 338
    AluSP = 0o523, // 0x153 --- 339
    AluImmediate = 0o524, // 0x154 --- 340
    AluPop = 0o525, // 0x155 --- 341
    LogandFP = 0o526, // 0x156 --- 342
    LogandLP = 0o527, // 0x157 --- 343
    LogandSP = 0o530, // 0x158 --- 344
    LogandImmediate = 0o531, // 0x159 --- 345
    LogandPop = 0o532, // 0x15A --- 346
    LogxorFP = 0o533, // 0x15B --- 347
    LogxorLP = 0o534, // 0x15C --- 348
    LogxorSP = 0o535, // 0x15D --- 349
    LogxorImmediate = 0o536, // 0x15E --- 350
    LogxorPop = 0o537, // 0x15F --- 351
    LogiorFP = 0o540, // 0x160 --- 352
    LogiorLP = 0o541, // 0x161 --- 353
    LogiorSP = 0o542, // 0x162 --- 354
    LogiorImmediate = 0o543, // 0x163 --- 355
    LogiorPop = 0o544, // 0x164 --- 356
    RotFP = 0o545, // 0x165 --- 357
    RotLP = 0o546, // 0x166 --- 358
    RotSP = 0o547, // 0x167 --- 359
    RotImmediate = 0o550, // 0x168 --- 360
    RotPop = 0o551, // 0x169 --- 361
    LshFP = 0o552, // 0x16A --- 362
    LshLP = 0o553, // 0x16B --- 363
    LshSP = 0o554, // 0x16C --- 364
    LshImmediate = 0o555, // 0x16D --- 365
    LshPop = 0o556, // 0x16E --- 366
    MultiplyDoubleFP = 0o557, // 0x16F --- 367
    MultiplyDoubleLP = 0o560, // 0x170 --- 368
    MultiplyDoubleSP = 0o561, // 0x171 --- 369
    MultiplyDoubleImmediate = 0o562, // 0x172 --- 370
    MultiplyDoublePop = 0o563, // 0x173 --- 371
    LshcBignumStepFP = 0o564, // 0x174 --- 372
    LshcBignumStepLP = 0o565, // 0x175 --- 373
    LshcBignumStepSP = 0o566, // 0x176 --- 374
    LshcBignumStepImmediate = 0o567, // 0x177 --- 375
    LshcBignumStepPop = 0o570, // 0x178 --- 376
    StackBltFP = 0o571, // 0x179 --- 377
    StackBltLP = 0o572, // 0x17A --- 378
    StackBltSP = 0o573, // 0x17B --- 379
    StackBltImmediate = 0o574, // 0x17C --- 380
    StackBltPop = 0o575, // 0x17D --- 381
    RgetfFP = 0o576, // 0x17E --- 382
    RgetfLP = 0o577, // 0x17F --- 383
    RgetfSP = 0o600, // 0x180 --- 384
    RgetfImmediate = 0o601, // 0x181 --- 385
    RgetfPop = 0o602, // 0x182 --- 386
    MemberFP = 0o603, // 0x183 --- 387
    MemberLP = 0o604, // 0x184 --- 388
    MemberSP = 0o605, // 0x185 --- 389
    MemberImmediate = 0o606, // 0x186 --- 390
    MemberPop = 0o607, // 0x187 --- 391
    AssocFP = 0o610, // 0x188 --- 392
    AssocLP = 0o611, // 0x189 --- 393
    AssocSP = 0o612, // 0x18A --- 394
    AssocImmediate = 0o613, // 0x18B --- 395
    AssocPop = 0o614, // 0x18C --- 396
    PointerPlusFP = 0o615, // 0x18D --- 397
    PointerPlusLP = 0o616, // 0x18E --- 398
    PointerPlusSP = 0o617, // 0x18F --- 399
    PointerPlusImmediate = 0o620, // 0x190 --- 400
    PointerPlusPop = 0o621, // 0x191 --- 401
    PointerDifferenceFP = 0o622, // 0x192 --- 402
    PointerDifferenceLP = 0o623, // 0x193 --- 403
    PointerDifferenceSP = 0o624, // 0x194 --- 404
    PointerDifferenceImmediate = 0o625, // 0x195 --- 405
    PointerDifferencePop = 0o626, // 0x196 --- 406
    AshFP = 0o627, // 0x197 --- 407
    AshLP = 0o630, // 0x198 --- 408
    AshSP = 0o631, // 0x199 --- 409
    AshImmediate = 0o632, // 0x19A --- 410
    AshPop = 0o633, // 0x19B --- 411
    StoreConditionalFP = 0o634, // 0x19C --- 412
    StoreConditionalLP = 0o635, // 0x19D --- 413
    StoreConditionalSP = 0o636, // 0x19E --- 414
    StoreConditionalImmediate = 0o637, // 0x19F --- 415
    StoreConditionalPop = 0o640, // 0x1A0 --- 416
    MemoryWriteFP = 0o641, // 0x1A1 --- 417
    MemoryWriteLP = 0o642, // 0x1A2 --- 418
    MemoryWriteSP = 0o643, // 0x1A3 --- 419
    MemoryWriteImmediate = 0o644, // 0x1A4 --- 420
    MemoryWritePop = 0o645, // 0x1A5 --- 421
    PStoreContentsFP = 0o646, // 0x1A6 --- 422
    PStoreContentsLP = 0o647, // 0x1A7 --- 423
    PStoreContentsSP = 0o650, // 0x1A8 --- 424
    PStoreContentsImmediate = 0o651, // 0x1A9 --- 425
    PStoreContentsPop = 0o652, // 0x1AA --- 426
    BindLocativeToValueFP = 0o653, // 0x1AB --- 427
    BindLocativeToValueLP = 0o654, // 0x1AC --- 428
    BindLocativeToValueSP = 0o655, // 0x1AD --- 429
    BindLocativeToValueImmediate = 0o656, // 0x1AE --- 430
    BindLocativeToValuePop = 0o657, // 0x1AF --- 431
    UnifyFP = 0o660, // 0x1B0 --- 432
    UnifyLP = 0o661, // 0x1B1 --- 433
    UnifySP = 0o662, // 0x1B2 --- 434
    UnifyImmediate = 0o663, // 0x1B3 --- 435
    UnifyPop = 0o664, // 0x1B4 --- 436
    PopLexicalVarFP = 0o665, // 0x1B5 --- 437
    PopLexicalVarLP = 0o666, // 0x1B6 --- 438
    PopLexicalVarSP = 0o667, // 0x1B7 --- 439
    PopLexicalVarImmediate = 0o670, // 0x1B8 --- 440
    PopLexicalVarPop = 0o671, // 0x1B9 --- 441
    MovemLexicalVarFP = 0o672, // 0x1BA --- 442
    MovemLexicalVarLP = 0o673, // 0x1BB --- 443
    MovemLexicalVarSP = 0o674, // 0x1BC --- 444
    MovemLexicalVarImmediate = 0o675, // 0x1BD --- 445
    MovemLexicalVarPop = 0o676, // 0x1BE --- 446
    EqualNumberFP = 0o677, // 0x1BF --- 447
    EqualNumberLP = 0o700, // 0x1C0 --- 448
    EqualNumberSP = 0o701, // 0x1C1 --- 449
    EqualNumberImmediate = 0o702, // 0x1C2 --- 450
    EqualNumberPop = 0o703, // 0x1C3 --- 451
    LesspFP = 0o704, // 0x1C4 --- 452
    LesspLP = 0o705, // 0x1C5 --- 453
    LesspSP = 0o706, // 0x1C6 --- 454
    LesspImmediate = 0o707, // 0x1C7 --- 455
    LesspPop = 0o710, // 0x1C8 --- 456
    GreaterpFP = 0o711, // 0x1C9 --- 457
    GreaterpLP = 0o712, // 0x1CA --- 458
    GreaterpSP = 0o713, // 0x1CB --- 459
    GreaterpImmediate = 0o714, // 0x1CC --- 460
    GreaterpPop = 0o715, // 0x1CD --- 461
    EqlFP = 0o716, // 0x1CE --- 462
    EqlLP = 0o717, // 0x1CF --- 463
    EqlSP = 0o720, // 0x1D0 --- 464
    EqlImmediate = 0o721, // 0x1D1 --- 465
    EqlPop = 0o722, // 0x1D2 --- 466
    EqualNumberNoPopFP = 0o723, // 0x1D3 --- 467
    EqualNumberNoPopLP = 0o724, // 0x1D4 --- 468
    EqualNumberNoPopSP = 0o725, // 0x1D5 --- 469
    EqualNumberNoPopImmediate = 0o726, // 0x1D6 --- 470
    EqualNumberNoPopPop = 0o727, // 0x1D7 --- 471
    LesspNoPopFP = 0o730, // 0x1D8 --- 472
    LesspNoPopLP = 0o731, // 0x1D9 --- 473
    LesspNoPopSP = 0o732, // 0x1DA --- 474
    LesspNoPopImmediate = 0o733, // 0x1DB --- 475
    LesspNoPopPop = 0o734, // 0x1DC --- 476
    GreaterpNoPopFP = 0o735, // 0x1DD --- 477
    GreaterpNoPopLP = 0o736, // 0x1DE --- 478
    GreaterpNoPopSP = 0o737, // 0x1DF --- 479
    GreaterpNoPopImmediate = 0o740, // 0x1E0 --- 480
    GreaterpNoPopPop = 0o741, // 0x1E1 --- 481
    EqlNoPopFP = 0o742, // 0x1E2 --- 482
    EqlNoPopLP = 0o743, // 0x1E3 --- 483
    EqlNoPopSP = 0o744, // 0x1E4 --- 484
    EqlNoPopImmediate = 0o745, // 0x1E5 --- 485
    EqlNoPopPop = 0o746, // 0x1E6 --- 486
    EqFP = 0o747, // 0x1E7 --- 487
    EqLP = 0o750, // 0x1E8 --- 488
    EqSP = 0o751, // 0x1E9 --- 489
    EqImmediate = 0o752, // 0x1EA --- 490
    EqPop = 0o753, // 0x1EB --- 491
    LogtestFP = 0o754, // 0x1EC --- 492
    LogtestLP = 0o755, // 0x1ED --- 493
    LogtestSP = 0o756, // 0x1EE --- 494
    LogtestImmediate = 0o757, // 0x1EF --- 495
    LogtestPop = 0o760, // 0x1F0 --- 496
    EqNoPopFP = 0o761, // 0x1F1 --- 497
    EqNoPopLP = 0o762, // 0x1F2 --- 498
    EqNoPopSP = 0o763, // 0x1F3 --- 499
    EqNoPopImmediate = 0o764, // 0x1F4 --- 500
    EqNoPopPop = 0o765, // 0x1F5 --- 501
    LogtestNoPopFP = 0o766, // 0x1F6 --- 502
    LogtestNoPopLP = 0o767, // 0x1F7 --- 503
    LogtestNoPopSP = 0o770, // 0x1F8 --- 504
    LogtestNoPopImmediate = 0o771, // 0x1F9 --- 505
    LogtestNoPopPop = 0o772, // 0x1FA --- 506
    AddFP = 0o773, // 0x1FB --- 507
    AddLP = 0o774, // 0x1FC --- 508
    AddSP = 0o775, // 0x1FD --- 509
    AddImmediate = 0o776, // 0x1FE --- 510
    AddPop = 0o777, // 0x1FF --- 511
    SubFP = 0o1000, // 0x200 --- 512
    SubLP = 0o1001, // 0x201 --- 513
    SubSP = 0o1002, // 0x202 --- 514
    SubImmediate = 0o1003, // 0x203 --- 515
    SubPop = 0o1004, // 0x204 --- 516
    Dispatch32BitPlusFP = 0o1005, // 0x205 --- 517
    Dispatch32BitPlusLP = 0o1006, // 0x206 --- 518
    Dispatch32BitPlusSP = 0o1007, // 0x207 --- 519
    Dispatch32BitPlusImmediate = 0o1010, // 0x208 --- 520
    Dispatch32BitPlusPop = 0o1011, // 0x209 --- 521
    Dispatch32BitDifferenceFP = 0o1012, // 0x20A --- 522
    Dispatch32BitDifferenceLP = 0o1013, // 0x20B --- 523
    Dispatch32BitDifferenceSP = 0o1014, // 0x20C --- 524
    Dispatch32BitDifferenceImmediate = 0o1015, // 0x20D --- 525
    Dispatch32BitDifferencePop = 0o1016, // 0x20E --- 526
    AddBignumStepFP = 0o1017, // 0x20F --- 527
    AddBignumStepLP = 0o1020, // 0x210 --- 528
    AddBignumStepSP = 0o1021, // 0x211 --- 529
    AddBignumStepImmediate = 0o1022, // 0x212 --- 530
    AddBignumStepPop = 0o1023, // 0x213 --- 531
    SubBignumStepFP = 0o1024, // 0x214 --- 532
    SubBignumStepLP = 0o1025, // 0x215 --- 533
    SubBignumStepSP = 0o1026, // 0x216 --- 534
    SubBignumStepImmediate = 0o1027, // 0x217 --- 535
    SubBignumStepPop = 0o1030, // 0x218 --- 536
    MultiplyBignumStepFP = 0o1031, // 0x219 --- 537
    MultiplyBignumStepLP = 0o1032, // 0x21A --- 538
    MultiplyBignumStepSP = 0o1033, // 0x21B --- 539
    MultiplyBignumStepImmediate = 0o1034, // 0x21C --- 540
    MultiplyBignumStepPop = 0o1035, // 0x21D --- 541
    DivideBignumStepFP = 0o1036, // 0x21E --- 542
    DivideBignumStepLP = 0o1037, // 0x21F --- 543
    DivideBignumStepSP = 0o1040, // 0x220 --- 544
    DivideBignumStepImmediate = 0o1041, // 0x221 --- 545
    DivideBignumStepPop = 0o1042, // 0x222 --- 546
    Aset1FP = 0o1043, // 0x223 --- 547
    Aset1LP = 0o1044, // 0x224 --- 548
    Aset1SP = 0o1045, // 0x225 --- 549
    Aset1Immediate = 0o1046, // 0x226 --- 550
    Aset1Pop = 0o1047, // 0x227 --- 551
    AllocateListBlockFP = 0o1050, // 0x228 --- 552
    AllocateListBlockLP = 0o1051, // 0x229 --- 553
    AllocateListBlockSP = 0o1052, // 0x22A --- 554
    AllocateListBlockImmediate = 0o1053, // 0x22B --- 555
    AllocateListBlockPop = 0o1054, // 0x22C --- 556
    Aref1FP = 0o1055, // 0x22D --- 557
    Aref1LP = 0o1056, // 0x22E --- 558
    Aref1SP = 0o1057, // 0x22F --- 559
    Aref1Immediate = 0o1060, // 0x230 --- 560
    Aref1Pop = 0o1061, // 0x231 --- 561
    Aloc1FP = 0o1062, // 0x232 --- 562
    Aloc1LP = 0o1063, // 0x233 --- 563
    Aloc1SP = 0o1064, // 0x234 --- 564
    Aloc1Immediate = 0o1065, // 0x235 --- 565
    Aloc1Pop = 0o1066, // 0x236 --- 566
    StoreArrayLeaderFP = 0o1067, // 0x237 --- 567
    StoreArrayLeaderLP = 0o1070, // 0x238 --- 568
    StoreArrayLeaderSP = 0o1071, // 0x239 --- 569
    StoreArrayLeaderImmediate = 0o1072, // 0x23A --- 570
    StoreArrayLeaderPop = 0o1073, // 0x23B --- 571
    AllocateStructureBlockFP = 0o1074, // 0x23C --- 572
    AllocateStructureBlockLP = 0o1075, // 0x23D --- 573
    AllocateStructureBlockSP = 0o1076, // 0x23E --- 574
    AllocateStructureBlockImmediate = 0o1077, // 0x23F --- 575
    AllocateStructureBlockPop = 0o1100, // 0x240 --- 576
    ArrayLeaderFP = 0o1101, // 0x241 --- 577
    ArrayLeaderLP = 0o1102, // 0x242 --- 578
    ArrayLeaderSP = 0o1103, // 0x243 --- 579
    ArrayLeaderImmediate = 0o1104, // 0x244 --- 580
    ArrayLeaderPop = 0o1105, // 0x245 --- 581
    AlocLeaderFP = 0o1106, // 0x246 --- 582
    AlocLeaderLP = 0o1107, // 0x247 --- 583
    AlocLeaderSP = 0o1110, // 0x248 --- 584
    AlocLeaderImmediate = 0o1111, // 0x249 --- 585
    AlocLeaderPop = 0o1112, // 0x24A --- 586
    PopInstanceVariable = 0o1113, // 0x24B --- 587
    MovemInstanceVariable = 0o1114, // 0x24C --- 588
    PopInstanceVariableOrdered = 0o1115, // 0x24D --- 589
    MovemInstanceVariableOrdered = 0o1116, // 0x24E --- 590
    InstanceRefFP = 0o1117, // 0x24F --- 591
    InstanceRefLP = 0o1120, // 0x250 --- 592
    InstanceRefSP = 0o1121, // 0x251 --- 593
    InstanceRefImmediate = 0o1122, // 0x252 --- 594
    InstanceRefPop = 0o1123, // 0x253 --- 595
    InstanceSetFP = 0o1124, // 0x254 --- 596
    InstanceSetLP = 0o1125, // 0x255 --- 597
    InstanceSetSP = 0o1126, // 0x256 --- 598
    InstanceSetImmediate = 0o1127, // 0x257 --- 599
    InstanceSetPop = 0o1130, // 0x258 --- 600
    InstanceLocFP = 0o1131, // 0x259 --- 601
    InstanceLocLP = 0o1132, // 0x25A --- 602
    InstanceLocSP = 0o1133, // 0x25B --- 603
    InstanceLocImmediate = 0o1134, // 0x25C --- 604
    InstanceLocPop = 0o1135, // 0x25D --- 605
    SetTagFP = 0o1136, // 0x25E --- 606
    SetTagLP = 0o1137, // 0x25F --- 607
    SetTagSP = 0o1140, // 0x260 --- 608
    SetTagImmediate = 0o1141, // 0x261 --- 609
    SetTagPop = 0o1142, // 0x262 --- 610
    UnsignedLesspFP = 0o1143, // 0x263 --- 611
    UnsignedLesspLP = 0o1144, // 0x264 --- 612
    UnsignedLesspSP = 0o1145, // 0x265 --- 613
    UnsignedLesspImmediate = 0o1146, // 0x266 --- 614
    UnsignedLesspPop = 0o1147, // 0x267 --- 615
    UnsignedLesspNoPopFP = 0o1150, // 0x268 --- 616
    UnsignedLesspNoPopLP = 0o1151, // 0x269 --- 617
    UnsignedLesspNoPopSP = 0o1152, // 0x26A --- 618
    UnsignedLesspNoPopImmediate = 0o1153, // 0x26B --- 619
    UnsignedLesspNoPopPop = 0o1154, // 0x26C --- 620
    PopFP = 0o1155, // 0x26D --- 621
    PopLP = 0o1156, // 0x26E --- 622
    PopSP = 0o1157, // 0x26F --- 623
    PopImmediate = 0o1160, // 0x270 --- 624
    PopPop = 0o1161, // 0x271 --- 625
    MovemFP = 0o1162, // 0x272 --- 626
    MovemLP = 0o1163, // 0x273 --- 627
    MovemSP = 0o1164, // 0x274 --- 628
    MovemImmediate = 0o1165, // 0x275 --- 629
    MovemPop = 0o1166, // 0x276 --- 630
    MergeCdrNoPopFP = 0o1167, // 0x277 --- 631
    MergeCdrNoPopLP = 0o1170, // 0x278 --- 632
    MergeCdrNoPopSP = 0o1171, // 0x279 --- 633
    MergeCdrNoPopImmediate = 0o1172, // 0x27A --- 634
    MergeCdrNoPopPop = 0o1173, // 0x27B --- 635
    FastAref1FP = 0o1174, // 0x27C --- 636
    FastAref1LP = 0o1175, // 0x27D --- 637
    FastAref1SP = 0o1176, // 0x27E --- 638
    FastAref1Immediate = 0o1177, // 0x27F --- 639
    FastAref1Pop = 0o1200, // 0x280 --- 640
    FastAset1FP = 0o1201, // 0x281 --- 641
    FastAset1LP = 0o1202, // 0x282 --- 642
    FastAset1SP = 0o1203, // 0x283 --- 643
    FastAset1Immediate = 0o1204, // 0x284 --- 644
    FastAset1Pop = 0o1205, // 0x285 --- 645
    StackBltAddressFP = 0o1206, // 0x286 --- 646
    StackBltAddressLP = 0o1207, // 0x287 --- 647
    StackBltAddressSP = 0o1210, // 0x288 --- 648
    StackBltAddressImmediate = 0o1211, // 0x289 --- 649
    StackBltAddressPop = 0o1212, // 0x28A --- 650
    Dpb = 0o1213, // 0x28B --- 651
    CharDpb = 0o1214, // 0x28C --- 652
    PDpb = 0o1215, // 0x28D --- 653
    PTagDpb = 0o1216, // 0x28E --- 654
    LoopIncrementTosLessThan = 0o1217, // 0x28F --- 655
    CatchOpen = 0o1220, // 0x290 --- 656
    Hack = 0o1221, // 0x291 --- 657
    PushNull = 0o1222, // 0x292 --- 658
    PushMonitorForward = 0o1223, // 0x293 --- 659
    PushHeaderP = 0o1224, // 0x294 --- 660
    PushHeaderI = 0o1225, // 0x295 --- 661
    PushExternalValueCellPointer = 0o1226, // 0x296 --- 662
    PushOneQForward = 0o1227, // 0x297 --- 663
    PushHeaderForward = 0o1230, // 0x298 --- 664
    PushElementForward = 0o1231, // 0x299 --- 665
    PushFixnum = 0o1232, // 0x29A --- 666
    PushSmallRatio = 0o1233, // 0x29B --- 667
    PushSingleFloat = 0o1234, // 0x29C --- 668
    PushDoubleFloat = 0o1235, // 0x29D --- 669
    PushBignum = 0o1236, // 0x29E --- 670
    PushBigRatio = 0o1237, // 0x29F --- 671
    PushComplex = 0o1240, // 0x2A0 --- 672
    PushSpareNumber = 0o1241, // 0x2A1 --- 673
    PushInstance = 0o1242, // 0x2A2 --- 674
    PushListInstance = 0o1243, // 0x2A3 --- 675
    PushArrayInstance = 0o1244, // 0x2A4 --- 676
    PushStringInstance = 0o1245, // 0x2A5 --- 677
    PushNil = 0o1246, // 0x2A6 --- 678
    PushList = 0o1247, // 0x2A7 --- 679
    PushArray = 0o1250, // 0x2A8 --- 680
    PushString = 0o1251, // 0x2A9 --- 681
    PushSymbol = 0o1252, // 0x2AA --- 682
    PushLocative = 0o1253, // 0x2AB --- 683
    PushLexicalClosure = 0o1254, // 0x2AC --- 684
    PushDynamicClosure = 0o1255, // 0x2AD --- 685
    PushCompiledFunction = 0o1256, // 0x2AE --- 686
    PushGenericFunction = 0o1257, // 0x2AF --- 687
    PushSparePointer1 = 0o1260, // 0x2B0 --- 688
    PushSparePointer2 = 0o1261, // 0x2B1 --- 689
    PushPhysicalAddress = 0o1262, // 0x2B2 --- 690
    PushSpareImmediate1 = 0o1263, // 0x2B3 --- 691
    PushBoundLocation = 0o1264, // 0x2B4 --- 692
    PushCharacter = 0o1265, // 0x2B5 --- 693
    PushLogicVariable = 0o1266, // 0x2B6 --- 694
    PushGcForward = 0o1267, // 0x2B7 --- 695
    PushEvenPc = 0o1270, // 0x2B8 --- 696
    PushOddPc = 0o1271, // 0x2B9 --- 697
    CallCompiledEven = 0o1272, // 0x2BA --- 698
    CallCompiledOdd = 0o1273, // 0x2BB --- 699
    CallIndirect = 0o1274, // 0x2BC --- 700
    CallGeneric = 0o1275, // 0x2BD --- 701
    CallCompiledEvenPrefetch = 0o1276, // 0x2BE --- 702
    CallCompiledOddPrefetch = 0o1277, // 0x2BF --- 703
    CallIndirectPrefetch = 0o1300, // 0x2C0 --- 704
    CallGenericPrefetch = 0o1301, // 0x2C1 --- 705
    PushPackedInstruction60 = 0o1302, // 0x2C2 --- 706
    PushPackedInstruction61 = 0o1303, // 0x2C3 --- 707
    PushPackedInstruction62 = 0o1304, // 0x2C4 --- 708
    PushPackedInstruction63 = 0o1305, // 0x2C5 --- 709
    PushPackedInstruction64 = 0o1306, // 0x2C6 --- 710
    PushPackedInstruction65 = 0o1307, // 0x2C7 --- 711
    PushPackedInstruction66 = 0o1310, // 0x2C8 --- 712
    PushPackedInstruction67 = 0o1311, // 0x2C9 --- 713
    PushPackedInstruction70 = 0o1312, // 0x2CA --- 714
    PushPackedInstruction71 = 0o1313, // 0x2CB --- 715
    PushPackedInstruction72 = 0o1314, // 0x2CC --- 716
    PushPackedInstruction73 = 0o1315, // 0x2CD --- 717
    PushPackedInstruction74 = 0o1316, // 0x2CE --- 718
    PushPackedInstruction75 = 0o1317, // 0x2CF --- 719
    PushPackedInstruction76 = 0o1320, // 0x2D0 --- 720
    PushPackedInstruction77 = 0o1321, // 0x2D1 --- 721
    InstructionCacheLookup = 0o1322, // 0x2D2 --- 722
    #[default] // Default constructor
    IllegalInstruction = 0o1323, // 0x2D3 --- 723
}

impl fmt::Display for IvoryDispatch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

#[derive(Default, Copy, Clone, Debug, PartialEq, Eq, FromPrimitive, TryFromPrimitive)]
#[repr(u32)]
pub enum IvoryDispatch2 {
    CarFP = 0,
    CarLP,
    CarSP,
    CarImmediate,
    CarPop,
    CdrFP,
    CdrLP,
    CdrSP,
    CdrImmediate,
    CdrPop,
    EndpFP,
    EndpLP,
    EndpSP,
    EndpImmediate,
    EndpPop,
    Setup1dArrayFP,
    Setup1dArrayLP,
    Setup1dArraySP,
    Setup1dArrayImmediate,
    Setup1dArrayPop,
    SetupForce1dArrayFP,
    SetupForce1dArrayLP,
    SetupForce1dArraySP,
    SetupForce1dArrayImmediate,
    SetupForce1dArrayPop,
    BindLocativeFP,
    BindLocativeLP,
    BindLocativeSP,
    BindLocativeImmediate,
    BindLocativePop,
    RestoreBindingStackFP,
    RestoreBindingStackLP,
    RestoreBindingStackSP,
    RestoreBindingStackImmediate,
    RestoreBindingStackPop,
    EphemeralpFP,
    EphemeralpLP,
    EphemeralpSP,
    EphemeralpImmediate,
    EphemeralpPop,
    StartCallFP,
    StartCallLP,
    StartCallSP,
    StartCallImmediate,
    StartCallPop,
    JumpFP,
    JumpLP,
    JumpSP,
    JumpImmediate,
    JumpPop,
    TagFP,
    TagLP,
    TagSP,
    TagImmediate,
    TagPop,
    DereferenceFP,
    DereferenceLP,
    DereferenceSP,
    DereferenceImmediate,
    DereferencePop,
    LogicTailTestFP,
    LogicTailTestLP,
    LogicTailTestSP,
    LogicTailTestImmediate,
    LogicTailTestPop,
    ProcBreakpointFP,
    ProcBreakpointLP,
    ProcBreakpointSP,
    ProcBreakpointImmediate,
    ProcBreakpointPop,
    PushLexicalVarFP,
    PushLexicalVarLP,
    PushLexicalVarSP,
    PushLexicalVarImmediate,
    PushLexicalVarPop,
    Block0WriteFP,
    Block0WriteLP,
    Block0WriteSP,
    Block0WriteImmediate,
    Block0WritePop,
    Block1WriteFP,
    Block1WriteLP,
    Block1WriteSP,
    Block1WriteImmediate,
    Block1WritePop,
    Block2WriteFP,
    Block2WriteLP,
    Block2WriteSP,
    Block2WriteImmediate,
    Block2WritePop,
    Block3WriteFP,
    Block3WriteLP,
    Block3WriteSP,
    Block3WriteImmediate,
    Block3WritePop,
    ZeropFP,
    ZeropLP,
    ZeropSP,
    ZeropImmediate,
    ZeropPop,
    MinuspFP,
    MinuspLP,
    MinuspSP,
    MinuspImmediate,
    MinuspPop,
    PluspFP,
    PluspLP,
    PluspSP,
    PluspImmediate,
    PluspPop,
    TypeMember,
    TypeMemberNoPop,
    LocateLocals,
    CatchClose,
    GenericDispatch,
    MessageDispatch,
    CheckPreemptRequest,
    PushGlobalLogicVariable,
    NoOp,
    Halt,
    BranchTrue,
    BranchTrueElseExtraPop,
    BranchTrueAndExtraPop,
    BranchTrueExtraPop,
    BranchTrueNoPop,
    BranchTrueAndNoPop,
    BranchTrueElseNoPop,
    BranchTrueAndNoPopElseNoPopExtraPop,
    BranchFalse,
    BranchFalseElseExtraPop,
    BranchFalseAndExtraPop,
    BranchFalseExtraPop,
    BranchFalseNoPop,
    BranchFalseAndNoPop,
    BranchFalseElseNoPop,
    BranchFalseAndNoPopElseNoPopExtraPop,
    PushFP,
    PushLP,
    PushSP,
    PushImmediate,
    PushPop,
    PushNNils,
    PushAddressSpRelativeFP,
    PushAddressSpRelativeLP,
    PushAddressSpRelativeSP,
    PushAddressSpRelativeImmediate,
    PushAddressSpRelativePop,
    PushLocalLogicVariablesFP,
    PushLocalLogicVariablesLP,
    PushLocalLogicVariablesSP,
    PushLocalLogicVariablesImmediate,
    PushLocalLogicVariablesPop,
    ReturnMultipleFP,
    ReturnMultipleLP,
    ReturnMultipleSP,
    ReturnMultipleImmediate,
    ReturnMultiplePop,
    ReturnKludgeFP,
    ReturnKludgeLP,
    ReturnKludgeSP,
    ReturnKludgeImmediate,
    ReturnKludgePop,
    TakeValues,
    UnbindNImmediate,
    UnbindNPop,
    PushInstanceVariable,
    PushAddressInstanceVariable,
    PushInstanceVariableOrdered,
    PushAddressInstanceVariableOrdered,
    UnaryMinusFP,
    UnaryMinusLP,
    UnaryMinusSP,
    UnaryMinusImmediate,
    UnaryMinusPop,
    ReturnSingleNIL,
    ReturnSingleT,
    ReturnSingleTOS,
    MemoryRead,
    MemoryReadAddress,
    Block0Read,
    Block1Read,
    Block2Read,
    Block3Read,
    Block0ReadShift,
    Block1ReadShift,
    Block2ReadShift,
    Block3ReadShift,
    Block0ReadTest,
    Block1ReadTest,
    Block2ReadTest,
    Block3ReadTest,
    FinishCallN,
    FinishCallNApply,
    FinishCallTos,
    FinishCallTosApply,
    SetToCarFP,
    SetToCarLP,
    SetToCarSP,
    SetToCarImmediate,
    SetToCarPop,
    SetToCdrFP,
    SetToCdrLP,
    SetToCdrSP,
    SetToCdrImmediate,
    SetToCdrPop,
    SetToCdrPushCarFP,
    SetToCdrPushCarLP,
    SetToCdrPushCarSP,
    SetToCdrPushCarImmediate,
    SetToCdrPushCarPop,
    IncrementFP,
    IncrementLP,
    IncrementSP,
    IncrementImmediate,
    IncrementPop,
    DecrementFP,
    DecrementLP,
    DecrementSP,
    DecrementImmediate,
    DecrementPop,
    PointerIncrementFP,
    PointerIncrementLP,
    PointerIncrementSP,
    PointerIncrementImmediate,
    PointerIncrementPop,
    SetCdrCode1FP,
    SetCdrCode1LP,
    SetCdrCode1SP,
    SetCdrCode1Immediate,
    SetCdrCode1Pop,
    SetCdrCode2FP,
    SetCdrCode2LP,
    SetCdrCode2SP,
    SetCdrCode2Immediate,
    SetCdrCode2Pop,
    PushAddressFP,
    PushAddressLP,
    PushAddressSP,
    PushAddressImmediate,
    PushAddressPop,
    SetSpToAddressFP,
    SetSpToAddressLP,
    SetSpToAddressSP,
    SetSpToAddressImmediate,
    SetSpToAddressPop,
    SetSpToAddressSaveTosFP,
    SetSpToAddressSaveTosLP,
    SetSpToAddressSaveTosSP,
    SetSpToAddressSaveTosImmediate,
    SetSpToAddressSaveTosPop,
    ReadInternalRegister,
    WriteInternalRegister,
    CoprocessorRead,
    CoprocessorWrite,
    Block0ReadAluFP,
    Block0ReadAluLP,
    Block0ReadAluSP,
    Block0ReadAluImmediate,
    Block0ReadAluPop,
    Block1ReadAluFP,
    Block1ReadAluLP,
    Block1ReadAluSP,
    Block1ReadAluImmediate,
    Block1ReadAluPop,
    Block2ReadAluFP,
    Block2ReadAluLP,
    Block2ReadAluSP,
    Block2ReadAluImmediate,
    Block2ReadAluPop,
    Block3ReadAluFP,
    Block3ReadAluLP,
    Block3ReadAluSP,
    Block3ReadAluImmediate,
    Block3ReadAluPop,
    Ldb,
    CharLdb,
    PLdb,
    PTagLdb,
    Branch,
    LoopDecrementTos,
    EntryRestAccepted,
    EntryRestNotAccepted,
    RplacaFP,
    RplacaLP,
    RplacaSP,
    RplacaImmediate,
    RplacaPop,
    RplacdFP,
    RplacdLP,
    RplacdSP,
    RplacdImmediate,
    RplacdPop,
    MultiplyFP,
    MultiplyLP,
    MultiplySP,
    MultiplyImmediate,
    MultiplyPop,
    QuotientFP,
    QuotientLP,
    QuotientSP,
    QuotientImmediate,
    QuotientPop,
    CeilingFP,
    CeilingLP,
    CeilingSP,
    CeilingImmediate,
    CeilingPop,
    FloorFP,
    FloorLP,
    FloorSP,
    FloorImmediate,
    FloorPop,
    TruncateFP,
    TruncateLP,
    TruncateSP,
    TruncateImmediate,
    TruncatePop,
    RoundFP,
    RoundLP,
    RoundSP,
    RoundImmediate,
    RoundPop,
    RationalQuotientFP,
    RationalQuotientLP,
    RationalQuotientSP,
    RationalQuotientImmediate,
    RationalQuotientPop,
    MinFP,
    MinLP,
    MinSP,
    MinImmediate,
    MinPop,
    MaxFP,
    MaxLP,
    MaxSP,
    MaxImmediate,
    MaxPop,
    AluFP,
    AluLP,
    AluSP,
    AluImmediate,
    AluPop,
    LogandFP,
    LogandLP,
    LogandSP,
    LogandImmediate,
    LogandPop,
    LogxorFP,
    LogxorLP,
    LogxorSP,
    LogxorImmediate,
    LogxorPop,
    LogiorFP,
    LogiorLP,
    LogiorSP,
    LogiorImmediate,
    LogiorPop,
    RotFP,
    RotLP,
    RotSP,
    RotImmediate,
    RotPop,
    LshFP,
    LshLP,
    LshSP,
    LshImmediate,
    LshPop,
    MultiplyDoubleFP,
    MultiplyDoubleLP,
    MultiplyDoubleSP,
    MultiplyDoubleImmediate,
    MultiplyDoublePop,
    LshcBignumStepFP,
    LshcBignumStepLP,
    LshcBignumStepSP,
    LshcBignumStepImmediate,
    LshcBignumStepPop,
    StackBltFP,
    StackBltLP,
    StackBltSP,
    StackBltImmediate,
    StackBltPop,
    RgetfFP,
    RgetfLP,
    RgetfSP,
    RgetfImmediate,
    RgetfPop,
    MemberFP,
    MemberLP,
    MemberSP,
    MemberImmediate,
    MemberPop,
    AssocFP,
    AssocLP,
    AssocSP,
    AssocImmediate,
    AssocPop,
    PointerPlusFP,
    PointerPlusLP,
    PointerPlusSP,
    PointerPlusImmediate,
    PointerPlusPop,
    PointerDifferenceFP,
    PointerDifferenceLP,
    PointerDifferenceSP,
    PointerDifferenceImmediate,
    PointerDifferencePop,
    AshFP,
    AshLP,
    AshSP,
    AshImmediate,
    AshPop,
    StoreConditionalFP,
    StoreConditionalLP,
    StoreConditionalSP,
    StoreConditionalImmediate,
    StoreConditionalPop,
    MemoryWriteFP,
    MemoryWriteLP,
    MemoryWriteSP,
    MemoryWriteImmediate,
    MemoryWritePop,
    PStoreContentsFP,
    PStoreContentsLP,
    PStoreContentsSP,
    PStoreContentsImmediate,
    PStoreContentsPop,
    BindLocativeToValueFP,
    BindLocativeToValueLP,
    BindLocativeToValueSP,
    BindLocativeToValueImmediate,
    BindLocativeToValuePop,
    UnifyFP,
    UnifyLP,
    UnifySP,
    UnifyImmediate,
    UnifyPop,
    PopLexicalVarFP,
    PopLexicalVarLP,
    PopLexicalVarSP,
    PopLexicalVarImmediate,
    PopLexicalVarPop,
    MovemLexicalVarFP,
    MovemLexicalVarLP,
    MovemLexicalVarSP,
    MovemLexicalVarImmediate,
    MovemLexicalVarPop,
    EqualNumberFP,
    EqualNumberLP,
    EqualNumberSP,
    EqualNumberImmediate,
    EqualNumberPop,
    LesspFP,
    LesspLP,
    LesspSP,
    LesspImmediate,
    LesspPop,
    GreaterpFP,
    GreaterpLP,
    GreaterpSP,
    GreaterpImmediate,
    GreaterpPop,
    EqlFP,
    EqlLP,
    EqlSP,
    EqlImmediate,
    EqlPop,
    EqualNumberNoPopFP,
    EqualNumberNoPopLP,
    EqualNumberNoPopSP,
    EqualNumberNoPopImmediate,
    EqualNumberNoPopPop,
    LesspNoPopFP,
    LesspNoPopLP,
    LesspNoPopSP,
    LesspNoPopImmediate,
    LesspNoPopPop,
    GreaterpNoPopFP,
    GreaterpNoPopLP,
    GreaterpNoPopSP,
    GreaterpNoPopImmediate,
    GreaterpNoPopPop,
    EqlNoPopFP,
    EqlNoPopLP,
    EqlNoPopSP,
    EqlNoPopImmediate,
    EqlNoPopPop,
    EqFP,
    EqLP,
    EqSP,
    EqImmediate,
    EqPop,
    LogtestFP,
    LogtestLP,
    LogtestSP,
    LogtestImmediate,
    LogtestPop,
    EqNoPopFP,
    EqNoPopLP,
    EqNoPopSP,
    EqNoPopImmediate,
    EqNoPopPop,
    LogtestNoPopFP,
    LogtestNoPopLP,
    LogtestNoPopSP,
    LogtestNoPopImmediate,
    LogtestNoPopPop,
    AddFP,
    AddLP,
    AddSP,
    AddImmediate,
    AddPop,
    SubFP,
    SubLP,
    SubSP,
    SubImmediate,
    SubPop,
    Dispatch32BitPlusFP,
    Dispatch32BitPlusLP,
    Dispatch32BitPlusSP,
    Dispatch32BitPlusImmediate,
    Dispatch32BitPlusPop,
    Dispatch32BitDifferenceFP,
    Dispatch32BitDifferenceLP,
    Dispatch32BitDifferenceSP,
    Dispatch32BitDifferenceImmediate,
    Dispatch32BitDifferencePop,
    AddBignumStepFP,
    AddBignumStepLP,
    AddBignumStepSP,
    AddBignumStepImmediate,
    AddBignumStepPop,
    SubBignumStepFP,
    SubBignumStepLP,
    SubBignumStepSP,
    SubBignumStepImmediate,
    SubBignumStepPop,
    MultiplyBignumStepFP,
    MultiplyBignumStepLP,
    MultiplyBignumStepSP,
    MultiplyBignumStepImmediate,
    MultiplyBignumStepPop,
    DivideBignumStepFP,
    DivideBignumStepLP,
    DivideBignumStepSP,
    DivideBignumStepImmediate,
    DivideBignumStepPop,
    Aset1FP,
    Aset1LP,
    Aset1SP,
    Aset1Immediate,
    Aset1Pop,
    AllocateListBlockFP,
    AllocateListBlockLP,
    AllocateListBlockSP,
    AllocateListBlockImmediate,
    AllocateListBlockPop,
    Aref1FP,
    Aref1LP,
    Aref1SP,
    Aref1Immediate,
    Aref1Pop,
    Aloc1FP,
    Aloc1LP,
    Aloc1SP,
    Aloc1Immediate,
    Aloc1Pop,
    StoreArrayLeaderFP,
    StoreArrayLeaderLP,
    StoreArrayLeaderSP,
    StoreArrayLeaderImmediate,
    StoreArrayLeaderPop,
    AllocateStructureBlockFP,
    AllocateStructureBlockLP,
    AllocateStructureBlockSP,
    AllocateStructureBlockImmediate,
    AllocateStructureBlockPop,
    ArrayLeaderFP,
    ArrayLeaderLP,
    ArrayLeaderSP,
    ArrayLeaderImmediate,
    ArrayLeaderPop,
    AlocLeaderFP,
    AlocLeaderLP,
    AlocLeaderSP,
    AlocLeaderImmediate,
    AlocLeaderPop,
    PopInstanceVariable,
    MovemInstanceVariable,
    PopInstanceVariableOrdered,
    MovemInstanceVariableOrdered,
    InstanceRefFP,
    InstanceRefLP,
    InstanceRefSP,
    InstanceRefImmediate,
    InstanceRefPop,
    InstanceSetFP,
    InstanceSetLP,
    InstanceSetSP,
    InstanceSetImmediate,
    InstanceSetPop,
    InstanceLocFP,
    InstanceLocLP,
    InstanceLocSP,
    InstanceLocImmediate,
    InstanceLocPop,
    SetTagFP,
    SetTagLP,
    SetTagSP,
    SetTagImmediate,
    SetTagPop,
    UnsignedLesspFP,
    UnsignedLesspLP,
    UnsignedLesspSP,
    UnsignedLesspImmediate,
    UnsignedLesspPop,
    UnsignedLesspNoPopFP,
    UnsignedLesspNoPopLP,
    UnsignedLesspNoPopSP,
    UnsignedLesspNoPopImmediate,
    UnsignedLesspNoPopPop,
    PopFP,
    PopLP,
    PopSP,
    PopImmediate,
    PopPop,
    MovemFP,
    MovemLP,
    MovemSP,
    MovemImmediate,
    MovemPop,
    MergeCdrNoPopFP,
    MergeCdrNoPopLP,
    MergeCdrNoPopSP,
    MergeCdrNoPopImmediate,
    MergeCdrNoPopPop,
    FastAref1FP,
    FastAref1LP,
    FastAref1SP,
    FastAref1Immediate,
    FastAref1Pop,
    FastAset1FP,
    FastAset1LP,
    FastAset1SP,
    FastAset1Immediate,
    FastAset1Pop,
    StackBltAddressFP,
    StackBltAddressLP,
    StackBltAddressSP,
    StackBltAddressImmediate,
    StackBltAddressPop,
    Dpb,
    CharDpb,
    PDpb,
    PTagDpb,
    LoopIncrementTosLessThan,
    CatchOpen,
    Hack,
    PushNull,
    PushMonitorForward,
    PushHeaderP,
    PushHeaderI,
    PushExternalValueCellPointer,
    PushOneQForward,
    PushHeaderForward,
    PushElementForward,
    PushFixnum,
    PushSmallRatio,
    PushSingleFloat,
    PushDoubleFloat,
    PushBignum,
    PushBigRatio,
    PushComplex,
    PushSpareNumber,
    PushInstance,
    PushListInstance,
    PushArrayInstance,
    PushStringInstance,
    PushNil,
    PushList,
    PushArray,
    PushString,
    PushSymbol,
    PushLocative,
    PushLexicalClosure,
    PushDynamicClosure,
    PushCompiledFunction,
    PushGenericFunction,
    PushSparePointer1,
    PushSparePointer2,
    PushPhysicalAddress,
    PushSpareImmediate1,
    PushBoundLocation,
    PushCharacter,
    PushLogicVariable,
    PushGcForward,
    PushEvenPc,
    PushOddPc,
    CallCompiledEven,
    CallCompiledOdd,
    CallIndirect,
    CallGeneric,
    CallCompiledEvenPrefetch,
    CallCompiledOddPrefetch,
    CallIndirectPrefetch,
    CallGenericPrefetch,
    PushPackedInstruction60,
    PushPackedInstruction61,
    PushPackedInstruction62,
    PushPackedInstruction63,
    PushPackedInstruction64,
    PushPackedInstruction65,
    PushPackedInstruction66,
    PushPackedInstruction67,
    PushPackedInstruction70,
    PushPackedInstruction71,
    PushPackedInstruction72,
    PushPackedInstruction73,
    PushPackedInstruction74,
    PushPackedInstruction75,
    PushPackedInstruction76,
    PushPackedInstruction77,
    InstructionCacheLookup,
    #[default] // Default constructor
    IllegalInstruction,
}

impl fmt::Display for IvoryDispatch2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}
