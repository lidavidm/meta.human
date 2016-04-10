use std::cell::RefCell;
use std::rc::Rc;

use isa;
use memory;
use types;

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

struct Interpreter {
    register_file: RegisterFile,
    memory: Rc<RefCell<memory::Memory>>,
    cache: memory::Cache,
}

struct Action {
    // pc: before after
    // read registers
    // written register(s)
    // read memory
    // written memory
}

impl Interpreter {
    fn step(&mut self) {

    }

    fn step_back(&mut self) {

    }
}
