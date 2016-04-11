use std::cell::RefCell;
use std::rc::Rc;

use isa;
use memory;
use types;

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
}

struct Action {
    // pc: before after
    // read registers
    // written register(s)
    // read memory
    // written memory
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
        }
    }

    pub fn step(&mut self) {
        let instruction = &self.program[self.pc];
        match instruction {
            &isa::Instruction::I {
                opcode: opcode,
                rd: rd,
                rs1: rs1,
                imm: imm,
            } => {
                match opcode {
                    isa::IOpcode::ADDI => {
                        let rs1 = self.register_file.read_word(rs1);
                        self.register_file.write_word(rd, rs1 + imm);
                    },

                    _ => {
                        panic!("Unsupported instruction {:?}", self.program[self.pc]);
                    }
                }
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
