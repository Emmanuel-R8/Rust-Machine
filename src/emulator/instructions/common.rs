// Instructions are normally executed in the order in which they are stored in
// memory. Since full-word instructions cannot cross word boundaries, it would
// occasionally be necessary to insert a no-op instruction in places where a
// full-word instruction or constant followed a half-word instruction that did not
// fall on an odd halfword address. This costs address space, I Cache space, and
// possibly execution time to execute the no-op.

// ******
// NOTE: A full word address is split between an __even__ half-world address,
// followed by an __odd__ half-word address.
// ******

// The `cdr` code field of each word executed contains sequencing information to
// minimize this waste. The `cdr` code takes on one of four values, which specify how
// much the PC is incremented after executing an instruction from this word. Note
// that the PC contains a half-word address.

// | CDR Code | PC Increment  | Comment                 |
// |:----------:|:---------:|:----------------------------|
// | 0  | +1  | Normal instruction sequencing                   |
// | 1  | Illegal  | Fence; marks end of compiled function                |
// | 2   | -1 | On some constants. |
// | 3   | +2 PC even | Before some constants, on some constants |
// |   | +3 PC odd |  |
// :CDR header {tbl-colwidths="[15, 15, 70]"}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ImmediateArgumentType {
    UNDEFINED,
    NOT_APPLICABLE,
    SIGNED,
    UNSIGNED,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum InstructionFamily {
    UNDEFINED,
    LIST,
    INTERRUPTIBLE,
    PREDICATE,
    NUMERIC,
    DATA_MOVEMENT,
    FIELD_EXTRACTION,
    ARRAY,
    BRANCH_LOOP,
    BLOCK,
    FUNCTION_CALLING,
    BINDING,
    CATCH,
    LEXICAL_VARIABLE_ACCESS,
    INSTANCE_VARIABLE_ACCESS,
    SUBPRIMITIVE,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Instruction<'a> {
    pub family: InstructionFamily,
    pub arg_count: u32,
    pub ret_count: u32,
    pub immediate_arg_type: ImmediateArgumentType,
    pub name: &'a str,
    pub opcode: u32,
    pub is_implemented: bool,
}

impl Default for Instruction<'static> {
    fn default() -> Self {
        return Self {
            family: InstructionFamily::UNDEFINED,
            arg_count: 0,
            ret_count: 0,
            immediate_arg_type: ImmediateArgumentType::UNDEFINED,
            name: "unknown",
            opcode: 0,
            is_implemented: false,
        };
    }
}

impl Instruction<'static> {
    pub fn new() -> Self {
        let i: Instruction<'static> = Instruction::default();
        return i;
    }

    pub fn set_family(mut self, family: InstructionFamily) -> Self {
        self.family = family;
        return self;
    }
    pub fn set_arg_count(mut self, arg_count: u32) -> Self {
        self.arg_count = arg_count;
        return self;
    }
    pub fn set_ret_count(mut self, ret_count: u32) -> Self {
        self.ret_count = ret_count;
        return self;
    }
    pub fn set_immediate_arg_type(mut self, immediate_arg_type: ImmediateArgumentType) -> Self {
        self.immediate_arg_type = immediate_arg_type;
        return self;
    }
    pub fn set_name(mut self, name: &'static str) -> Self {
        self.name = name;
        return self;
    }
    pub fn set_opcode(mut self, opcode: u32) -> Self {
        self.opcode = opcode;
        return self;
    }
    pub fn set_is_implemented(mut self, is_implemented: bool) -> Self {
        self.is_implemented = is_implemented;
        return self;
    }
}

const EMPTY_INSTRUCTION: Instruction = Instruction::default();
pub static mut INSTRUCTIONS_SET: [Instruction<'static>; 0o777] = [
    EMPTY_INSTRUCTION;
    0o777
];