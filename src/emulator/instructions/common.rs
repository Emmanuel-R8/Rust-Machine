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

use crate::{common::constants::OpCode, hardware::machine::VirtualMachine};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ImmediateArgumentType {
    Undefined,
    NotApplicable,
    Signed,
    Unsigned,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum InstructionFamily {
    Undefined,
    List,
    Interruptible,
    Predicate,
    Numeric,
    DataMovement,
    FieldExtraction,
    Array,
    BranchLoop,
    Block,
    FunctionCalling,
    Binding,
    Catch,
    LexicalVariableAccess,
    InstanceVariableAccess,
    Subprimitive,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum InstructionFormat {
    Undefined,
    OperandFromStack,
    OperandFromStackImmediate,
    Immediate10Bits,
    FieldExtraction,
    EntryInstruction,
}

#[derive(Clone)]
pub struct Instruction {
    pub name: String,
    pub family: InstructionFamily,
    pub format: InstructionFormat,
    pub arg_count: u32,
    pub ret_count: u32,
    pub immediate_arg_type: ImmediateArgumentType,
    pub opcode: u32,
    // Code to execute when this instruction is executed.
    pub exec: Option<fn(&mut VirtualMachine) -> &mut VirtualMachine>,
}

impl Default for Instruction {
    fn default() -> Self {
        return Self {
            family: InstructionFamily::Undefined,
            format: InstructionFormat::Undefined,
            arg_count: 0,
            ret_count: 0,
            immediate_arg_type: ImmediateArgumentType::Undefined,
            name: "unknown".to_string(),
            opcode: 0,
            exec: None,
        };
    }
}

impl Instruction {
    pub fn new() -> Self {
        let i: Instruction = Instruction::default();
        return i;
    }

    pub fn set_family(mut self, family: InstructionFamily) -> Self {
        self.family = family;
        return self;
    }

    pub fn set_format(mut self, format: InstructionFormat) -> Self {
        self.format = format;
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
    pub fn set_name(mut self, name: String) -> Self {
        self.name = name;
        return self;
    }
    pub fn set_opcode(mut self, opcode: OpCode) -> Self {
        self.opcode = opcode as u32;
        return self;
    }
    pub fn set_exec(
        mut self,
        exec: Option<fn(&mut VirtualMachine) -> &mut VirtualMachine>,
    ) -> Self {
        self.exec = exec;
        return self;
    }
}
