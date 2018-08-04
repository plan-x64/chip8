use std::env;
use std::fs::File;
use std::path::Path;

mod cart;
use cart::Cartridge;

mod cpu;
use cpu::opcodes::{Opcode, get_opcode};

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let mut f = File::open(&Path::new(filename)).expect(&format!("File not found: {}", filename));
    let cart = Cartridge::load(&mut f);

    println!("Cart Loaded. Size={} bytes", cart.size);
    println!(" {:2}  |  {:2}  | {}", "PC", "OP", "INSTRUCTION");


    let mut pc: usize = 0;
    while pc < cart.size {
        let opcode: u16 = ((cart.buffer[pc] as u16) << 8) | cart.buffer[pc+1] as u16;
        println!("{:04X} | {:04X} | {}", pc, opcode, get_opcode(opcode));
        pc = pc + 2;
    }
}