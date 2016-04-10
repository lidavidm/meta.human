use std::cell::RefCell;
use std::rc::Rc;

use types;
use types::IsaType;

#[derive(Clone, Debug, PartialEq)]
pub enum MemoryError {
    InvalidAddress,
}

pub struct MemoryAccess<T>(T, usize);
pub type Result<T> = ::std::result::Result<MemoryAccess<T>, MemoryError>;

trait MemoryInterface {
    // fn prefetch(&mut self, address: types::Address);
    // fn invalidate(&mut self, address: types::Address);

    fn is_address_accessible(&self, address: types::Address) -> bool;

    fn read_word(&mut self, address: types::Address) -> Result<types::Word>;
    fn write_word(&mut self, address: types::Address, value: types::Word) -> Result<()>;

    // TODO: check address more thoroughly
    // TODO: get rid of panics

    fn read_halfword(&mut self, address: types::Address) -> Result<types::HalfWord> {
        let result = self.read_word(address);
        let offset = (address & 0b10).0;

        match result {
            Ok(MemoryAccess(word, cycles)) => Ok(MemoryAccess(match offset {
                0 => (word & 0xFFFF).as_half_word(),
                2 => ((word & 0xFFFF0000) >> 16).as_half_word(),
                _ => panic!("Invalid halfword offset: address {:x}", address),
            }, cycles)),
            Err(e) => Err(e),
        }
    }

    fn write_halfword(&mut self, address: types::Address, value: types::HalfWord) -> Result<()> {
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
                self.write_word(address, value).map(|MemoryAccess((), write_cycles)| {
                    MemoryAccess((), cycles + write_cycles)
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

    fn write_byte(&mut self, address: types::Address, value: types::Byte) -> Result<()> {
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
                self.write_word(address, value).map(|MemoryAccess((), write_cycles)| {
                    MemoryAccess((), cycles + write_cycles)
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
    sets: Vec<Vec<u32>>,
}
