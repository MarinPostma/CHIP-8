use std::fs::File;
use std::io::Read;

use crate::cpu::CPU;
use crate::ram::RAM;

struct Chip8 {
    cpu: CPU,
    ram: RAM,
}

impl Chip8 {

    pub fn new() -> Self {
        cpu: CPU::new(),
        ram: RAM::new(),
    }

    pub fn load_rom(&self, filename: &str) {
        let mut file = File::open(filename).expect("There was an error opening the file");
        let mut data = Vec::new();
        
        file.read_to_end(data);
        self.ram.mem_cpy(data, PGM_OFFSET);
    }
}
