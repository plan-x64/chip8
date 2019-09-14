use self::opcodes::*;
use std::fmt::{Display, Formatter};
use std::fmt;

pub const MAX_STACK_SIZE: usize = 16;
pub const MAX_MEMORY_SIZE: usize = 4096;
pub const STARTING_PROGRAM_COUNTER: u16 = 0x200;

pub mod opcodes;

#[derive(Copy, Clone)]
pub struct ProcState {
    pub mem: [u8; 4096],
    pub vreg: [u8; 16],
    pub ireg: u16,
    pub pc: u16,
    pub sp: usize,
    pub stack: [u16; MAX_STACK_SIZE],
}

impl Display for ProcState {

    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "PC={} | SP={} | I={}", self.pc, self.sp, self.ireg)
    }
}

impl ProcState {
    pub fn reset(self) -> ProcState {
        ProcState::new([0x0; MAX_MEMORY_SIZE])
    }

    pub fn new(mem: [u8; MAX_MEMORY_SIZE]) -> ProcState {
        ProcState {
            mem,
            vreg: [0x0; 16],
            ireg: 0x0,
            pc: STARTING_PROGRAM_COUNTER,
            sp: 0x0,
            stack: [0x0; 16],
        }
    }

    pub fn pop(&mut self) -> u16 {
        let val = self.stack[self.sp as usize];
        let (wrapped_sp, overflowed) = self.sp.overflowing_sub(1);

        self.sp = wrapped_sp;

        if overflowed {
            panic!("Under-flowed stack: {}", &self);
        }

        return val;
    }

    pub fn push(&mut self, val: u16) {
        self.sp = self.sp + 1;

        if self.sp > MAX_STACK_SIZE {
            panic!("Over-flowed stack: {}", &self)
        }

        self.stack[self.sp] = val;
    }

    pub fn fetch_and_decode_opcode(&mut self) -> Opcode {
        let high_byte: u8 = self.mem[self.pc as usize];
        let low_byte: u8 = self.mem[(self.pc+1) as usize];
        self.pc = self.pc + 2;
        let opcode = (high_byte as u16) << 8 | (low_byte as u16);

        return get_opcode(opcode);
    }

    pub fn execute_opcode(&mut self, op: Opcode) {
        match op {
            Opcode::CLS => panic!("CLS not implemented!"),
            Opcode::RET => { self.pc = self.pop(); },
            Opcode::JP{addr} => { self.pc = addr; },
            Opcode::CALL{addr} => { let cur_pc = self.pc; self.push(cur_pc); self.pc = addr; },
            Opcode::SEVxByte{x, byte} => {
                if self.vreg[x as usize] == byte {
                    self.skip_next_instruction();
                }
            },
            Opcode::SNEVxByte{x, byte} => {
                if self.vreg[x as usize] != byte {
                    self.skip_next_instruction();
                }
            },
            Opcode::SEVxVy{x, y} => {
                if self.vreg[x as usize] == self.vreg[y as usize] {
                    self.skip_next_instruction();
                }
            },
            Opcode::LDVxByte{x, byte} => { self.vreg[x as usize] = byte; },
            Opcode::ADDVxByte{x, byte} => { self.vreg[x as usize] = self.vreg[x as usize] + byte; },
            Opcode::LDVxVy{x, y} => { self.vreg[x as usize] = self.vreg[y as usize]; },
            Opcode::ORVxVy{x, y} => { self.vreg[x as usize] = self.vreg[x as usize] | self.vreg[y as usize]; },
            Opcode::ADDVxVy{x, y} => {
                let (val, overflowed) = self.vreg[x as usize].overflowing_add(self.vreg[y as usize]);
                self.vreg[x as usize] = val;
                self.vreg[0xF] = if overflowed { 1 } else { 0 };
            },
            Opcode::XORVxVy{x, y} => { self.vreg[x as usize] = self.vreg[x as usize] ^ self.vreg[y as usize]; }
            Opcode::ANDVxVy{x, y} => { self.vreg[x as usize] = self.vreg[x as usize] & self.vreg[y as usize]; },
            Opcode::SUBVxVy{x, y} => panic!("not implemented"),
            Opcode::SHRVxVy{x, y} => panic!("not implemented"),
            Opcode::SUBNVxVy{x, y} => panic!("not implemented"),
            Opcode::SHLVxVy{x, y} => panic!("not implemented"),
            Opcode::SNEVxVy{x, y} => panic!("not implemented"),
            Opcode::LDIAddr{addr} => panic!("not implemented"),
            Opcode::JPV0Addr{addr} => panic!("not implemented"),
            Opcode::RNDVxByte{x, byte} => panic!("not implemented"),
            Opcode::DRW{x, y, nibble} => panic!("not implemented"),
            Opcode::SKPVx{x} => panic!("not implemented"),
            Opcode::SKNPVx{x} => panic!("not implemented"),
            Opcode::LDVxDT{x} => panic!("not implemented"),
            Opcode::LDVxK{x} => panic!("not implemented"),
            Opcode::LDDTVx{x} => panic!("not implemented"),
            Opcode::LDSTVx{x} => panic!("not implemented"),
            Opcode::ADDIVx{x} => panic!("not implemented"),
            Opcode::LDFVx{x} => panic!("not implemented"),
            Opcode::LDBVx{x} => panic!("not implemented"),
            Opcode::LDVxI{x} => panic!("not implemented"),
            Opcode::LDIVx{x} => panic!("not implemented"),
            Opcode::UNKNOWN{opcode} => panic!("not implemented"),
        }
    }

    fn skip_next_instruction(&mut self) {
        self.sp = self.sp + 2;
    }
}

#[cfg(test)]
mod test_cpu_basics {
    use super::{MAX_MEMORY_SIZE, MAX_STACK_SIZE, ProcState};

    #[test]
    #[should_panic]
    pub fn push_panics_when_upper_bound_exceeded() {
        let mut state = ProcState::new([0x0; MAX_MEMORY_SIZE]);

        for i in 0..(MAX_STACK_SIZE as u16) {
            state.push(i);
        }
    }

    #[test]
    pub fn push_does_not_panic_when_upper_bound_not_exceeded() {
        let mut state = ProcState::new([0x0; MAX_MEMORY_SIZE]);

        for i in 0..((MAX_STACK_SIZE-1) as u16) {
            state.push(i);
        }
    }

    #[test]
    #[should_panic]
    pub fn pop_panics_when_lower_bound_exceeded() {
        let mut state = ProcState::new([0x0; MAX_MEMORY_SIZE]);
        state.pop();
    }

    #[test]
    pub fn pop_does_not_panic_when_lower_bound_not_exceeded() {
        let mut state = ProcState::new([0x0; MAX_MEMORY_SIZE]);
        state.push(0);
        state.pop();
    }

    #[test]
    pub fn opcode_is_correctly_fetched() {
        let mut mem = [0x0; MAX_MEMORY_SIZE];
    }
}