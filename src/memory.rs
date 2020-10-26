use crate::memory::Error::InvalidMemoryAddress;

#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidMemoryAddress,

}

pub trait Memory<T> {
    fn new(size: usize, max_size: usize) -> Self;
    fn read_byte(&self, addr: usize) -> u8;
    fn write_byte(&mut self, addr: usize, val: u8);
    fn read_word(&self, addr: usize) -> T;
    fn write_word(&mut self, addr: usize, val: T);
}

struct Mem {
    max_size: usize,
    mem: Vec<u8>,
}

impl Memory<T> for Mem {
    fn new(size: usize, max_size: usize) -> Self {
        Mem {
            max_size,
            mem: vec![0; size],
        }
    }

    fn read_byte(&mut self, addr: usize) -> Result<u8, Error> {
        if addr < 0 || addr > self.max_size {
            Err(Error::InvalidMemoryAddress)
        }
        if addr >= self.mem.len() {
            self.mem.resize(addr, 0);
            Ok(0)
        }
        Ok(self.mem[addr])
    }

    fn write_byte(&mut self, addr: usize, val: u8) {
        self.mem[addr] = val
    }

    fn read_word(&self, addr: usize) -> _ {
        unimplemented!()
    }

    fn write_word(&mut self, addr: usize, val: _) {
        unimplemented!()
    }
}