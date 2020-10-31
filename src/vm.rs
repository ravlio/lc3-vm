use num_traits::FromPrimitive;
use num_derive::FromPrimitive;
use std::fs::File;
use std::io::{Write, BufReader};
use byteorder::{LittleEndian, WriteBytesExt, ReadBytesExt, BigEndian};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use signal_hook::{iterator::Signals, SIGINT};
use std::thread;
use crate::memory::{Memory, Endianness};
use crate::error::Error;

const PC_START: u16 = 0x3000;

#[derive(FromPrimitive)]
enum Reg {
    R0 = 0,
    R1 = 1,
    R2 = 2,
    R3 = 3,
    R4 = 4,
    R5 = 5,
    R6 = 6,
    R7 = 7,
    PC = 8,
    Cond = 9,
    Count = 10,
}

#[derive(Debug)]
#[derive(FromPrimitive)]
enum Opcode {
    BR = 0,
    // add
    ADD = 1,
    // load
    LD = 2,
    // store
    ST = 3,
    // jump register
    JSR = 4,
    // bitwise and
    AND = 5,
    // load register
    LDR = 6,
    // store register
    STR = 7,
    // unused
    RTI = 8,
    // bitwise not
    NOT = 9,
    // load indirect
    LDI = 10,
    // store indirect
    STI = 11,
    // jump
    JMP = 12,
    // reserved
    RES = 13,
    // load effective address
    LEA = 14,
    // execute trap
    TRAP = 15,
}

#[derive(FromPrimitive)]
enum MMAP {
    // keyboard status
    KBSR = 0xff00,
    // keyboard data
    KBDR = 0xfe02,
}

// #[derive(Primitive)]
enum Flag {
    Pos = 1 << 0,
    Zro = 1 << 1,
    Neg = 1 << 2,
}

#[derive(FromPrimitive)]
enum Trap {
    // get character from keyboard, not echoed onto the terminal
    GETC = 0x20,
    // output a character
    OUT = 0x21,
    // output a word string
    PUTS = 0x22,
    // get character from keyboard, echoed onto the terminal
    IN = 0x23,
    // output a byte string
    PUTSP = 0x24,
    // halt the program
    HALT = 0x25,
}

pub struct VM {
    memory: Memory<u16>,
    reg: [u16; 12],
    is_running: Arc<AtomicBool>,
}

pub fn new() -> VM {
    VM {
        memory: Memory::new(std::u16::MAX as usize, std::u16::MAX as usize, Endianness::LittleEndian),
        reg: [0; 12],
        is_running: Arc::new(AtomicBool::new(false)),
    }
}

impl VM {
    pub fn run(&mut self) -> Result<(), Error> {
        self.is_running.store(true, Ordering::SeqCst);
        self.set_reg(Reg::PC as usize, PC_START);

        let signals = Signals::new(&[SIGINT])?;
        let is_running = self.is_running.clone();
        thread::spawn(move || {
            for sig in signals.forever() {
                println!("Received signal {:?}", sig);
                is_running.store(false, Ordering::SeqCst);
            }
        });

        while self.is_running.load(Ordering::SeqCst) {
            let instr = self.mem_read(self.get_reg(Reg::PC as usize))?;
            self.inc_reg(Reg::PC as usize, 1);
            let op = instr >> 12;

            match Opcode::from_u16(op) {
                Some(Opcode::ADD) => {
                    let r0 = ((instr >> 9) & 0x7) as usize;
                    let r1 = ((instr >> 6) & 0x7) as usize;
                    let imm_flag = (instr >> 5) & 0x1;

                    if imm_flag == 1 {
                        let imm5 = sign_extend(instr & 0x1F, 5);
                        self.set_reg(r0, self.get_reg(r1).wrapping_add(imm5));
                    } else {
                        let r2 = (instr & 0x7) as usize;
                        self.set_reg(r0, self.get_reg(r1).wrapping_add(self.get_reg(r2)));
                    }
                    self.update_flags(r0);
                }
                Some(Opcode::LDI) => {
                    let r0 = ((instr >> 9) & 0b111) as usize;
                    let pc_offset = sign_extend(instr & 0x1FF, 9);
                    let addr = self.mem_read(self.reg[Reg::PC as usize] + pc_offset)?;
                    let mem = self.mem_read(addr)?;
                    self.set_reg(r0, mem);
                    self.update_flags(r0);
                }
                Some(Opcode::AND) => {
                    let r0 = ((instr >> 9) & 0x7) as usize;
                    let r1 = ((instr >> 6) & 0x7) as usize;
                    let imm_flag = (instr >> 5) & 0x1;

                    if imm_flag == 1 {
                        let imm5 = sign_extend(instr & 0x1F, 5);
                        self.set_reg(r0, self.get_reg(r1) & imm5);
                    } else {
                        let r2 = (instr >> 0x7) as usize;
                        self.set_reg(r0, self.get_reg(r1) & self.get_reg(r2));
                    }
                    self.update_flags(r0);
                }
                Some(Opcode::BR) => {
                    let pc_offset = sign_extend(instr & 0x1FF, 9);
                    let cond_flag = (instr >> 9) & 0x7;
                    if cond_flag & self.reg[Reg::Cond as usize] > 0 {
                        self.inc_reg(Reg::PC as usize, pc_offset);
                    }
                }
                Some(Opcode::JMP) => {
                    let r0 = ((instr >> 6) & 0x7) as usize;
                    self.set_reg(Reg::PC as usize, self.get_reg(r0));
                }
                Some(Opcode::JSR) => {
                    let long_flag = (instr >> 11) & 1;
                    self.set_reg(Reg::R7 as usize, self.get_reg(Reg::PC as usize));
                    if long_flag > 0 {
                        let long_pc_offset = sign_extend(instr & 0x7FF, 11);
                        self.inc_reg(Reg::PC as usize, long_pc_offset);
                    } else {
                        let r0 = ((instr >> 6) & 0x7) as usize;
                        self.set_reg(Reg::PC as usize, self.get_reg(r0));
                    }
                }
                Some(Opcode::LD) => {
                    let r0 = ((instr >> 9) & 0x7) as usize;
                    let pc_offset = sign_extend(instr & 0x1FF, 9);
                    let mem = self.mem_read(self.get_reg(Reg::PC as usize) + pc_offset)?;
                    self.set_reg(r0, mem);
                    self.update_flags(r0);
                }
                Some(Opcode::LDR) => {
                    let r0 = ((instr >> 9) & 0x7) as usize;
                    let r1 = ((instr >> 6) & 0x7) as usize;
                    let offset = sign_extend(instr & 0x3F, 6);
                    let mem = self.mem_read(self.get_reg(r1))?;
                    self.set_reg(r0, mem + offset);
                    self.update_flags(r0);
                }
                Some(Opcode::STR) => {
                    let r0 = ((instr >> 9) & 0x7) as usize;
                    let r1 = ((instr >> 6) & 0x7) as usize;
                    let offset = sign_extend(instr & 0x3F, 6);
                    self.mem_write(self.get_reg(r1) + offset, self.get_reg(r0))?;
                }
                Some(Opcode::NOT) => {
                    let r0 = ((instr >> 9) & 0x7) as usize;
                    let r1 = ((instr >> 6) & 0x7) as usize;
                    self.set_reg(r0, !self.get_reg(r1));
                    self.update_flags(r0);
                }
                Some(Opcode::STI) => {
                    let r0 = ((instr >> 9) & 0x7) as usize;
                    let offset = sign_extend(instr & 0x1FF, 9);
                    let addr = self.mem_read(self.get_reg(Reg::PC as usize) + offset)?;
                    self.mem_write(addr, self.get_reg(r0))?;
                }
                Some(Opcode::LEA) => {
                    let r0 = ((instr >> 9) & 0x7) as usize;
                    let offset = sign_extend(instr & 0x1FF, 9);
                    self.set_reg(r0, self.get_reg(Reg::PC as usize) + offset);
                    self.update_flags(r0);
                }
                Some(Opcode::ST) => {
                    let r0 = ((instr >> 9) & 0x7) as usize;
                    let offset = sign_extend(instr & 0x1FF, 9);
                    self.mem_write(self.get_reg(Reg::PC as usize) + offset, self.get_reg(r0))?;
                }
                Some(Opcode::TRAP) => {
                    match Trap::from_u16(instr & 0xFF) {
                        Some(Trap::GETC) => {
                            self.set_reg(Reg::R0 as usize, std::io::stdin().read_u8()? as u16);
                        }
                        Some(Trap::OUT) => {
                            std::io::stdout().write_u8(self.get_reg(Reg::R0 as usize) as u8)?;
                            std::io::stdout().flush()?;
                        }
                        Some(Trap::PUTS) => {
                            for i in 0.. {
                                let char = self.mem_read(self.get_reg(Reg::R0 as usize) + i)?;
                                if char == 0 {
                                    break;
                                }
                                std::io::stdout().write_u8(char as u8)?;
                                std::io::stdout().flush()?;
                            }
                        }
                        Some(Trap::IN) => {
                            print!("Enter a character: ");
                            self.set_reg(Reg::R0 as usize, std::io::stdin().read_u8()? as u16);
                            std::io::stdout().write_u8(self.get_reg(Reg::R0 as usize) as u8)?;
                            std::io::stdout().flush()?;
                        }
                        Some(Trap::PUTSP) => {
                            for i in 0.. {
                                let b = self.mem_read(self.get_reg(Reg::R0 as usize) + i)?;
                                if b == 0 {
                                    break;
                                }
                                std::io::stdout().write_u16::<LittleEndian>(b)?;
                            }

                            std::io::stdout().flush()?;
                        }
                        Some(Trap::HALT) => {
                            std::io::stdout().write("HALT".as_bytes())?;
                            std::io::stdout().flush()?;
                            self.is_running.store(false, Ordering::SeqCst);
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        };

        Ok(())
    }

    fn get_reg(&self, reg: usize) -> u16 {
        self.reg[reg]
    }
    fn set_reg(&mut self, reg: usize, val: u16) {
        self.reg[reg] = val;
    }
    fn inc_reg(&mut self, reg: usize, delta: u16) {
        self.reg[reg] = self.reg[reg].wrapping_add(delta);
    }

    pub fn load_image(&mut self, file: &str) -> Result<(), Error> {
        let mut f = BufReader::new(File::open(file)?);
        let mut origin = f.read_u16::<BigEndian>()? as usize;
        while let Ok(m) = f.read_u16::<BigEndian>() {
            self.memory.write(origin, m)?;
            origin += 1;
        };

        std::io::stdout().flush()?;
        Ok(())
    }

    fn update_flags(&mut self, r: usize) {
        if self.reg[r] == 0 {
            self.reg[Reg::Cond as usize] = Flag::Zro as u16;
        } else if self.reg[r] >> 15 > 0 {
            self.reg[Reg::Cond as usize] = Flag::Neg as u16;
        } else {
            self.reg[Reg::Cond as usize] = Flag::Pos as u16;
        }
    }
    fn mem_write(&mut self, addr: u16, val: u16) -> Result<(), Error> {
        self.memory.write(addr as usize, val)
    }
    fn mem_read(&mut self, addr: u16) -> Result<u16, Error> {
        if addr == MMAP::KBSR as u16 {
            if let Ok(ch) = std::io::stdin().read_u16::<LittleEndian>() {
                self.memory.write(MMAP::KBSR as usize, 1 << 15)?;
                self.memory.write(MMAP::KBDR as usize, ch)?;
            } else {
                self.memory.write(MMAP::KBSR as usize, 0)?;
            }
        }

        self.memory.read(addr as usize)
    }
}

fn sign_extend(x: u16, bit_count: i16) -> u16 {
    if (x >> (bit_count - 1)) & 1 > 0 {
        x | (0xffff << bit_count)
    } else {
        x
    }
}
