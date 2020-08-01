use std::cell::Cell;
use std::fmt::{Display, Formatter};
use std::fmt;
use std::rc::Rc;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::SCREEN_HEIGHT;

use self::opcodes::*;

pub mod opcodes;

pub const MAX_STACK_SIZE: usize = 16;
pub const MAX_MEMORY_SIZE: usize = 4096;
pub const STARTING_PROGRAM_COUNTER: u16 = 0x200;

#[derive(Clone)]
pub struct ProcState {
    pub mem: [u8; 4096],
    pub vreg: [u8; 16],
    pub ireg: u16,
    pub pc: u16,
    pub sp: usize,
    pub stack: [u16; MAX_STACK_SIZE],
    pub delay_t: u8,
    pub sound_t: u8,
    pub io_queue: Rc<Cell<Option<u8>>>,
    pub video_buffer: [u64; SCREEN_HEIGHT],
    pub clock: u64
}

impl Display for ProcState {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "PC={:#05x} | SP={:#03x} | I={:#05x}", self.pc, self.sp, self.ireg)
    }
}

impl ProcState {
    pub fn reset(self) -> Self {
        ProcState::new([0x0; MAX_MEMORY_SIZE], self.io_queue)
    }

    pub fn new(mem: [u8; MAX_MEMORY_SIZE], io_queue: Rc<Cell<Option<u8>>>) -> Self {
        ProcState {
            mem,
            vreg: [0x0; 16],
            ireg: 0x0,
            pc: STARTING_PROGRAM_COUNTER,
            sp: 0x0,
            stack: [0x0; 16],
            delay_t: 0,
            sound_t: 0,
            io_queue,
            video_buffer: [0x0; SCREEN_HEIGHT],
            clock: 0
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

    pub fn clock_tick(&mut self, _freq: u64) {
        self.clock += 1;
        self.delay_t = self.delay_t.checked_sub(1).unwrap_or(0);
        self.sound_t = self.sound_t.checked_sub(1).unwrap_or(0);
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
            Opcode::CLS => {

            },
            Opcode::RET => {
                self.pc = self.pop();
            },
            Opcode::JP{addr} => {
                self.pc = addr;
            },
            Opcode::CALL{addr} => {
                let cur_pc = self.pc; self.push(cur_pc); self.pc = addr;
            },
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
            Opcode::LDVxByte{x, byte} => {
                self.vreg[x as usize] = byte;
            },
            Opcode::ADDVxByte{x, byte} => {
                let (val, _) = self.vreg[x as usize].overflowing_add(byte);
                self.vreg[x as usize] = val;
            },
            Opcode::LDVxVy{x, y} => {
                self.vreg[x as usize] = self.vreg[y as usize];
            },
            Opcode::ORVxVy{x, y} => {
                self.vreg[x as usize] = self.vreg[x as usize] | self.vreg[y as usize];
            },
            Opcode::ADDVxVy{x, y} => {
                let (val, overflowed) = self.vreg[x as usize].overflowing_add(self.vreg[y as usize]);
                self.vreg[x as usize] = val;
                self.vreg[0xF] = if overflowed { 1 } else { 0 };
            },
            Opcode::XORVxVy{x, y} => {
                self.vreg[x as usize] = self.vreg[x as usize] ^ self.vreg[y as usize];
            }
            Opcode::ANDVxVy{x, y} => {
                self.vreg[x as usize] = self.vreg[x as usize] & self.vreg[y as usize];
            },
            Opcode::SUBVxVy{x, y} => {
                let (val, borrowed) = self.vreg[x as usize].overflowing_sub(self.vreg[y as usize]);
                self.vreg[x as usize] = val;
                self.vreg[0xF] = if borrowed { 0 } else { 1 };
            },
            Opcode::SHRVxVy{x, y: _} => {
                self.vreg[0xF] = self.vreg[x as usize] & 0x1;
                self.vreg[x as usize] = self.vreg[x as usize] >> 1;
            },
            Opcode::SUBNVxVy{x, y} => {
                let (val, borrowed) = self.vreg[y as usize].overflowing_sub(self.vreg[x as usize]);
                self.vreg[x as usize] = val;
                self.vreg[0xF] = if borrowed { 0 } else { 1 };
            },
            Opcode::SHLVxVy{x, y: _} => {
                self.vreg[0xF] = self.vreg[x as usize] >> 7 & 0x1;
                self.vreg[x as usize] = self.vreg[x as usize] << 1;
            },
            Opcode::SNEVxVy{x, y} => {
                if self.vreg[x as usize] != self.vreg[y as usize] {
                    self.skip_next_instruction();
                }
            },
            Opcode::LDIAddr{addr} => {
                self.ireg = addr;
            },
            Opcode::JPV0Addr{addr} => {
                self.pc = self.vreg[0x0] as u16 + addr;
            },
            Opcode::RNDVxByte{x, byte} => {
                self.vreg[x as usize] = self.rand() & byte;
            },
            Opcode::DRW{x, y, nibble} => {
                let mut sprite_mask: [u64; SCREEN_HEIGHT] = [0x0; SCREEN_HEIGHT];
                for i in 0 .. nibble {
                    let sprite_line = self.mem[(self.ireg + i as u16) as usize];
                    let xpos = self.vreg[x as usize];
                    let ypos = self.vreg[y as usize];
                    if (ypos as usize + i as usize) < SCREEN_HEIGHT {
                        sprite_mask[(ypos as usize) + (i as usize)] = ((sprite_line as u64) << 56) >> (xpos as u64);
                    }
                }

                self.vreg[0xF] = 0;
                let old_buffer = self.video_buffer;
                for i in 0 .. SCREEN_HEIGHT {
                    self.video_buffer[i] = sprite_mask[i] ^ self.video_buffer[i];
                    if (old_buffer[i] & !self.video_buffer[i]) != 0 {
                        self.vreg[0xF] = 1;
                    }
                }
            },
            Opcode::SKPVx{x} => {
                let curr_key = self.io_queue.get();
                match curr_key {
                    None => (),
                    Some(key) => if key == self.vreg[x as usize] { self.skip_next_instruction() }
                }
            },
            Opcode::SKNPVx{x} => {
                let curr_key = self.io_queue.get();
                match curr_key {
                    None => self.skip_next_instruction(),
                    Some(key) => if key != self.vreg[x as usize] { self.skip_next_instruction() }
                }
            },
            Opcode::LDVxDT{x} => {
                self.vreg[x as usize] = self.delay_t;
            },
            Opcode::LDVxK{x} => {
                let curr_key = self.io_queue.get();
                match curr_key {
                    None => self.pc = self.pc - 2, // Reset to give appearance of blocking
                    Some(key) => self.vreg[x as usize] = key
                }
            },
            Opcode::LDDTVx{x} => {
                self.delay_t = self.vreg[x as usize];
            },
            Opcode::LDSTVx{x} => {
                self.sound_t = self.vreg[x as usize];
            },
            Opcode::ADDIVx{x} => {
                self.ireg = self.ireg + (self.vreg[x as usize] as u16);
            },
            Opcode::LDFVx{x} => {
                self.ireg = (self.vreg[x as usize] as u16) * 5;
            },
            Opcode::LDBVx{x} => {
                let vx = self.vreg[x as usize];
                let hundreds = vx / 100;
                let tens = (vx - (hundreds * 100)) / 10;
                let ones = vx - (hundreds * 100) - (tens * 10);

                self.mem[self.ireg as usize] = hundreds;
                self.mem[(self.ireg as usize) + 1] = tens;
                self.mem[(self.ireg as usize) + 2] = ones;
            },
            Opcode::LDIVx{x} => {
                for k in 0 ..= x {
                    self.mem[(self.ireg as usize) + (k as usize)] = self.vreg[k as usize];
                }
            },
            Opcode::LDVxI{x} => {
                for k in 0 ..= x {
                    self.vreg[k as usize] = self.mem[(self.ireg as usize) + (k as usize)];
                }
            },
            Opcode::UNKNOWN{opcode: _} => panic!("unknown opcode {}", op),
        }
    }

    fn skip_next_instruction(&mut self) {
        self.pc = self.pc + 2;
    }

    fn rand(&self) -> u8 {
        // Generates a pseudo random number without needing an 3P create
        // Credit: https://users.rust-lang.org/t/random-number-without-using-the-external-crate/17260/9
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .subsec_nanos();

        return (nanos & 0xFF) as u8;
    }
}

#[cfg(test)]
mod test_cpu_basics {
    use std::cell::Cell;
    use std::rc::Rc;

    use crate::cpu::{MAX_MEMORY_SIZE, MAX_STACK_SIZE, ProcState};
    use crate::cpu::opcodes::Opcode;

    #[test]
    pub fn pc_double_inc_on_skip_next_instruction() {
        let mut state = ProcState::new([0x0; MAX_MEMORY_SIZE], Rc::new(Cell::new(Option::None)));

        for _ in 0..MAX_MEMORY_SIZE {
            let current_pc = state.pc;
            state.skip_next_instruction();
            assert_eq!(state.pc, current_pc + 2);
        }
    }

    #[test]
    #[should_panic]
    pub fn push_panics_when_upper_bound_exceeded() {
        let mut state = ProcState::new([0x0; MAX_MEMORY_SIZE], Rc::new(Cell::new(Option::None)));

        for i in 0..(MAX_STACK_SIZE as u16) {
            state.push(i);
        }
    }

    #[test]
    pub fn push_does_not_panic_when_upper_bound_not_exceeded() {
        let mut state = ProcState::new([0x0; MAX_MEMORY_SIZE], Rc::new(Cell::new(Option::None)));

        for i in 0..((MAX_STACK_SIZE-1) as u16) {
            state.push(i);
        }
    }

    #[test]
    #[should_panic]
    pub fn pop_panics_when_lower_bound_exceeded() {
        let mut state = ProcState::new([0x0; MAX_MEMORY_SIZE], Rc::new(Cell::new(Option::None)));
        state.pop();
    }

    #[test]
    pub fn pop_does_not_panic_when_lower_bound_not_exceeded() {
        let mut state = ProcState::new([0x0; MAX_MEMORY_SIZE], Rc::new(Cell::new(Option::None)));
        state.push(0);
        state.pop();
    }

    #[test]
    pub fn opcode_is_correctly_fetched() {
        let mut mem = [0x0; MAX_MEMORY_SIZE];
        mem[0x0] = 0xA1;
        mem[0x1] = 0x23;

        let mut state = ProcState::new(mem, Rc::new(Cell::new(Option::None)));
        state.pc = 0;

        let opcode = state.fetch_and_decode_opcode();

        assert_eq!(opcode, Opcode::LDIAddr { addr: 0x123 });
    }

    #[test]
    pub fn program_counter_is_incremented_after_fetch() {
        let mut state = ProcState::new([0x0; MAX_MEMORY_SIZE], Rc::new(Cell::new(Option::None)));

        let initial_pc = state.pc;
        state.fetch_and_decode_opcode();

        assert_eq!(state.pc, initial_pc + 2);
    }
}

#[cfg(test)]
mod test_cpu_execution {
    use std::cell::Cell;
    use std::rc::Rc;

    use crate::cpu::{MAX_MEMORY_SIZE, ProcState};
    use crate::cpu::opcodes::Opcode;

    #[test]
    pub fn ret_decrements_stack_pointer() {
        let mut state = ProcState::new([0x0; MAX_MEMORY_SIZE], Rc::new(Cell::new(Option::None)));

        for _ in 0..10 {
            state.push(0x200);
        }

        let initial_sp = state.sp;

        state.execute_opcode(Opcode::RET);

        assert_eq!(state.sp, initial_sp - 1);
    }

    #[test]
    pub fn ret_sets_program_counter_to_value_on_top_of_stack() {
        let mut state = ProcState::new([0x0; MAX_MEMORY_SIZE], Rc::new(Cell::new(Option::None)));
        let expected_pc: u16 = state.pc + 0x2FE;
        state.push(expected_pc);

        state.execute_opcode(Opcode::RET);

        assert_eq!(state.pc, expected_pc);
    }

    #[test]
    pub fn jmp_unconditionally_sets_program_counter() {
        let mut state = ProcState::new([0x0; MAX_MEMORY_SIZE], Rc::new(Cell::new(Option::None)));

        for i in 0x200..(MAX_MEMORY_SIZE as u16) {
            state.execute_opcode(Opcode::JP { addr: i });
            assert_eq!(state.pc, i);
        }
    }

    #[test]
    pub fn call_updates_program_counter_to_addr() {
        let mut state = ProcState::new([0x0; MAX_MEMORY_SIZE], Rc::new(Cell::new(Option::None)));
        let addr = 0x123;

        state.execute_opcode(Opcode::CALL { addr });
        assert_eq!(state.pc, addr);
    }

    #[test]
    pub fn call_stores_existing_program_counter_on_stack() {
        let mut state = ProcState::new([0x0; MAX_MEMORY_SIZE],Rc::new(Cell::new(Option::None)));
        let previous_pc = 0x456;
        state.pc = previous_pc;

        state.execute_opcode(Opcode::CALL { addr: 0x123 });

        assert_eq!(state.pop(), previous_pc);
    }
}