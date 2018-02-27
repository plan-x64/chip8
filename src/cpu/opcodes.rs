use std::fmt::{Display, Result, Formatter};
use super::ProcState;

pub trait Opcode: Display {
    fn exec(&self, proc_state: &mut ProcState);
}

pub struct CLS;
impl Opcode for CLS {
    fn exec(&self, proc_state: &mut ProcState) {
        panic!("CLS not implemented")
    }
}


impl Display for CLS {
    fn fmt(&self, f: &mut Formatter) -> Result {
        writeln!(f, "CLS")
    }
}

pub struct RET;
impl Opcode for RET {
    fn exec(&self, proc_state: &mut ProcState) {
        proc_state.pop();
    }
}


impl Display for RET {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "RET")
    }
}

pub struct JP { addr: u16 }
impl JP {
    pub fn new(nnn: u16) -> JP {
        JP { addr: nnn }
    }
}

impl Opcode for JP {
    fn exec(&self, proc_state: &mut ProcState) {
        proc_state.pc = self.addr;
    }
}

impl Display for JP {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "JP {}", self.addr)
    }
}

pub struct CALL { addr: u16 }
impl CALL {
    pub fn new(nnn: u16) -> CALL {
        CALL { addr: nnn }
    }
}

impl Opcode for CALL {
    fn exec(&self, proc_state: &mut ProcState) {
        unimplemented!()
    }
}

impl Display for CALL {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "CALL {}", self.addr)
    }
}

pub struct SEVxByte { x: u8, byte: u8 }
impl SEVxByte {
    pub fn new(x: u8, byte: u8) -> SEVxByte {
        SEVxByte { x, byte }
    }
}

impl Opcode for SEVxByte {
    fn exec(&self, proc_state: &mut ProcState) {
        unimplemented!()
    }
}

impl Display for SEVxByte {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "SE V{}, {}", self.x, self.byte)
    }
}

pub struct SNEVxByte { x: u8, byte: u8 }
impl SNEVxByte {
    pub fn new(x: u8, byte: u8) -> SNEVxByte {
        SNEVxByte { x, byte }
    }
}

impl Opcode for SNEVxByte {
    fn exec(&self, proc_state: &mut ProcState) {
        unimplemented!()
    }
}

impl Display for SNEVxByte {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "SNE V{}, {}", self.x, self.byte)
    }
}

pub struct SEVxVy { x: u8, y: u8 }
impl SEVxVy {
    pub fn new(x: u8, y: u8) -> SEVxVy {
        SEVxVy { x, y }
    }
}

impl Opcode for SEVxVy {
    fn exec(&self, proc_state: &mut ProcState) {
        unimplemented!()
    }
}

impl Display for SEVxVy {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "SE V{}, V{}", self.x, self.y)
    }
}

pub struct LDVxByte { x: u8, byte: u8 }
impl LDVxByte {
    pub fn new(x: u8, byte: u8) -> LDVxByte {
        LDVxByte { x, byte }
    }
}

impl Opcode for LDVxByte {
    fn exec(&self, proc_state: &mut ProcState) {
        unimplemented!()
    }
}

impl Display for LDVxByte {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "LD V{}, {}", self.x, self.byte)
    }
}

pub struct ADDVxByte { x: u8, byte: u8 }
impl ADDVxByte {
    pub fn new(x: u8, byte: u8) -> ADDVxByte {
        ADDVxByte { x, byte }
    }
}

impl Opcode for ADDVxByte {
    fn exec(&self, proc_state: &mut ProcState) {
        unimplemented!()
    }
}

impl Display for ADDVxByte {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "ADD V{}, {}", self.x, self.byte)
    }
}

pub struct LDVxVy { x: u8, y: u8 }
impl LDVxVy {
    pub fn new(x: u8, y: u8) -> LDVxVy {
        LDVxVy { x, y }
    }
}

impl Opcode for LDVxVy {
    fn exec(&self, proc_state: &mut ProcState) {
        unimplemented!()
    }
}

impl Display for LDVxVy {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "LD V{}, V{}", self.x, self.y)
    }
}

pub struct ORVxVy { x: u8, y: u8 }
impl ORVxVy {
    pub fn new(x: u8, y: u8) -> ORVxVy {
        ORVxVy { x, y }
    }
}

impl Opcode for ORVxVy {
    fn exec(&self, proc_state: &mut ProcState) {
        unimplemented!()
    }
}

impl Display for ORVxVy {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "OR V{}, V{}", self.x, self.y)
    }
}

pub struct ADDVxVy { x: u8, y: u8 }
impl ADDVxVy {
    pub fn new(x: u8, y: u8) -> ADDVxVy {
        ADDVxVy { x, y }
    }
}

impl Opcode for ADDVxVy {
    fn exec(&self, proc_state: &mut ProcState) {
        unimplemented!()
    }
}

impl Display for ADDVxVy {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "ADD V{}, V{}", self.x, self.y)
    }
}

pub struct XORVxVy { x: u8, y: u8 }
impl XORVxVy {
    pub fn new(x: u8, y: u8) -> XORVxVy {
        XORVxVy { x, y }
    }
}

impl Opcode for XORVxVy {
    fn exec(&self, proc_state: &mut ProcState) {
        unimplemented!()
    }
}

impl Display for XORVxVy {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "XOR V{}, V{}", self.x, self.y)
    }
}

pub struct ANDVxVy { x: u8, y: u8 }
impl ANDVxVy {
    pub fn new(x: u8, y: u8) -> ANDVxVy {
        ANDVxVy { x, y }
    }
}

impl Opcode for ANDVxVy {
    fn exec(&self, proc_state: &mut ProcState) {
        unimplemented!()
    }
}

impl Display for ANDVxVy {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, " V{}, V{}", self.x, self.y)
    }
}

pub struct SUBVxVy { x: u8, y: u8 }
impl SUBVxVy {
    pub fn new(x: u8, y: u8) -> SUBVxVy {
        SUBVxVy { x, y }
    }
}

impl Opcode for SUBVxVy {
    fn exec(&self, proc_state: &mut ProcState) {
        unimplemented!()
    }
}

impl Display for SUBVxVy {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "SUB V{}, V{}", self.x, self.y)
    }
}

pub struct SHRVxVy { x: u8, y: u8 }
impl SHRVxVy {
    pub fn new(x: u8, y: u8) -> SHRVxVy {
        SHRVxVy { x, y }
    }
}

impl Opcode for SHRVxVy {
    fn exec(&self, proc_state: &mut ProcState) {
        unimplemented!()
    }
}

impl Display for SHRVxVy {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "SHR V{} {{, V{}}}", self.x, self.y)
    }
}

pub struct SUBNVxVy { x: u8, y: u8 }
impl SUBNVxVy {
    pub fn new(x: u8, y: u8) -> SUBNVxVy {
        SUBNVxVy { x, y }
    }
}

impl Opcode for SUBNVxVy {
    fn exec(&self, proc_state: &mut ProcState) {
        unimplemented!()
    }
}

impl Display for SUBNVxVy {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "SUBN V{}, V{}", self.x, self.y)
    }
}

pub struct SHLVxVy { x: u8, y: u8 }
impl SHLVxVy {
    pub fn new(x: u8, y: u8) -> SHLVxVy {
        SHLVxVy { x, y }
    }
}

impl Opcode for SHLVxVy {
    fn exec(&self, proc_state: &mut ProcState) {
        unimplemented!()
    }
}

impl Display for SHLVxVy {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "SHL V{} {{, V{}}}", self.x, self.y)
    }
}

pub struct SNEVxVy { x: u8, y: u8 }
impl SNEVxVy {
    pub fn new(x: u8, y: u8) -> SNEVxVy {
        SNEVxVy { x, y }
    }
}

impl Opcode for SNEVxVy {
    fn exec(&self, proc_state: &mut ProcState) {
        unimplemented!()
    }
}

impl Display for SNEVxVy {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "SNE V{}, V{}", self.x, self.y)
    }
}

pub struct LDIAddr { addr: u16 }
impl LDIAddr {
    pub fn new(addr: u16) -> LDIAddr {
        LDIAddr { addr }
    }
}

impl Opcode for LDIAddr {
    fn exec(&self, proc_state: &mut ProcState) {
        unimplemented!()
    }
}

impl Display for LDIAddr {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "LD I, {}", self.addr)
    }
}

pub struct JPV0Addr { addr: u16 }
impl JPV0Addr {
    pub fn new(addr: u16) -> JPV0Addr {
        JPV0Addr { addr }
    }
}

impl Opcode for JPV0Addr {
    fn exec(&self, proc_state: &mut ProcState) {
        unimplemented!()
    }
}

impl Display for JPV0Addr {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "JP V0, {}", self.addr)
    }
}

pub struct RNDVxByte { x: u8, byte: u8 }
impl RNDVxByte {
    pub fn new(x: u8, byte: u8) -> RNDVxByte {
        RNDVxByte{ x, byte }
    }
}

impl Opcode for RNDVxByte {
    fn exec(&self, proc_state: &mut ProcState) {
        unimplemented!()
    }
}

impl Display for RNDVxByte {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "RND V{}, {}", self.x, self.byte)
    }
}

pub struct DRW { x: u8, y: u8, nibble: u8 }
impl DRW {
    pub fn new(x: u8, y: u8, nibble: u8) -> DRW {
        DRW{ x, y, nibble }
    }
}

impl Opcode for DRW {
    fn exec(&self, proc_state: &mut ProcState) {
        unimplemented!()
    }
}

impl Display for DRW {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "DRW V{}, V{}, {}", self.x, self.y, self.nibble)

    }
}

pub struct SKPVx { x: u8 }
impl SKPVx {
    pub fn new(x: u8) -> SKPVx {
        SKPVx { x }
    }
}

impl Opcode for SKPVx {
    fn exec(&self, proc_state: &mut ProcState) {
        unimplemented!()
    }
}

impl Display for SKPVx {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "SKP V{}", self.x)
    }
}

pub struct SKNPVx { x: u8 }
impl SKNPVx {
    pub fn new(x: u8) -> SKNPVx {
        SKNPVx { x }
    }
}

impl Opcode for SKNPVx {
    fn exec(&self, proc_state: &mut ProcState) {
        unimplemented!()
    }
}

impl Display for SKNPVx {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "SKNP V{}", self.x)
    }
}

pub struct LDVxDT { x: u8 }
impl LDVxDT {
    pub fn new(x: u8) -> LDVxDT {
        LDVxDT { x }
    }
}

impl Opcode for LDVxDT {
    fn exec(&self, proc_state: &mut ProcState) {
        unimplemented!()
    }
}

impl Display for LDVxDT {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "LD V{}, DT", self.x)
    }
}

pub struct LDVxK { x: u8 }
impl LDVxK {
    pub fn new(x: u8) -> LDVxK {
        LDVxK { x }
    }
}

impl Opcode for LDVxK {
    fn exec(&self, proc_state: &mut ProcState) {
        unimplemented!()
    }
}

impl Display for LDVxK {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "LD V{}, K", self.x)
    }
}

pub struct LDDTVx { x: u8 }
impl LDDTVx {
    pub fn new(x: u8) -> LDDTVx {
        LDDTVx { x }
    }
}

impl Opcode for LDDTVx {
    fn exec(&self, proc_state: &mut ProcState) {
        unimplemented!()
    }
}

impl Display for LDDTVx {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "LD DT, V{}", self.x)
    }
}

pub struct LDSTVx { x: u8 }
impl LDSTVx {
    pub fn new(x: u8) -> LDSTVx {
        LDSTVx { x }
    }
}

impl Opcode for LDSTVx {
    fn exec(&self, proc_state: &mut ProcState) {
        unimplemented!()
    }
}

impl Display for LDSTVx {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "LD ST, V{}", self.x)
    }
}

pub struct ADDIVx { x: u8 }
impl ADDIVx {
    pub fn new(x: u8) -> ADDIVx {
        ADDIVx { x }
    }
}

impl Opcode for ADDIVx {
    fn exec(&self, proc_state: &mut ProcState) {
        unimplemented!()
    }
}

impl Display for ADDIVx {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "ADD I, V{}", self.x)
    }
}

pub struct LDFVx { x: u8 }
impl LDFVx {
    pub fn new(x: u8) -> LDFVx {
        LDFVx { x }
    }
}

impl Opcode for LDFVx {
    fn exec(&self, proc_state: &mut ProcState) {
        unimplemented!()
    }
}

impl Display for LDFVx {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "LD F, V{}", self.x)
    }
}

pub struct LDBVx { x: u8 }
impl LDBVx {
    pub fn new(x: u8) -> LDBVx {
        LDBVx { x }
    }
}

impl Opcode for LDBVx {
    fn exec(&self, proc_state: &mut ProcState) {
        unimplemented!()
    }
}

impl Display for LDBVx {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "LD B, V{}", self.x)
    }
}

pub struct LDVxI { x: u8 }
impl LDVxI {
    pub fn new(x: u8) -> LDVxI {
        LDVxI { x }
    }
}

impl Opcode for LDVxI {
    fn exec(&self, proc_state: &mut ProcState) {
        unimplemented!()
    }
}

impl Display for LDVxI {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "LD V{}, [I]", self.x)
    }
}

pub struct LDIVx { x: u8 }
impl LDIVx {
    pub fn new(x: u8) -> LDIVx {
        LDIVx { x }
    }
}

impl Opcode for LDIVx {
    fn exec(&self, proc_state: &mut ProcState) {
        unimplemented!()
    }
}

impl Display for LDIVx {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "LD [I], V{}", self.x)
    }
}





pub fn get_opcode(op: u16) -> Box<Opcode> {
    match split_opcode(op) {
        (0x0, 0x0, 0xE, 0x0) =>  Box::new(CLS),
        (0x0, 0x0, 0xE, 0xE) => Box::new(RET),
        (0x1, _, _, _) => Box::new(JP::new(op_nnn(op))),
        (0x2, _, _, _) => Box::new(CALL::new(op_nnn(op))),
        (0x3, _, _, _) => Box::new(SEVxByte::new(op_x(op), op_kk(op))),
        (0x4, _, _, _) => Box::new(SNEVxByte::new(op_x(op), op_kk(op))),
        (0x5, _, _, 0x0) => Box::new(SEVxVy::new(op_xy(op).0, op_xy(op).1)),
        (0x6, _, _, _) => panic!("Not implemented for opcode={}", op),
        (0x7, _, _, _) => panic!("Not implemented for opcode={}", op),
        (0x8, _, _, 0x0) => panic!("Not implemented for opcode={}", op),
        (0x8, _, _, 0x1) => panic!("Not implemented for opcode={}", op),
        (0x8, _, _, 0x2) => panic!("Not implemented for opcode={}", op),
        (0x8, _, _, 0x3) => panic!("Not implemented for opcode={}", op),
        (0x8, _, _, 0x4) => panic!("Not implemented for opcode={}", op),
        (0x8, _, _, 0x5) => panic!("Not implemented for opcode={}", op),
        (0x8, _, _, 0x6) => panic!("Not implemented for opcode={}", op),
        (0x8, _, _, 0x7) => panic!("Not implemented for opcode={}", op),
        (0x8, _, _, 0xE) => panic!("Not implemented for opcode={}", op),
        (0x9, _, _, 0x0) => panic!("Not implemented for opcode={}", op),
        (0xA, _, _, _) => panic!("Not implemented for opcode={}", op),
        (0xB, _, _, _) => panic!("Not implemented for opcode={}", op),
        (0xC, _, _, _) => panic!("Not implemented for opcode={}", op),
        (0xD, _, _, _) => panic!("Not implemented for opcode={}", op),
        (0xE, _, 0x9, 0xE) => panic!("Not implemented for opcode={}", op),
        (0xE, _, 0xA, 0x1) => panic!("Not implemented for opcode={}", op),
        (0xF, _, 0x0, 0x7) => panic!("Not implemented for opcode={}", op),
        (0xF, _, 0x0, 0xA) => panic!("Not implemented for opcode={}", op),
        (0xF, _, 0x1, 0x5) => panic!("Not implemented for opcode={}", op),
        (0xF, _, 0x1, 0x8) => panic!("Not implemented for opcode={}", op),
        (0xF, _, 0x1, 0xE) => panic!("Not implemented for opcode={}", op),
        (0xF, _, 0x2, 0x9) => panic!("Not implemented for opcode={}", op),
        (0xF, _, 0x3, 0x3) => panic!("Not implemented for opcode={}", op),
        (0xF, _, 0x5, 0x5) => panic!("Not implemented for opcode={}", op),
        (0xF, _, 0x6, 0x5) => panic!("Not implemented for opcode={}", op),
        _ => panic!("Not implemented for opcode={}", op)
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
    (((op & 0xF00) >> 4) as u8,
     (op & 0xF0) as u8)
}

pub fn op_x(op: u16) -> u8 {
    ((op & 0xF00) >> 4) as u8
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn split_opcode_test() {
        assert_eq!((0xB, 0xE, 0xE, 0xF), split_opcode(0xBEEF));
    }

    #[test]
    pub fn op_nnn_strips_first_four_bits() {
        assert_eq!(0x5C4, op_nnn(0xF5C4));
    }
}