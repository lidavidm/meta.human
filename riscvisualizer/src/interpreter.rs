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
    actions: Vec<Action>,
}

pub struct Action {
    // pc: before after
    pub pc: (usize, usize),
    // read registers - register and value read
    pub read_registers: Vec<(isa::Register, types::Word)>,
    // written register - register, old value, value written
    pub written_register: Option<(isa::Register, types::Word, types::Word)>,
    // read memory
    // written memory
}

fn register_action1<F>(pc: usize, register_file: &mut RegisterFile, reg: isa::Register, callback: F) -> Action
    where F: Fn(types::Word) -> (isa::Register, types::Word) {
    let input = register_file.read_word(reg);
    let (target, output) = callback(input);
    let original = register_file.read_word(target);
    register_file.write_word(target, output);
    Action {
        pc: (pc, pc + 1),
        read_registers: vec![(reg, input)],
        written_register: Some((target, original, output)),
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

    pub fn step(&mut self) -> &Action {
        let instruction = &self.program[self.pc];
        match instruction {
            &isa::Instruction::I {
                opcode,
                rd,
                rs1,
                imm,
            } => {
                match opcode {
                    isa::IOpcode::ADDI => {
                        let action = register_action1(self.pc, &mut self.register_file, rs1, |rs1| {
                            (rd, rs1 + imm)
                        });
                        self.pc = action.pc.1;
                        self.actions.push(action);
                        &self.actions.last().unwrap()
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
