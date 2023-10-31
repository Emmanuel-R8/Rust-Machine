static INSTRUCTIONS_LIST: [Instruction; 7] = [
    Instruction {
        family: InstructionFamily::LIST,
        opcode: 0o000,
        name: "car",
        is_implemented: false,
    },
    Instruction {
        family: InstructionFamily::LIST,
        opcode: 0o001,
        name: "cdr",
        is_implemented: false,
    },
    Instruction {
        family: InstructionFamily::LIST,
        opcode: 0o140,
        name: "set-to-car",
        is_implemented: false,
    },
    Instruction {
        family: InstructionFamily::LIST,
        opcode: 0o141,
        name: "set-to-cdr",
        is_implemented: false,
    },
    Instruction {
        family: InstructionFamily::LIST,
        opcode: 0o142,
        name: "set-to-cdr-push-car",
        is_implemented: false,
    },
    Instruction {
        family: InstructionFamily::LIST,
        opcode: 0o200,
        name: "rplaca",
        is_implemented: false,
    },
    Instruction {
        family: InstructionFamily::LIST,
        opcode: 0o201,
        name: "rplacd",
        is_implemented: false,
    },
];
