use std::cell::RefCell;
use std::rc::Rc;

use types;
use types::IsaType;

#[derive(Clone, Debug, PartialEq)]
pub enum MemoryError {
    InvalidAddress,
}

pub struct MemoryAccess<T>(pub T, pub usize);
pub type Result<T> = ::std::result::Result<MemoryAccess<T>, MemoryError>;

impl<T> MemoryAccess<T> where T: IsaType {
    pub fn as_word(self) -> MemoryAccess<types::Word> {
        MemoryAccess(self.0.as_word(), self.1)
    }
}

pub trait MemoryInterface {
    // fn prefetch(&mut self, address: types::Address);
    // fn invalidate(&mut self, address: types::Address);

    fn is_address_accessible(&self, address: types::Address) -> bool;

    fn read_word(&mut self, address: types::Address) -> Result<types::Word>;
    fn write_word(&mut self, address: types::Address, value: types::Word) -> Result<types::Word>;

    // TODO: check address more thoroughly
    // TODO: get rid of panics

    fn read_halfword(&mut self, address: types::Address) -> Result<types::HalfWord> {
        let result = self.read_word(address);
        let offset = (address & 0b10).0;

        match result {
            Ok(MemoryAccess(word, cycles)) => Ok(MemoryAccess(match offset {
                0 => (word & 0xFFFF).as_halfword(),
                2 => ((word & 0xFFFF0000) >> 16).as_halfword(),
                _ => panic!("Invalid halfword offset: address {:x}", address),
            }, cycles)),
            Err(e) => Err(e),
        }
    }

    fn write_halfword(&mut self, address: types::Address, value: types::HalfWord) -> Result<types::HalfWord> {
        let result = self.read_word(address);
        let offset = (address & 0b10).0;
        let value = value.as_word();

        match result {
            Ok(MemoryAccess(word, cycles)) => {
                let value = match offset {
                    0 => (word & 0xFFFF0000) | value,
                    2 => (word & 0x0000FFFF) | (value << 16),
                    _ => panic!("Invalid halfword offset: address {:x}", address),
                };
                self.write_word(address, value).map(|MemoryAccess(v, write_cycles)| {
                    MemoryAccess(match offset {
                        0 => word & 0xFFFF,
                        2 => (word & 0xFFFF0000) >> 16,
                        _ => panic!("Invalid halfword offset: address {:x}", address),
                    }.as_halfword(), cycles + write_cycles)
                })
            },
            Err(e) => Err(e),
        }
    }

    fn read_byte(&mut self, address: types::Address) -> Result<types::Byte> {
        let result = self.read_word(address);
        let offset = (address % 4).0;

        match result {
            Ok(MemoryAccess(word, cycles)) => Ok(MemoryAccess(match offset {
                0 => (word & 0xFF).as_byte(),
                1 => ((word & 0xFF00) >> 8).as_byte(),
                2 => ((word & 0xFF0000) >> 16).as_byte(),
                3 => ((word & 0xFF000000) >> 24).as_byte(),
                _ => panic!("Invalid byte offset: {:x}", address),
            }, cycles)),
            Err(e) => Err(e),
        }
    }

    fn write_byte(&mut self, address: types::Address, value: types::Byte) -> Result<types::Byte> {
        let result = self.read_word(address);
        let offset = (address % 4).0;
        let value = value.as_word();

        match result {
            Ok(MemoryAccess(word, cycles)) => {
                let value = match offset {
                    0 => (word & !(0xFF)) | value,
                    1 => (word & !(0xFF00)) | (value << 8),
                    2 => (word & !(0xFF0000)) | (value << 16),
                    3 => (word & !(0xFF000000)) | (value << 24),
                    _ => panic!("Invalid byte offset: {:x}", address),
                };
                self.write_word(address, value).map(|MemoryAccess(v, write_cycles)| {
                    MemoryAccess(v.as_bytes()[offset as usize], cycles + write_cycles)
                })
            },
            Err(e) => Err(e),
        }
    }
}

pub struct Memory {
    memory: Vec<u32>,
}

pub struct Cache {
    main_memory: Rc<RefCell<Memory>>,
    num_sets: usize,
    block_words: usize,
    sets: Vec<Vec<u32>>,
}

impl Memory {
    pub fn new(words: usize) -> Memory {
        Memory {
            memory: Vec::with_capacity(words),
        }
    }
}

impl MemoryInterface for Memory {
    fn is_address_accessible(&self, address: types::Address) -> bool {
        let word_address = address.0 as usize / 4;

        word_address >= 0 && word_address < self.memory.len()
    }

    fn read_word(&mut self, address: types::Address) -> Result<types::Word> {
        if self.is_address_accessible(address) {
            let address = address.0 as usize / 4;
            Ok(MemoryAccess(types::Word(self.memory[address]), 0))
        }
        else {
            Err(MemoryError::InvalidAddress)
        }
    }

    fn write_word(&mut self, address: types::Address, value: types::Word) -> Result<types::Word> {
        if self.is_address_accessible(address) {
            let address = address.0 as usize / 4;
            let original = self.memory[address];
            self.memory[address] = value.0;
            Ok(MemoryAccess(types::Word(original), 0))
        }
        else {
            Err(MemoryError::InvalidAddress)
        }
    }
}

impl Cache {
    pub fn new(memory: Rc<RefCell<Memory>>, num_sets: usize, block_words: usize) -> Cache {
        let sets = vec![vec![0; block_words]; num_sets];
        Cache {
            main_memory: memory,
            num_sets: num_sets,
            block_words: block_words,
            sets: Vec::new(),
        }
    }
}
