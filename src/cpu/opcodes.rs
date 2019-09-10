use std::fmt::{Display, Formatter, Result};

use super::ProcState;

pub enum Opcode {
    CLS,
    RET,
    JP { addr: u16 },
    CALL{ addr: u16 },
    SEVxByte{ x: u8, byte: u8 },
    SNEVxByte{ x: u8, byte: u8 },
    SEVxVy{ x: u8, y: u8 },
    LDVxByte{ x: u8, byte: u8 },
    ADDVxByte{ x: u8, byte: u8 },
    LDVxVy{ x: u8, y: u8 },
    ORVxVy{ x: u8, y: u8 },
    ANDVxVy{ x: u8, y: u8 },
    XORVxVy{ x: u8, y: u8 },
    ADDVxVy{ x: u8, y: u8 },
    SUBVxVy{ x: u8, y: u8 },
    SHRVxVy{ x: u8, y: u8 },
    SUBNVxVy{ x: u8, y: u8 },
    SHLVxVy{ x: u8, y: u8 },
    SNEVxVy{ x: u8, y: u8 },
    LDIAddr{ addr: u16 },
    JPV0Addr{ addr: u16 },
    RNDVxByte{ x: u8, byte: u8 },
    DRW{ x: u8, y: u8, nibble: u8 },
    SKPVx{ x: u8 },
    SKNPVx{ x: u8 },
    LDVxDT{ x: u8 },
    LDVxK{ x: u8 },
    LDDTVx{ x: u8 },
    LDSTVx{ x: u8 },
    ADDIVx{ x: u8 },
    LDFVx{ x: u8 },
    LDBVx{ x: u8 },
    LDIVx{ x: u8 },
    LDVxI{ x: u8 },
    UNKNOWN{ opcode: (u8, u8, u8, u8) }
}

impl Display for Opcode {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Opcode::CLS => write!(f, "CLS"),
            Opcode::RET => write!(f, "RET"),
            Opcode::JP{addr} => write!(f, "JP {:#05x}", addr),
            Opcode::CALL{addr} => write!(f, "CALL {:#05x}", addr),
            Opcode::SEVxByte{x, byte} => write!(f, "SE V{:x}, {:#04x}", x, byte),
            Opcode::SNEVxByte{x, byte} => write!(f, "SNE V{:x}, {:#02x}", x, byte),
            Opcode::SEVxVy{x, y} => write!(f, "SE V{:x}, V{:x}", x, y),
            Opcode::LDVxByte{x, byte} => write!(f, "LD V{:x}, {:#04x}", x, byte),
            Opcode::ADDVxByte{x, byte} => write!(f, "ADD V{:x}, {:#04x}", x, byte),
            Opcode::LDVxVy{x, y} => write!(f, "LD V{:x}, V{:x}", x, y),
            Opcode::ORVxVy{x, y} => write!(f, "OR V{:x}, V{:x}", x, y),
            Opcode::ADDVxVy{x, y} => write!(f, "ADD V{:x}, V{:x}", x, y),
            Opcode::XORVxVy{x, y} => write!(f, "XOR V{:x}, V{:x}", x, y),
            Opcode::ANDVxVy{x, y} => write!(f, "AND V{:x}, V{:x}", x, y),
            Opcode::SUBVxVy{x, y} => write!(f, "SUB V{:x}, V{:x}", x, y),
            Opcode::SHRVxVy{x, y} => write!(f, "SHR V{:x} {{, V{:x}}}", x, y),
            Opcode::SUBNVxVy{x, y} => write!(f, "SUBN V{:x}, V{:X}", x, y),
            Opcode::SHLVxVy{x, y} => write!(f, "SHL V{:x} {{, V{:x}}}", x, y),
            Opcode::SNEVxVy{x, y} => write!(f, "SNE V{:x}, V{:x}", x, y),
            Opcode::LDIAddr{addr} => write!(f, "LD I, {:#05x}", addr),
            Opcode::JPV0Addr{addr} => write!(f, "JP V0, {:#05x}", addr),
            Opcode::RNDVxByte{x, byte} => write!(f, "RND V{:x}, {:#04x}", x, byte),
            Opcode::DRW{x, y, nibble} => write!(f, "DRW V{:x}, V{:x}, {:#03x}", x, y, nibble),
            Opcode::SKPVx{x} => write!(f, "SKP V{:x}", x),
            Opcode::SKNPVx{x} => write!(f, "SKNP V{:x}", x),
            Opcode::LDVxDT{x} => write!(f, "LD V{:x}, DT", x),
            Opcode::LDVxK{x} => write!(f, "LD V{:x}, K", x),
            Opcode::LDDTVx{x} => write!(f, "LD DT, V{:x}", x),
            Opcode::LDSTVx{x} => write!(f, "LD ST, V{:x}", x),
            Opcode::ADDIVx{x} => write!(f, "ADD I, V{:x}", x),
            Opcode::LDFVx{x} => write!(f, "LD F, V{:x}", x),
            Opcode::LDBVx{x} => write!(f, "LD B, V{:x}", x),
            Opcode::LDVxI{x} => write!(f, "LD V{:x}, [I]", x),
            Opcode::LDIVx{x} => write!(f, "LD [I], V{:x}", x),
            Opcode::UNKNOWN{opcode} => write!(f, "UNKNOWN ({:#03x}, {:#03x}, {:#03x}, {:#03x})", opcode.0, opcode.1, opcode.2, opcode.3),
        }
    }
}

pub fn get_opcode(op: u16) -> Opcode {
    match split_opcode(op) {
        (0x0, 0x0, 0xE, 0x0) => Opcode::CLS,
        (0x0, 0x0, 0xE, 0xE) => Opcode::RET,
        (0x1, _, _, _)       => Opcode::JP{ addr: op_nnn(op) },
        (0x2, _, _, _)       => Opcode::CALL{ addr: op_nnn(op) },
        (0x3, _, _, _)       => Opcode::SEVxByte{ x: op_x(op), byte: op_kk(op) },
        (0x4, _, _, _)       => Opcode::SNEVxByte{ x: op_x(op), byte: op_kk(op) },
        (0x5, _, _, 0x0)     => Opcode::SEVxVy{ x: op_xy(op).0, y: op_xy(op).1 },
        (0x6, _, _, _)       => Opcode::LDVxByte{ x: op_x(op), byte: op_kk(op) },
        (0x7, _, _, _)       => Opcode::ADDVxByte{ x: op_x(op), byte: op_kk(op) },
        (0x8, _, _, 0x0)     => Opcode::LDVxVy{ x: op_xy(op).0, y: op_xy(op).1 },
        (0x8, _, _, 0x1)     => Opcode::ORVxVy{ x: op_xy(op).0, y: op_xy(op).1 },
        (0x8, _, _, 0x2)     => Opcode::ANDVxVy{ x: op_xy(op).0, y: op_xy(op).1 },
        (0x8, _, _, 0x3)     => Opcode::XORVxVy{ x: op_xy(op).0, y: op_xy(op).1 },
        (0x8, _, _, 0x4)     => Opcode::ADDVxVy{ x: op_xy(op).0, y: op_xy(op).1 },
        (0x8, _, _, 0x5)     => Opcode::SUBVxVy{ x: op_xy(op).0, y: op_xy(op).1 },
        (0x8, _, _, 0x6)     => Opcode::SHRVxVy{ x: op_xy(op).0, y: op_xy(op).1 },
        (0x8, _, _, 0x7)     => Opcode::SUBNVxVy{ x: op_xy(op).0, y: op_xy(op).1 },
        (0x8, _, _, 0xE)     => Opcode::SHLVxVy{ x: op_xy(op).0, y: op_xy(op).1 },
        (0x9, _, _, 0x0)     => Opcode::SNEVxVy{ x: op_xy(op).0, y: op_xy(op).1 },
        (0xA, _, _, _)       => Opcode::LDIAddr{ addr: op_nnn(op) },
        (0xB, _, _, _)       => Opcode::JPV0Addr{ addr: op_nnn(op) },
        (0xC, _, _, _)       => Opcode::RNDVxByte{ x: op_x(op), byte: op_kk(op) },
        (0xD, _, _, _)       => Opcode::DRW{ x: op_xy(op).0, y: op_xy(op).1, nibble: op_n(op) },
        (0xE, _, 0x9, 0xE)   => Opcode::SKPVx{ x: op_x(op) },
        (0xE, _, 0xA, 0x1)   => Opcode::SKNPVx{ x: op_x(op) },
        (0xF, _, 0x0, 0x7)   => Opcode::LDVxDT{ x: op_x(op) },
        (0xF, _, 0x0, 0xA)   => Opcode::LDVxK{ x: op_x(op) },
        (0xF, _, 0x1, 0x5)   => Opcode::LDDTVx{ x: op_x(op) },
        (0xF, _, 0x1, 0x8)   => Opcode::LDSTVx{ x: op_x(op) },
        (0xF, _, 0x1, 0xE)   => Opcode::ADDIVx{ x: op_x(op) },
        (0xF, _, 0x2, 0x9)   => Opcode::LDFVx{ x: op_x(op) },
        (0xF, _, 0x3, 0x3)   => Opcode::LDBVx{ x: op_x(op) },
        (0xF, _, 0x5, 0x5)   => Opcode::LDIVx{ x: op_x(op) },
        (0xF, _, 0x6, 0x5)   => Opcode::LDVxI{ x: op_x(op) },
        _                    => Opcode::UNKNOWN{ opcode: split_opcode(op) }
    }
}

pub fn split_opcode(op: u16) -> (u8, u8, u8, u8) {
    ((op >> 12) as u8 & 0xF,
     (op >> 8) as u8 & 0xF,
     (op >> 4) as u8 & 0xF,
     (op) as u8 & 0xF)
}

pub fn op_nnn(op: u16) -> u16 {
    op & 0xFFF
}

pub fn op_n(op:u16) -> u8 {
    (op & 0xF) as u8
}

pub fn op_kk(op: u16) -> u8 {
    (op & 0xFF) as u8
}

pub fn op_xy(op: u16) -> (u8, u8) {
    (((op & 0xF00) >> 8) as u8,
     ((op & 0xF0) >> 4) as u8)
}

pub fn op_x(op: u16) -> u8 {
    ((op & 0xF00) >> 8) as u8
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn split_opcode_test() {
        assert_eq!((0xB, 0xE, 0xE, 0xF), split_opcode(0xBEEF));
    }

    #[test]
    pub fn op_nnn_pulls_correct_value() {
        assert_eq!(0x5C4, op_nnn(0xF5C4));
    }

    #[test]
    pub fn op_x_pulls_correct_value() {
        assert_eq!(0x2, op_x(0x1234));
    }

    #[test]
    pub fn op_xy_pulls_correct_values() {
        assert_eq!((0xB, 0xC), op_xy(0xABCD));
    }

    #[test]
    pub fn op_kk_pulls_correct_value() {
        assert_eq!((0x89), op_kk(0x6789))
    }

    #[test]
    pub fn op_n_pulls_correct_value() {
        assert_eq!((0xF), op_n(0xCDEF));
    }
}