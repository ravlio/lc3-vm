use crate::error::Error;
#[allow(dead_code)]
pub enum Endianness {
    LittleEndian = 0,
    BigEndian = 1,
}

pub struct Memory<T> {
    max_size: usize,
    mem: Vec<T>,
    endianness: Endianness,
}

pub trait Word: Copy + Default {
    fn to_bee(&self) -> Self;
    fn from_be(v: Self) -> Self;
}

impl Word for u8 {
    fn to_bee(&self) -> u8 {
        *self
    }
    fn from_be(v: u8) -> u8 {
        v
    }
}

impl Word for u16 {
    fn to_bee(&self) -> u16 {
        self.to_be()
    }
    fn from_be(v: u16) -> u16 {
        u16::from_be(v)
    }
}

impl Word for u32 {
    fn to_bee(&self) -> u32 {
        self.to_be()
    }
    fn from_be(v: u32) -> u32 {
        u32::from_be(v)
    }
}

impl Word for u64 {
    fn to_bee(&self) -> u64 {
        self.to_be()
    }
    fn from_be(v: u64) -> u64 {
        u64::from_be(v)
    }
}

impl<T: Word> Memory<T> {
    pub fn new(size: usize, max_size: usize, endianness: Endianness) -> Self {
        Memory {
            max_size,
            mem: vec![T::default(); size],
            endianness,
        }
    }

    fn check_bounds(&self, addr: usize) -> bool {
        addr > 0 && addr < self.max_size
    }

    pub fn read(&self, addr: usize) -> Result<T, Error> {
        if !self.check_bounds(addr) {
            return Err(Error::InvalidMemoryAddress);
        }

        match self.endianness {
            Endianness::BigEndian => {
                Ok(self.mem[addr])
            }
            Endianness::LittleEndian => {
                let a = self.mem[addr];
                Ok(a.to_bee())
            }
        }
    }

    pub fn write(&mut self, addr: usize, val: T) -> Result<(), Error> {
        if !self.check_bounds(addr) {
            return Err(Error::InvalidMemoryAddress);
        }
        if addr >= self.mem.len() {
            self.mem.resize(addr, T::default());
        }

        match self.endianness {
            Endianness::BigEndian => {
                self.mem[addr] = val;
            }
            Endianness::LittleEndian => {
                self.mem[addr] = Word::from_be(val);
            }
        }

        Ok(())
    }
}