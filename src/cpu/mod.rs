const STACK_UPPER_BOUND: usize = 16;

mod opcodes;
use self::opcodes::*;

#[derive(Copy, Clone)]
pub struct ProcState {
    pub mem: [u8; 4096],
    pub vreg: [u8; 16],
    pub ireg: u16,
    pub pc: u16,
    pub sp: usize,
    pub stack: [u16; STACK_UPPER_BOUND],
}

impl ProcState {
    pub fn reset() -> ProcState {
        ProcState {
            mem: [0x0; 4096],
            vreg: [0x0; 16],
            ireg: 0x0,
            pc: 0x200,
            sp: 0x0,
            stack: [0x0; 16],
        }
    }

    pub fn pop(mut self) -> u16 {
        let val = self.stack[self.sp as usize];
        self.sp = self.sp - 1;

        self.check_sp_bounds();

        return val;
    }

    pub fn push(mut self, val: u16) {
        self.stack[self.sp] = val;
        self.sp = self.sp + 1;

        self.check_sp_bounds();
    }

    fn check_sp_bounds(self) {
        if (self.sp >= 0x0) && (self.sp <= STACK_UPPER_BOUND) {
            panic!("SP={} is out of bounds!", self.sp)
        }
    }

    pub fn execute_opcode<T: Opcode>(op: u16, proc_state: ProcState) {
        let opcode: Box<Opcode> = get_opcode(op);

        let mut proc_state = proc_state;
        opcode.exec(&mut proc_state);
    }
}

#[cfg(test)]
mod test {
    use super::ProcState;

    #[test]
    pub fn pop_correctly_performs_bounds_check() {
        let state = ProcState::reset();
    }
}