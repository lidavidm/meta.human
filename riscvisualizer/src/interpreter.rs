use std;
use std::cell::RefCell;
use std::rc::Rc;
use std::slice;


use isa;
use memory::{self, MemoryInterface};
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
    Jump(usize, usize),

    // stall

    // read cache

    // read memory

    // write cache

    // write memory

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
        match instruction {
            &isa::Instruction::I {
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
                        _ => {
                            panic!("Unsupported instruction");
                        }
                    };

                match action {
                    Action::WriteRegister(target, _, value) => {
                        self.register_file.write_word(target, value);
                    }

                    _ => {},
                };

                self.actions.push(action);
                self.actions.push(Action::Jump(self.pc, self.pc + 1));
                self.pc += 1;
            },

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
