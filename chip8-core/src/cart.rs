use std::io::Read;

const MAX_CART_SIZE: usize = 3584;

pub struct Cartridge {
    pub buffer: [u8; MAX_CART_SIZE],
    pub size: usize
}

impl Cartridge {
    pub fn load(reader: &mut Read) -> Cartridge {
        let mut buffer = [0u8; MAX_CART_SIZE];
        match reader.read(&mut buffer) {
            Ok(size) => Cartridge { buffer, size },
            Err(_) => panic!("Failed to read!")
        }
    }
}