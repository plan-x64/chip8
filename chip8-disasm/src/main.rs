use std::env;
use std::fs::File;
use std::path::Path;
use std::vec::Vec;

use chip8_core::cart::Cartridge;
use chip8_core::cpu::{ProcState, MAX_MEMORY_SIZE};
use std::rc::Rc;
use std::cell::Cell;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let mut f = File::open(&Path::new(filename)).expect(&format!("File not found: {}", filename));

    let cart = Cartridge::load(&mut f);
    let mut mem = [0x0; MAX_MEMORY_SIZE];
    for i in 0..cart.size {
        mem[i + 0x200] = cart.buffer[i];
    }
    let mut state = ProcState::new(mem, Rc::new(Cell::new(Option::None)));

    println!("Cart Loaded. Size={} bytes", cart.size);
    println!(" {:2} |  {:2}  | {}", "ADDR", "OP", "INSTRUCTION");

    while (state.pc-0x200) < (cart.size as u16) {
        let current_addr = state.pc;
        let opcode = state.fetch_and_decode_opcode();
        println!(" {:#04x} | {}", current_addr, opcode);
    }
}