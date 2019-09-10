pub const MAX_STACK_SIZE: usize = 16;
pub const MAX_MEMORY_SIZE: usize = 4096;

pub mod opcodes;
use self::opcodes::*;

#[derive(Copy, Clone)]
pub struct ProcState {
    pub mem: [u8; 4096],
    pub vreg: [u8; 16],
    pub ireg: u16,
    pub pc: u16,
    pub sp: usize,
    pub stack: [u16; MAX_STACK_SIZE],
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
            pc: 0x200,
            sp: 0x0,
            stack: [0x0; 16],
        }
    }

    pub fn pop(&mut self) -> u16 {
        let val = self.stack[self.sp as usize];
        self.sp = self.sp - 1;

        self.check_sp_bounds();

        return val;
    }

    pub fn push(&mut self, val: u16) {
        self.sp = self.sp + 1;
        self.stack[self.sp] = val;

        self.check_sp_bounds();
    }

    fn check_sp_bounds(&self) {
        if self.sp > MAX_STACK_SIZE {
            panic!("SP={} is out of bounds!", self.sp)
        }
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
                    self.pc = self.pc + 2;
                }
            },
            Opcode::SNEVxByte{x, byte} => {
                if self.vreg[x as usize] != byte {
                    self.pc = self.pc + 2;
                }
            },
            Opcode::SEVxVy{x, y} => {
                if self.vreg[x as usize] == self.vreg[y as usize] {
                    self.pc + self.pc + 2;
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
}

#[cfg(test)]
mod test {
    use super::{ProcState, MAX_MEMORY_SIZE};

    #[test]
    pub fn pop_correctly_performs_bounds_check() {
        let state = ProcState::new([0x0; MAX_MEMORY_SIZE]);
    }
}