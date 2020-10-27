use crate::memory::Error::InvalidMemoryAddress;
use core::mem;

#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidMemoryAddress,
}

pub enum Endianness {
    LittleEndian = 0,
    BigEndian = 1,
}

pub trait Byte {
    fn read_byte(&self, addr: usize) -> u8;
    fn write_byte(&mut self, addr: usize, val: u8);
}

pub trait Word<T>: Byte {
    fn read_word(&self, addr: usize) -> Result<T, Error>;
    fn write_word(&mut self, addr: usize, val: T) -> Result<(), Error>;
}

struct Memory {
    max_size: usize,
    mem: Vec<u8>,
    endianness: Endianness,
}

impl Mem {
    fn new(size: usize, max_size: usize, endianness: Endianness) -> Self {
        Mem {
            max_size,
            mem: vec![0; size],
            endianness,
        }
    }

    fn check_bounds(&self, addr: usize) -> bool {
        addr > 0 && addr < self.mem.len()
    }
}

impl Byte for Mem {
    fn read_byte(&mut self, addr: usize) -> Result<u8, Error> {
        if !self.check_bounds(addr) {
            Err(Error::InvalidMemoryAddress)
        }
        Ok(self.mem[addr])
    }

    fn write_byte(&mut self, addr: usize, val: u8) -> Result<(), Error> {
        if !self.check_bounds(addr) {
            Err(Error::InvalidMemoryAddress)
        }
        if addr >= self.mem.len() {
            self.mem.resize(addr, 0);
        }
        self.mem[addr] = val;
        Ok(())
    }
}

impl Word<u8> for Mem {
    fn read_word(&self, addr: usize) -> Result<u8, Error> {
        self.read_b
    }

    fn write_word(&mut self, addr: usize, val: u8) -> Result<(), Error> {
        unimplemented!()
    }
}

/*fn read_word<T>(&self, addr: usize) -> Result<u8, Error> {}

fn read_word<T>(&self, addr: usize) -> Result<T, Error> {
    if !self.check_bounds(addr) {
        Err(Error::InvalidMemoryAddress)
    }

    let word_size_bits = mem::size_of::<T>();
    let word_size_bytes = size / 8;
    if addr * word_size_bytes + word_size_bytes >= self.mem.len() {
        Err(Error::InvalidMemoryAddress)
    }

    match self.endianness {
        Endianness::LittleEndian => {
            match word_size_bits {
                8 => {
                    Ok(1)
                }
                16 => {}
                32 => {}
                64 => {}
                _ => {
                    panic!("unknown word size {}", word_size_bits);
                }
            }
        }
        Endianness::BigEndian => {}
    }
    Ok(())
}*/

/*fn write_word(&mut self, addr: usize, val: _) {
    unimplemented!()
}*/
