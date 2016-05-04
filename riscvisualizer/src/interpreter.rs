use std;
use std::cell::RefCell;
use std::rc::Rc;
use std::slice;


use isa;
use memory::{self, MemoryAccess, MemoryInterface};
use types::{self, IsaType};

#[derive(Debug)]
pub struct RegisterFile {
    registers: [types::Word; 32],
}

impl RegisterFile {
    pub fn new() -> RegisterFile {
        RegisterFile {
            registers: [types::Word(0); 32],
        }
    }

    pub fn write_word<T: Into<isa::Register>>(&mut self, reg: T, value: types::Word) {
        // TODO: should be safe to use unchecked index
        let reg = reg.into();
        if reg == isa::Register::X0 { return; }
        self.registers[reg.as_num()] = value;
    }

    pub fn read_word<T: Into<isa::Register>>(&mut self, reg: T) -> types::Word {
        self.registers[reg.into().as_num()]
    }
}

pub struct Interpreter<'a> {
    register_file: RegisterFile,
    memory: Rc<RefCell<memory::Memory>>,
    cache: memory::Cache,
    program: &'a [isa::Instruction],
    pc: usize,
    actions: Vec<Action>,
}

pub enum Action {
    // read register - register, old value
    ReadRegister(isa::Register, types::Word),

    // written register - register, old value, value written
    WriteRegister(isa::Register, types::Word, types::Word),

    // PC before, PC after
    // Also EndOfInstruction
    Jump(usize, usize),

    // stall
    Stall(usize),

    // read cache

    // read memory

    // write cache

    // write memory - address, old value, new value
    WriteMemory(types::Word, types::Word, types::Word),

    Error {

    }
}

impl<'a> Interpreter<'a> {
    pub fn new(memory_words: usize, program: &'a [isa::Instruction]) -> Interpreter<'a> {
        let memory = Rc::new(RefCell::new(memory::Memory::new(memory_words)));

        Interpreter {
            register_file: RegisterFile::new(),
            memory: memory.clone(),
            cache: memory::Cache::new(memory.clone(), 4, 4),
            program: program,
            pc: 0,
            actions: Vec::new(),
        }
    }

    pub fn iter(&self) -> std::slice::Iter<Action> {
        self.actions.iter()
    }

    pub fn step(&mut self) {
        let instruction = &self.program[self.pc];
        match *instruction {
            isa::Instruction::I {
                opcode,
                rd,
                rs1,
                imm,
            } => {
                use isa::IOpcode::*;

                let rs1_reg = rs1;
                let rs1 = self.register_file.read_word(rs1);
                self.actions.push(Action::ReadRegister(rs1_reg, rs1));

                let rd_orig = self.register_file.read_word(rd);
                let mut stall = 0;
                let action =
                    match opcode {
                        LB => {
                            let result = self.memory.borrow_mut().read_byte(rs1 + imm);
                            if let Ok(read) = result {
                                stall = read.1;
                                Action::WriteRegister(rd, rd_orig, read.0.as_word())
                            }
                            else {
                                Action::Error {}
                            }
                        }
                        LH => {
                            let result = self.memory.borrow_mut().read_halfword(rs1 + imm);
                            if let Ok(read) = result {
                                stall = read.1;
                                Action::WriteRegister(rd, rd_orig, read.0.as_word())
                            }
                            else {
                                Action::Error {}
                            }
                        }
                        LBU => {
                            let result = self.memory.borrow_mut().read_byte(rs1 + imm);
                            if let Ok(read) = result {
                                stall = read.1;
                                Action::WriteRegister(rd, rd_orig, read.0.as_signed_word().as_word())
                            }
                            else {
                                Action::Error {}
                            }
                        }
                        LHU => {
                            let result = self.memory.borrow_mut().read_halfword(rs1 + imm);
                            if let Ok(read) = result {
                                stall = read.1;
                                Action::WriteRegister(rd, rd_orig, read.0.as_signed_word().as_word())
                            }
                            else {
                                Action::Error {}
                            }
                        }
                        LW => {
                            let result = self.memory.borrow_mut().read_word(rs1 + imm);
                            if let Ok(read) = result {
                                stall = read.1;
                                Action::WriteRegister(rd, rd_orig, read.0)
                            }
                            else {
                                Action::Error {}
                            }
                        }
                        ADDI => {
                            Action::WriteRegister(rd, rd_orig, rs1 + imm)
                        }
                        SLTI => {
                            if rs1.as_signed() < types::SignedWord(imm as i32) {
                                Action::WriteRegister(rd, rd_orig, types::Word(1))
                            }
                            else {
                                Action::WriteRegister(rd, rd_orig, types::Word(0))
                            }
                        }
                        SLTIU => {
                            if rs1 < types::Word(imm) {
                                Action::WriteRegister(rd, rd_orig, types::Word(1))
                            }
                            else {
                                Action::WriteRegister(rd, rd_orig, types::Word(0))
                            }
                        }
                        XORI => {
                            Action::WriteRegister(rd, rd_orig, rs1 ^ imm)
                        }
                        ORI => {
                            Action::WriteRegister(rd, rd_orig, rs1 | imm)
                        }
                        ANDI => {
                            Action::WriteRegister(rd, rd_orig, rs1 & imm)
                        }
                        JALR => {
                            panic!("JALR not implemented");
                        }
                        SCALL => {
                            panic!("JALR not implemented");
                        }
                    };

                if stall > 0 {
                    self.actions.push(Action::Stall(stall));
                }

                match action {
                    Action::WriteRegister(target, _, value) => {
                        self.register_file.write_word(target, value);
                    }

                    _ => {},
                };

                self.actions.push(action);
                self.actions.push(Action::Jump(self.pc, self.pc + 1));
                self.pc += 1;
            }

            isa::Instruction::RShift {
                opcode,
                rd,
                rs1,
                shamt,
            } => {
                use isa::RShiftOpcode::*;

                let rs1_orig = self.register_file.read_word(rs1);
                let rd_orig = self.register_file.read_word(rd);
                self.actions.push(Action::ReadRegister(rs1, rs1_orig));

                let result = match opcode {
                    SLLI => {
                        rs1_orig << shamt
                    }
                    SRLI => {
                        rs1_orig >> shamt
                    }
                    SRAI => {
                        (rs1_orig.as_signed() >> (shamt as i32)).as_word()
                    }
                };

                self.register_file.write_word(rd, result);
                self.actions.push(Action::WriteRegister(rd, rd_orig, result));
                self.actions.push(Action::Jump(self.pc, self.pc + 1));
                self.pc += 1;
            }

            isa::Instruction::R {
                opcode,
                rd,
                rs1,
                rs2,
            } => {
                use isa::ROpcode::*;

                let rs1_orig = self.register_file.read_word(rs1);
                let rs2_orig = self.register_file.read_word(rs2);
                let rd_orig = self.register_file.read_word(rd);
                self.actions.push(Action::ReadRegister(rs1, rs1_orig));
                self.actions.push(Action::ReadRegister(rs2, rs2_orig));

                // TODO: use wrapping ops, etc
                let result = match opcode {
                    ADD => {
                        rs1_orig.wrapping_add(rs2_orig)
                    }
                    SUB => {
                        rs1_orig.wrapping_sub(rs2_orig)
                    }
                    SLL => {
                        rs1_orig << rs2_orig
                    }
                    SLT => {
                        if rs1_orig.as_signed() < rs2_orig.as_signed() {
                            types::Word(1)
                        }
                        else {
                            types::Word(0)
                        }
                    }
                    SLTU => {
                        if rs1_orig < rs2_orig {
                            types::Word(1)
                        }
                        else {
                            types::Word(0)
                        }
                    }
                    XOR => {
                        rs1_orig ^ rs2_orig
                    }
                    SRL => {
                        rs1_orig >> rs2_orig
                    }
                    SRA => {
                        (rs1_orig.as_signed() >> rs2_orig.as_signed()).as_word()
                    }
                    OR => {
                        rs1_orig | rs2_orig
                    }
                    AND => {
                        rs1_orig & rs2_orig
                    }
                };

                self.register_file.write_word(rd, result);
                self.actions.push(Action::WriteRegister(rd, rd_orig, result));
                self.actions.push(Action::Jump(self.pc, self.pc + 1));
                self.pc += 1;
            }

            isa::Instruction::S {
                opcode,
                rs1,
                rs2,
                imm,
            } => {
                use isa::SOpcode::*;
                let rs1_orig = self.register_file.read_word(rs1);
                let rs2_orig = self.register_file.read_word(rs2);
                self.actions.push(Action::ReadRegister(rs1, rs1_orig));
                self.actions.push(Action::ReadRegister(rs2, rs2_orig));

                let address = (rs1_orig.as_signed() + imm).as_word();
                let result = match opcode {
                    SW => self.memory.borrow_mut().write_word(address, rs2_orig),
                    SH => self.memory.borrow_mut().write_halfword(address, rs2_orig.as_halfword()).map(MemoryAccess::as_word),
                    SB => self.memory.borrow_mut().write_byte(address, rs2_orig.as_byte()).map(MemoryAccess::as_word),
                };

                let action = if let Ok(read) = result {
                    self.actions.push(Action::Stall(read.1));
                    Action::WriteMemory(address, read.0, rs2_orig)
                }
                else {
                    Action::Error {}
                };

                self.actions.push(action);
                self.actions.push(Action::Jump(self.pc, self.pc + 1));
                self.pc += 1;
            }

            isa::Instruction::SB {
                opcode,
                rs1,
                rs2,
                imm,
            } => {
                use isa::SBOpcode::*;
                let rs1_orig = self.register_file.read_word(rs1);
                let rs2_orig = self.register_file.read_word(rs2);
                self.actions.push(Action::ReadRegister(rs1, rs1_orig));
                self.actions.push(Action::ReadRegister(rs2, rs2_orig));

                let target = if match opcode {
                    BEQ => rs1_orig == rs2_orig,
                    BNE => rs1_orig != rs2_orig,
                    BLT => rs1_orig.as_signed() < rs2_orig.as_signed(),
                    BGE => rs1_orig.as_signed() >= rs2_orig.as_signed(),
                    BLTU => rs1_orig < rs2_orig,
                    BGEU => rs1_orig >= rs2_orig,
                } {
                    (self.pc as isize + imm) as usize
                }
                else {
                    self.pc + 1
                };

                self.actions.push(Action::Jump(self.pc, target));
                self.pc = target;
            }

            _ => {
                panic!("Unsupported instruction {:?}", self.program[self.pc]);
            }
        }
    }

    pub fn step_back(&mut self) {

    }

    pub fn registers(&self) -> &RegisterFile {
        &self.register_file
    }
}
