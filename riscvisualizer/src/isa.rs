#[derive(Debug, PartialEq)]
pub enum Register {
    X0 = 0,
    X1 = 1,
    X2 = 2,
    X3 = 3,
    X4 = 4,
    X5 = 5,
    X6 = 6,
    X7 = 7,
    X8 = 8,
    X9 = 9,
    X10 = 10,
    X11 = 11,
    X12 = 12,
    X13 = 13,
    X14 = 14,
    X15 = 15,
    X16 = 16,
    X17 = 17,
    X18 = 18,
    X19 = 19,
    X20 = 20,
    X21 = 21,
    X22 = 22,
    X23 = 23,
    X24 = 24,
    X25 = 25,
    X26 = 26,
    X27 = 27,
    X28 = 28,
    X29 = 29,
    X30 = 30,
    X31 = 31,
}

impl Register {
    pub fn as_num(self) -> usize {
        self as usize
    }

    pub fn from_num(num: u32) -> Register {
        match num {
            0 => Register::X0,
            1 => Register::X1,
            2 => Register::X2,
            3 => Register::X3,
            4 => Register::X4,
            5 => Register::X5,
            6 => Register::X6,
            7 => Register::X7,
            8 => Register::X8,
            9 => Register::X9,
            10 => Register::X10,
            11 => Register::X11,
            12 => Register::X12,
            13 => Register::X13,
            14 => Register::X14,
            15 => Register::X15,
            16 => Register::X16,
            17 => Register::X17,
            18 => Register::X18,
            19 => Register::X19,
            20 => Register::X20,
            21 => Register::X21,
            22 => Register::X22,
            23 => Register::X23,
            24 => Register::X24,
            25 => Register::X25,
            26 => Register::X26,
            27 => Register::X27,
            28 => Register::X28,
            29 => Register::X29,
            30 => Register::X30,
            31 => Register::X31,
            _ => panic!("Invalid register number: {}", num),
        }
    }
}

pub enum UOpcode {
    LUI,
    AUIPC,
}
pub enum UJOpcode {
    JAL,
}
pub enum SBOpcode {
    BEQ,
    BNE,
    BLT,
    BGE,
    BLTU,
    BGEU,
}
pub enum SOpcode {
    SB,
    SH,
    SW,
}
pub enum IOpcode {
    JALR,
    LB,
    LH,
    LW,
    LBU,
    LHU,
    ADDI,
    SLTI,
    SLTIU,
    XORI,
    ORI,
    ANDI,
    SCALL,
}
pub enum ROpcode {
    ADD,
    SUB,
    SLL,
    SLT,
    SLTU,
    XOR,
    SRL,
    SRA,
    OR,
    AND,
}
pub enum RShiftOpcode {
    SLLI,
    SRLI,
    SRAI,
}

pub enum Instruction {
    RShift {
        opcode: RShiftOpcode,
        rd: Register,
        rs1: Register,
        shamt: u32,
    },
    R {
        opcode: ROpcode,
        rd: Register,
        rs1: Register,
        rs2: Register,
    },
    I {
        opcode: IOpcode,
        rd: Register,
        rs1: Register,
        imm: u32,
    },
    S {
        opcode: SOpcode,
        rs1: Register,
        rs2: Register,
        imm: u32,
    },
    SB {
        opcode: SBOpcode,
        rs1: Register,
        rs2: Register,
        imm: u32,
    },
    U {
        opcode: UOpcode,
        rd: Register,
        imm: u32,
    },
    UJ {
        opcode: UJOpcode,
        rd: Register,
        imm: u32,
    },
}
